use std::hash::{DefaultHasher, Hash, Hasher};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::any::Any;

use grid_forge_core::procgen_collapse::error::CollapseErrorKind;
use rand::Rng;

use grid_forge_core::procgen_collapse::pattern::{PatternTileData};
use grid_forge_core::id::{TypeIdMap, TypeIdSet, TypedData};
use grid_forge_core::id::IdentTileBuilder;
use grid_forge_core::id::IdDefault;
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::queue::EntrophyUniform;
use grid_forge_core::procgen_collapse::option::OptionWeights;

use crate::core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D, TileContainer2D as _};
use crate::procgen_collapse::option::{AdjacencyTable2D, PerOptionData2D};
use crate::procgen_collapse::queue::{EntrophyQueue2D, PositionQueue2D, PropagateItem2D, Propagator2D};
use crate::procgen_collapse::data::CollapsibleData2D;
use crate::procgen_collapse::{CollapseError2D, HistoryItem2D, HistorySubscriber2D, CollapsedGrid2D, CollapsibleGridError2D, DebugSubscriber2D};

grid_forge_core::__impl_pattern_collection! {
    struct_name: Pattern2DCollection,
    pattern_struct: Pattern2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_grid! {
    struct_name: CollapsiblePattern2DGrid,
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
    pattern_grid: CollapsiblePattern2DGrid,
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
    pattern_grid: CollapsiblePattern2DGrid,
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

impl <const SIZE_X: usize, const SIZE_Y: usize> Pattern2D<SIZE_X, SIZE_Y> { 
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

    pub (crate) fn set_id_for_pos(
        &mut self,
        anchor_pos: &GridPosition2D,
        pos: &GridPosition2D,
        tile_type_id: u64,
    ) {
        self.tile_type_ids[(pos.y() - anchor_pos.y()) as usize]
            [(pos.x() - anchor_pos.x()) as usize] = tile_type_id;
    }

    pub (crate) fn empty() -> Self {
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
    pub (crate) fn finalize(&mut self) {
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

impl <const SIZE_X: usize, const SIZE_Y: usize> Hash for Pattern2D<SIZE_X, SIZE_Y> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_type_ids.hash(state);
    }
}

#[cfg(test)]
mod test {
    use grid_forge_core::{procgen_collapse::{pattern::PatternTileData, data::CollapsedTileData}};

    use crate::{core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D}, procgen_collapse::pattern::{Pattern2DAnalyzer, CollapsiblePattern2DGrid}};


    /// ```text
    ///     0 1 2 3
    ///     -------
    /// 0 | 0 0 1 1
    /// 1 | 0 0 1 1
    /// 2 | 1 0 1 0
    /// 3 | 1 0 0 1
    /// ```
    fn test_grid_2d_2x2() -> GridMap2D<CollapsedTileData> {
        let mut map = GridMap2D::new(GridSize2D::new(4, 4));
        for tile in vec![
            Tile2D::new(GridPosition2D::new(0, 0), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(0, 1), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(0, 2), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(0, 3), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(1, 0), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 1), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 2), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 3), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 0), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(2, 1), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(2, 2), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(2, 3), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(3, 0), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(3, 1), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(3, 2), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 3), CollapsedTileData::new(1)),
        ] {
            map.insert(tile);
        }
        map
    }

    /// ```text
    ///     0 1 2 3 4 5
    ///     -----------
    /// 0 | 0 0 0 1 1 1
    /// 1 | 0 0 0 1 1 1
    /// 2 | 0 0 0 1 1 1
    /// 3 | 1 0 0 1 0 0
    /// 4 | 1 0 0 0 1 0
    /// 5 | 1 0 0 0 0 1
    /// ```
    fn test_grid_2d_3x3() -> GridMap2D<CollapsedTileData> {
        let mut map = GridMap2D::new(GridSize2D::new(6, 6));
        for tile in vec![
            Tile2D::new(GridPosition2D::new(0, 0), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 0), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 0), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 0), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(4, 0), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(5, 0), CollapsedTileData::new(1)),
            
            Tile2D::new(GridPosition2D::new(0, 1), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 1), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 1), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 1), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(4, 1), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(5, 1), CollapsedTileData::new(1)),
            
            Tile2D::new(GridPosition2D::new(0, 2), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(1, 2), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 2), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 2), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(4, 2), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(5, 2), CollapsedTileData::new(1)),

            Tile2D::new(GridPosition2D::new(0, 3), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(1, 3), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 3), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 3), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(4, 3), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(5, 3), CollapsedTileData::new(0)),
            
            Tile2D::new(GridPosition2D::new(0, 4), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(1, 4), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 4), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 4), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(4, 4), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(5, 4), CollapsedTileData::new(0)),

            Tile2D::new(GridPosition2D::new(0, 5), CollapsedTileData::new(1)),
            Tile2D::new(GridPosition2D::new(1, 5), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(2, 5), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(3, 5), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(4, 5), CollapsedTileData::new(0)),
            Tile2D::new(GridPosition2D::new(5, 5), CollapsedTileData::new(1)),
        ] {
            map.insert(tile);
        }
        map
    }

    fn retrieve_pattern<const SIZE_X: usize, const SIZE_Y: usize>(
        position: &GridPosition2D,
        map: &CollapsiblePattern2DGrid<SIZE_X, SIZE_Y>,
    ) -> (u64, u64) {
        let Some(data) = map.inner.data_at(position) else {
            panic!("Can't get tile at {position:?}");
        };
        let PatternTileData::WithPattern {
            tile_type_id,
            pattern_id,
        } = data
        else {
            panic!("Can't get WithPattern tile data at {position:?}");
        };
        (*tile_type_id, *pattern_id)
    }

    #[test]
    fn correct_adjacency_2d_2x2() {
        let mut analyzer = Pattern2DAnalyzer::<2, 2, CollapsedTileData>::default();
        let pattern_grid = analyzer.analyze(&test_grid_2d_2x2());

        let adjacency_rules = analyzer.get_adjacency();

        let p0000 = retrieve_pattern(&GridPosition2D::new(0, 0), &pattern_grid);
        let p0101 = retrieve_pattern(&GridPosition2D::new(1, 0), &pattern_grid);
        let p1111 = retrieve_pattern(&GridPosition2D::new(2, 0), &pattern_grid);

        for dir in Direction2D::ALL {
            assert!(
                !adjacency_rules.is_valid_at_dir(p0000.1, dir, p1111.1),
                "patterns are falsely compatible"
            )
        }

        assert!(adjacency_rules.is_valid_at_dir(p0000.1, Direction2D::Right, p0101.1));
        assert!(adjacency_rules.is_valid_at_dir(p0101.1, Direction2D::Left, p0000.1));
        assert!(!adjacency_rules.is_valid_at_dir(p0000.1, Direction2D::Up, p0101.1));
        assert!(!adjacency_rules.is_valid_at_dir(p0000.1, Direction2D::Down, p0101.1));
    }

    #[test]
    fn correct_adjacency_2d_3x3() {
        let mut analyzer = Pattern2DAnalyzer::<3, 3, CollapsedTileData>::default();
        let pattern_grid = analyzer.analyze(&test_grid_2d_3x3());
        let adjacency_rules = analyzer.get_adjacency();

        // Test some specific pattern combinations
        // let p000_000_000 = retrieve_pattern(&GridPosition2D::new(0, 0), &pattern_grid);
        let p111_111_111 = retrieve_pattern(&GridPosition2D::new(3, 0), &pattern_grid);
        // let p111_111_100 = retrieve_pattern(&GridPosition2D::new(2, 3), &pattern_grid);
        // let p111_100_010 = retrieve_pattern(&GridPosition2D::new(3, 3), &pattern_grid);
        // let p100_100_100 = retrieve_pattern(&GridPosition2D::new(0, 3), &pattern_grid);

        // Test some expected compatible patterns
        assert!(adjacency_rules.is_valid_at_dir(
            p111_111_111.1, 
            Direction2D::Down, 
            p111_111_111.1
        ));
        
        // assert!(adjacency_rules.is_valid_at_dir(
        //     p000111.1,
        //     Direction2D::Down,
        //     retrieve_pattern(&GridPosition2D::new(1, 2), &pattern_grid).1
        // ));

        // // Test some expected incompatible patterns
        // for dir in Direction2D::ALL {
        //     assert!(
        //         !adjacency_rules.is_valid_at_dir(p000000.1, dir, p101010.1),
        //         "Patterns should be incompatible in all directions"
        //     );
            
        //     assert!(
        //         !adjacency_rules.is_valid_at_dir(p000111.1, dir, p111000.1),
        //         "Patterns should be incompatible in all directions"
        //     );
        // }

        // // Test corner cases
        // assert!(adjacency_rules.is_valid_at_dir(
        //     retrieve_pattern(&GridPosition2D::new(1, 1), &pattern_grid).1,
        //     Direction2D::Right,
        //     retrieve_pattern(&GridPosition2D::new(1, 2), &pattern_grid).1
        // ));
    }
}