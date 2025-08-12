use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;

use rand::Rng;

use grid_forge_core::id::{IdDefault, IdentTileBuilder, TypeIdMap, TypeIdSet, TypedData};
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::error::CollapseErrorKind;
use grid_forge_core::procgen_collapse::option::OptionWeights;
use grid_forge_core::procgen_collapse::pattern::PatternTileData;
use grid_forge_core::procgen_collapse::queue::EntrophyUniform;

use crate::core::{
    Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D, TileContainer2D as _,
};
use crate::procgen_collapse::data::CollapsibleData2D;
use crate::procgen_collapse::option::{AdjacencyTable2D, PerOptionData2D};
use crate::procgen_collapse::queue::{
    EntrophyQueue2D, PositionQueue2D, PropagateItem2D, Propagator2D,
};
use crate::procgen_collapse::{
    CollapseError2D, CollapsedGrid2D, CollapsibleGridError2D, DebugSubscriber2D, HistoryItem2D,
    HistorySubscriber2D,
};

grid_forge_core::__impl_pattern_collection! {
    struct_name: Pattern2DCollection,
    pattern_struct: Pattern2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_grid! {
    struct_name: Pattern2DGrid,
    size_consts: [SIZE_X, SIZE_Y],
    pattern: Pattern2D,
    grid: GridMap2D,
    collection: Pattern2DCollection,
    position: GridPosition2D,
    grid_size: GridSize2D,
}

grid_forge_core::__impl_pattern_frequency_hints! {
    struct_name: Pattern2DFrequencyHints,
    pattern: Pattern2D,
    pattern_grid: Pattern2DGrid,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_adjacency_rules! {
    struct_name: Pattern2DAdjacencyRules,
    pattern_grid: OverlappingPattern2DGrid,
    collection: Pattern2DCollection,
    adjacency_table: AdjacencyTable2D,
    direction: Direction2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_analyzer! {
    struct_name: Pattern2DAnalyzer,
    pattern_grid: Pattern2DGrid,
    collection: Pattern2DCollection,
    frequency_hints: Pattern2DFrequencyHints,
    adjacency_rules: Pattern2DAdjacencyRules,
    grid: GridMap2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_subscriber_trait! {
    trait_name: Pattern2DSubscriber,
    position: GridPosition2D,
}

grid_forge_core::__impl_pattern_collapsible_grid! {
    struct_name: CollapsiblePatternGrid2D,
    pattern: Pattern2D,
    collapsible_data: CollapsibleData2D,
    collection: Pattern2DCollection,
    propagate_item: PropagateItem2D,
    per_option_data: PerOptionData2D,
    adjacency_rules: Pattern2DAdjacencyRules,
    frequency_hints: Pattern2DFrequencyHints,
    collapsible_grid_error: CollapsibleGridError2D,
    collapsed_grid: CollapsedGrid2D,
    grid: GridMap2D,
    grid_size: GridSize2D,
    tile: Tile2D,
    position: GridPosition2D,
    direction: Direction2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_resolver! {
    struct_name: Pattern2DResolver,
    subscriber_trait: Pattern2DSubscriber,
    collapsible_grid: CollapsiblePatternGrid2D,
    collapsible_tile: CollapsibleData2D,
    propagate_item: PropagateItem2D,
    propagator: Propagator2D,
    entrophy_queue: EntrophyQueue2D,
    position_queue: PositionQueue2D,
    collapse_error: CollapseError2D,
    position: GridPosition2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_debug_subscriber! {
    struct_name: DebugSubscriber2D,
    trait_name: Pattern2DSubscriber,
    position: GridPosition2D,
}

grid_forge_core::__impl_pattern_history_subscriber! {
    struct_name: HistorySubscriber2D,
    history_item_name: HistoryItem2D,
    trait_name: Pattern2DSubscriber,
    position: GridPosition2D,
}

#[derive(Debug, Clone, Copy)]
pub struct Pattern2D<const SIZE_X: usize, const SIZE_Y: usize> {
    pattern_id: u64,
    tile_type_id: u64,
    tile_type_ids: [[u64; SIZE_X]; SIZE_Y],
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Pattern2D<SIZE_X, SIZE_Y> {
    /// Gets `tile_type_id` for a [`TileData`](grid_forge_core::TileData) of a tile present in the pattern,
    /// given the [`GridPosition2D`] of the primary tile (`anchor_pos`) and specific position (`pos`).
    ///
    /// # Panic
    /// This method will panic if the `pos` is located beyond boundaries of the pattern.
    pub fn get_id_for_pos(&self, anchor_pos: &GridPosition2D, pos: &GridPosition2D) -> u64 {
        self.tile_type_ids[(pos.y() - anchor_pos.y()) as usize][(pos.x() - anchor_pos.x()) as usize]
    }

    /// Gets the identifier of the pattern.
    pub fn pattern_id(&self) -> u64 {
        self.pattern_id
    }

    /// Gets the identifier of the primary tile in the pattern.
    pub fn tile_type_id(&self) -> u64 {
        self.tile_type_id
    }

    pub(crate) fn set_id_for_pos(
        &mut self,
        anchor_pos: &GridPosition2D,
        pos: &GridPosition2D,
        tile_type_id: u64,
    ) {
        self.tile_type_ids[(pos.y() - anchor_pos.y()) as usize]
            [(pos.x() - anchor_pos.x()) as usize] = tile_type_id;
    }

    pub(crate) fn empty() -> Self {
        Self {
            pattern_id: 0,
            tile_type_id: 0,
            tile_type_ids: [[0; SIZE_X]; SIZE_Y],
        }
    }

    /// Checks if the pattern is compatible with another pattern in given direction.
    pub fn is_compatible_with(&self, other: &Self, direction: Direction2D) -> bool {
        match direction {
            Direction2D::Up => self.compare_up(other),
            Direction2D::Down => self.compare_down(other),
            Direction2D::Left => self.compare_left(other),
            Direction2D::Right => self.compare_right(other),
        }
    }

    /// Gets the positions of all of the secondary tiles in the pattern, relative to the anchor position.
    pub fn secondary_tile_positions(anchor_pos: &GridPosition2D) -> Vec<GridPosition2D> {
        let mut out = Vec::new();
        for x_off in 0..SIZE_X {
            for y_off in 0..SIZE_Y {
                if x_off == 0 && y_off == 0 {
                    continue;
                }
                out.push({
                    let mut pos = *anchor_pos;
                    pos += GridPosition2D::new(x_off as u32, y_off as u32);
                    pos
                })
            }
        }
        out
    }

    /// Finalizes the creation of the pattern by setting its `pattern_id` and `tile_type_id`.
    pub(crate) fn finalize(&mut self) {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        self.pattern_id = hasher.finish();
        self.tile_type_id = self.tile_type_ids[0][0];
    }

    // --------------------------- Comparison methods ---------------------- //
    // Methods used to compare pattern compatibility in specific directions.

    fn compare_up(&self, other: &Self) -> bool {
        if SIZE_Y == 1 {
            return true;
        }
        for y in 0..SIZE_Y - 1 {
            for x in 0..SIZE_X {
                if self.tile_type_ids[y][x] != other.tile_type_ids[y + 1][x] {
                    return false;
                }
            }
        }
        true
    }

    fn compare_down(&self, other: &Self) -> bool {
        if SIZE_Y == 1 {
            return true;
        }
        for y in 1..SIZE_Y {
            for x in 0..SIZE_X {
                if self.tile_type_ids[y][x] != other.tile_type_ids[y - 1][x] {
                    return false;
                }
            }
        }
        true
    }

    fn compare_left(&self, other: &Self) -> bool {
        if SIZE_X == 1 {
            return true;
        }
        for y in 0..SIZE_Y {
            for x in 0..SIZE_X - 1 {
                if self.tile_type_ids[y][x] != other.tile_type_ids[y][x + 1] {
                    return false;
                }
            }
        }
        true
    }

    fn compare_right(&self, other: &Self) -> bool {
        if SIZE_X == 1 {
            return true;
        }
        for y in 0..SIZE_Y {
            for x in 1..SIZE_X {
                if self.tile_type_ids[y][x] != other.tile_type_ids[y][x - 1] {
                    return false;
                }
            }
        }
        true
    }
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Hash for Pattern2D<SIZE_X, SIZE_Y> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_type_ids.hash(state);
    }
}
