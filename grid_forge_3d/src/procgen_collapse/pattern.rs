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
    Direction3D, Grid3D as _, GridMap3D, GridPosition3D, GridSize3D, Tile3D, TileContainer3D as _,
};
use crate::procgen_collapse::data::CollapsibleData3D;
use crate::procgen_collapse::option::{AdjacencyTable3D, PerOptionData3D};
use crate::procgen_collapse::queue::{
    EntrophyQueue3D, PositionQueue3D, PropagateItem3D, Propagator3D,
};
use crate::procgen_collapse::{
    CollapseError3D, CollapsedGrid3D, CollapsibleGridError3D, DebugSubscriber3D, HistoryItem3D,
    HistorySubscriber3D,
};

grid_forge_core::__impl_pattern_collection! {
    struct_name: Pattern3DCollection,
    pattern_struct: Pattern3D,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_grid! {
    struct_name: Pattern3DGrid,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
    pattern: Pattern3D,
    grid: GridMap3D,
    collection: Pattern3DCollection,
    position: GridPosition3D,
    grid_size: GridSize3D,
}

grid_forge_core::__impl_pattern_frequency_hints! {
    struct_name: Pattern3DFrequencyHints,
    pattern: Pattern3D,
    pattern_grid: Pattern3DGrid,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_adjacency_rules! {
    struct_name: Pattern3DAdjacencyRules,
    pattern_grid: OverlappingPattern3DGrid,
    collection: Pattern3DCollection,
    adjacency_table: AdjacencyTable3D,
    direction: Direction3D,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_analyzer! {
    struct_name: Pattern3DAnalyzer,
    pattern_grid: Pattern3DGrid,
    collection: Pattern3DCollection,
    frequency_hints: Pattern3DFrequencyHints,
    adjacency_rules: Pattern3DAdjacencyRules,
    grid: GridMap3D,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_subscriber_trait! {
    trait_name: Pattern3DSubscriber,
    position: GridPosition3D,
}

grid_forge_core::__impl_pattern_collapsible_grid! {
    struct_name: CollapsiblePatternGrid3D,
    pattern: Pattern3D,
    collapsible_data: CollapsibleData3D,
    collection: Pattern3DCollection,
    propagate_item: PropagateItem3D,
    per_option_data: PerOptionData3D,
    adjacency_rules: Pattern3DAdjacencyRules,
    frequency_hints: Pattern3DFrequencyHints,
    collapsible_grid_error: CollapsibleGridError3D,
    collapsed_grid: CollapsedGrid3D,
    grid: GridMap3D,
    grid_size: GridSize3D,
    tile: Tile3D,
    position: GridPosition3D,
    direction: Direction3D,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_resolver! {
    struct_name: Pattern3DResolver,
    subscriber_trait: Pattern3DSubscriber,
    collapsible_grid: CollapsiblePatternGrid3D,
    collapsible_tile: CollapsibleData3D,
    propagate_item: PropagateItem3D,
    propagator: Propagator3D,
    entrophy_queue: EntrophyQueue3D,
    position_queue: PositionQueue3D,
    collapse_error: CollapseError3D,
    position: GridPosition3D,
    size_consts: [SIZE_X, SIZE_Y, SIZE_Z],
}

grid_forge_core::__impl_pattern_debug_subscriber! {
    struct_name: DebugSubscriber3D,
    trait_name: Pattern3DSubscriber,
    position: GridPosition3D,
}

grid_forge_core::__impl_pattern_history_subscriber! {
    struct_name: HistorySubscriber3D,
    history_item_name: HistoryItem3D,
    trait_name: Pattern3DSubscriber,
    position: GridPosition3D,
}

#[derive(Debug, Clone, Copy)]
pub struct Pattern3D<const SIZE_X: usize, const SIZE_Y: usize, const SIZE_Z: usize> {
    pattern_id: u64,
    tile_type_id: u64,
    tile_type_ids: [[[u64; SIZE_X]; SIZE_Y]; SIZE_Z],
}

impl<const SIZE_X: usize, const SIZE_Y: usize, const SIZE_Z: usize>
    Pattern3D<SIZE_X, SIZE_Y, SIZE_Z>
{
    /// Gets `tile_type_id` for a [`TileData`](grid_forge_core::TileData) of a tile present in the pattern,
    /// given the [`GridPosition3D`] of the primary tile (`anchor_pos`) and specific position (`pos`).
    ///
    /// # Panic
    /// This method will panic if the `pos` is located beyond boundaries of the pattern.
    pub fn get_id_for_pos(&self, anchor_pos: &GridPosition3D, pos: &GridPosition3D) -> u64 {
        self.tile_type_ids[(pos.z() - anchor_pos.z()) as usize][(pos.y() - anchor_pos.y()) as usize]
            [(pos.x() - anchor_pos.x()) as usize]
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
        anchor_pos: &GridPosition3D,
        pos: &GridPosition3D,
        tile_type_id: u64,
    ) {
        self.tile_type_ids[(pos.x() - anchor_pos.x()) as usize]
            [(pos.y() - anchor_pos.y()) as usize][(pos.z() - anchor_pos.z()) as usize] =
            tile_type_id;
    }

    pub(crate) fn empty() -> Self {
        Self {
            pattern_id: 0,
            tile_type_id: 0,
            tile_type_ids: [[[0; SIZE_X]; SIZE_Y]; SIZE_Z],
        }
    }

    /// Checks if the pattern is compatible with another pattern in given direction.
    pub fn is_compatible_with(&self, other: &Self, direction: Direction3D) -> bool {
        match direction {
            Direction3D::Up => self.compare_up(other),
            Direction3D::Down => self.compare_down(other),
            Direction3D::Left => self.compare_left(other),
            Direction3D::Right => self.compare_right(other),
            Direction3D::Higher => self.compare_higher(other),
            Direction3D::Lower => self.compare_lower(other),
        }
    }

    /// Gets the positions of all of the secondary tiles in the pattern, relative to the anchor position.
    pub fn secondary_tile_positions(anchor_pos: &GridPosition3D) -> Vec<GridPosition3D> {
        let mut out = Vec::new();
        for x_off in 0..SIZE_X {
            for y_off in 0..SIZE_Y {
                for z_off in 0..SIZE_Z {
                    if x_off == 0 && y_off == 0 && z_off == 0 {
                        continue;
                    }
                    out.push({
                        let mut pos = *anchor_pos;
                        pos += GridPosition3D::new(x_off as u32, y_off as u32, z_off as u32);
                        pos
                    })
                }
            }
        }
        out
    }

    /// Finalizes the creation of the pattern by setting its `pattern_id` and `tile_type_id`.
    pub(crate) fn finalize(&mut self) {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        self.pattern_id = hasher.finish();
        self.tile_type_id = self.tile_type_ids[0][0][0];
    }

    // --------------------------- Comparison methods ---------------------- //
    // Methods used to compare pattern compatibility in specific directions.

    fn compare_up(&self, other: &Self) -> bool {
        if SIZE_Y == 1 {
            return true;
        }
        for z in 0..SIZE_Z {
            for y in 0..SIZE_Y - 1 {
                for x in 0..SIZE_X {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z][y + 1][x] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn compare_down(&self, other: &Self) -> bool {
        if SIZE_Y == 1 {
            return true;
        }
        for z in 0..SIZE_Z {
            for y in 1..SIZE_Y {
                for x in 0..SIZE_X {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z][y - 1][x] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn compare_left(&self, other: &Self) -> bool {
        if SIZE_X == 1 {
            return true;
        }
        for z in 0..SIZE_Z {
            for y in 0..SIZE_Y {
                for x in 0..SIZE_X - 1 {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z][y][x + 1] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn compare_right(&self, other: &Self) -> bool {
        if SIZE_X == 1 {
            return true;
        }
        for z in 0..SIZE_Z {
            for y in 0..SIZE_Y {
                for x in 1..SIZE_X {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z][y][x - 1] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn compare_higher(&self, other: &Self) -> bool {
        if SIZE_Z == 1 {
            return true;
        }
        for z in 0..SIZE_Z - 1 {
            for y in 0..SIZE_Y {
                for x in 0..SIZE_X {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z + 1][y][x] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn compare_lower(&self, other: &Self) -> bool {
        if SIZE_Z == 1 {
            return true;
        }
        for z in 1..SIZE_Z {
            for y in 0..SIZE_Y {
                for x in 0..SIZE_X {
                    if self.tile_type_ids[z][y][x] != other.tile_type_ids[z - 1][y][x] {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<const SIZE_X: usize, const SIZE_Y: usize, const SIZE_Z: usize> Hash
    for Pattern3D<SIZE_X, SIZE_Y, SIZE_Z>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_type_ids.hash(state);
    }
}
