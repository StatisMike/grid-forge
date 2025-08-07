use std::hash::{DefaultHasher, Hash, Hasher};
use std::collections::BTreeMap;
use std::marker::PhantomData;

use grid_forge_core::procgen_collapse::overlap::{PatternTileData};
use grid_forge_core::id::{TypeIdMap, TypeIdSet, TypedData};

use crate::core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, TileContainer2D as _};
use crate::procgen_collapse::option::AdjacencyTable2D;

grid_forge_core::__impl_pattern_collection! {
    struct_name: Pattern2DCollection,
    pattern_struct: OverlappingPattern2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_grid! {
    struct_name: OverlappingPattern2DGrid,
    size_consts: [SIZE_X, SIZE_Y],
    pattern: OverlappingPattern2D,
    grid: GridMap2D,
    collection: Pattern2DCollection,
    position: GridPosition2D,
    grid_size: GridSize2D,
}

grid_forge_core::__impl_pattern_frequency_hints! {
    struct_name: OverlappingPattern2DFrequencyHints,
    pattern: OverlappingPattern2D,
    pattern_grid: OverlappingPattern2DGrid,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_adjacency_rules! {
    struct_name: OverlappingPattern2DAdjacencyRules,
    pattern: OverlappingPattern2D,
    pattern_grid: OverlappingPattern2DGrid,
    collection: Pattern2DCollection,
    adjacency_table: AdjacencyTable2D,
    direction: Direction2D,
    size_consts: [SIZE_X, SIZE_Y],
}

grid_forge_core::__impl_pattern_analyzer! {
    struct_name: OverlappingPattern2DAnalyzer,
    pattern: OverlappingPattern2D,
    pattern_grid: OverlappingPattern2DGrid,
    collection: Pattern2DCollection,
    frequency_hints: OverlappingPattern2DFrequencyHints,
    adjacency_rules: OverlappingPattern2DAdjacencyRules,
    grid: GridMap2D,
    size_consts: [SIZE_X, SIZE_Y],
}

#[derive(Debug, Clone, Copy)]
pub struct OverlappingPattern2D<const SIZE_X: usize, const SIZE_Y: usize> { 
    pattern_id: u64,
    tile_type_id: u64,
    tile_type_ids: [[u64; SIZE_X]; SIZE_Y], 
}

impl <const SIZE_X: usize, const SIZE_Y: usize> OverlappingPattern2D<SIZE_X, SIZE_Y> { 
    /// Gets `tile_type_id` for a [`TileData`] of a tile present in the pattern, given the [`GridPosition2D`] of the
    /// primary tile (`anchor_pos`) and specific position (`pos`).
    ///
    /// # Panic
    /// This method will panic if the `pos` is located beyond boundaries of the pattern.
    pub fn get_id_for_pos(&self, anchor_pos: &GridPosition2D, pos: &GridPosition2D) -> u64 {
        self.tile_type_ids[(pos.y() - anchor_pos.y()) as usize][(pos.x() - anchor_pos.x()) as usize]
    }

    pub fn pattern_id(&self) -> u64 {
        self.pattern_id
    }

    pub fn tile_type_id(&self) -> u64 {
        self.tile_type_id
    }

    fn set_id_for_pos(
        &mut self,
        anchor_pos: &GridPosition2D,
        pos: &GridPosition2D,
        tile_type_id: u64,
    ) {
        self.tile_type_ids[(pos.y() - anchor_pos.y()) as usize]
            [(pos.x() - anchor_pos.x()) as usize] = tile_type_id;
    }

    fn empty() -> Self {
        Self {
            pattern_id: 0,
            tile_type_id: 0,
            tile_type_ids: [[0; SIZE_X]; SIZE_Y],
        }
    }

    pub fn is_compatible_with(&self, other: &Self, direction: Direction2D) -> bool {
        match direction {
            Direction2D::Up => self.compare_up(other),
            Direction2D::Down => self.compare_down(other),
            Direction2D::Left => self.compare_left(other),
            Direction2D::Right => self.compare_right(other),
        }
    }

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

    fn finalize(&mut self) {
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

impl <const SIZE_X: usize, const SIZE_Y: usize> Hash for OverlappingPattern2D<SIZE_X, SIZE_Y> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_type_ids.hash(state);
    }
}

#[cfg(test)]
mod test {
    use grid_forge_core::{procgen_collapse::{overlap::PatternTileData, tile::CollapsedTileData}};

    use crate::{core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D}, procgen_collapse::overlap::{OverlappingPattern2DAnalyzer, OverlappingPattern2DGrid}};


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
            map.insert_tile(tile);
        }
        map
    }

    fn retrieve_pattern<const SIZE_X: usize, const SIZE_Y: usize>(
        position: &GridPosition2D,
        map: &OverlappingPattern2DGrid<SIZE_X, SIZE_Y>,
    ) -> (u64, u64) {
        let Some(data) = map.inner.get_data_at_position(position) else {
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
        let mut analyzer = OverlappingPattern2DAnalyzer::<2, 2, CollapsedTileData>::default();
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
}
