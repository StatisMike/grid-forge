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
use grid_forge_core::procgen_collapse::queue::entrophy::EntrophyUniform;
use grid_forge_core::procgen_collapse::option::OptionWeights;

use crate::core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D, TileContainer2D as _};
use crate::procgen_collapse::option::{AdjacencyTable2D, PerOptionData2D};
use crate::procgen_collapse::queue::{EntrophyQueue2D, PositionQueue2D, PropagateItem2D, Propagator2D};
use crate::procgen_collapse::data::CollapsibleTile2D;
use crate::procgen_collapse::{CollapseError2D, CollapsedGrid2D, CollapsibleGridError2D};

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

grid_forge_core::__impl_pattern_subscriber_trait! {
    trait_name: OverlappingPattern2DSubscriber,
    position: GridPosition2D,
}

grid_forge_core::__impl_pattern_collapsible_grid! {
    struct_name: CollapsiblePatternGrid2D,
    pattern: OverlappingPattern2D,
    collapsible_data: CollapsibleTile2D,
    collection: Pattern2DCollection,
    propagate_item: PropagateItem2D,
    per_option_data: PerOptionData2D,
    adjacency_rules: OverlappingPattern2DAdjacencyRules,
    frequency_hints: OverlappingPattern2DFrequencyHints,
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
    struct_name: OverlappingPattern2DResolver,
    subscriber_trait: OverlappingPattern2DSubscriber,
    collapsible_grid: CollapsiblePatternGrid2D,
    collapsible_tile: CollapsibleTile2D,
    propagate_item: PropagateItem2D,
    propagator: Propagator2D,
    entrophy_queue: EntrophyQueue2D,
    position_queue: PositionQueue2D,
    collapse_error: CollapseError2D,
    position: GridPosition2D,
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
    use grid_forge_core::{procgen_collapse::{pattern::PatternTileData, data::CollapsedTileData}};

    use crate::{core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, Tile2D}, procgen_collapse::pattern::{OverlappingPattern2DAnalyzer, OverlappingPattern2DGrid}};


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

// pub struct CollapsiblePatternGrid2D<const SIZE_X: usize, const SIZE_Y: usize, Tile: TypedData> {
//     pub(crate) pattern_grid: GridMap2D<CollapsibleTile2D>,
//     pub(crate) patterns: Pattern2DCollection<SIZE_X, SIZE_Y>,
//     pub(crate) option_data: PerOptionData2D,
//     types: PhantomData<Tile>,
// }
// impl<const SIZE_X: usize, const SIZE_Y: usize, Tile: TypedData> Clone
//     for CollapsiblePatternGrid2D<SIZE_X, SIZE_Y, Tile>
// {
//     fn clone(&self) -> Self {
//         Self {
//             pattern_grid: self.pattern_grid.clone(),
//             patterns: self.patterns.clone(),
//             option_data: self.option_data.clone(),
//             types: self.types,
//         }
//     }
// }
// impl<const SIZE_X: usize, const SIZE_Y: usize, Tile: TypedData>
//     CollapsiblePatternGrid2D<SIZE_X, SIZE_Y, Tile>
// {
//     pub fn new_empty(
//         size: GridSize2D,
//         patterns: Pattern2DCollection<SIZE_X, SIZE_Y>,
//         frequencies: &OverlappingPattern2DFrequencyHints<SIZE_X, SIZE_Y, Tile>,
//         adjacencies: &OverlappingPattern2DAdjacencyRules<SIZE_X, SIZE_Y, Tile>,
//     ) -> Result<Self, CollapsibleGridError2D> {
//         let mut option_data = PerOptionData2D::default();
//         option_data.populate(
//             &frequencies.get_all_weights_cloned(),
//             adjacencies.inner().clone(),
//         );
//         let pattern_ids: TypeIdSet =
//             TypeIdSet::from_iter(patterns.inner().values().map(|p| p.pattern_id()));

//         let option_ids: TypeIdSet = TypeIdSet::from_iter(option_data.option_map.keys().copied());
//         let mut missing_ids = pattern_ids
//             .symmetric_difference(&option_ids)
//             .copied()
//             .collect::<Vec<_>>();
//         missing_ids.sort();
//         if !missing_ids.is_empty() {
//             return Err(CollapsibleGridError2D::new_missing(missing_ids));
//         }
//         Ok(Self {
//             pattern_grid: GridMap2D::new(size),
//             patterns,
//             option_data,
//             types: PhantomData,
//         })
//     }
//     pub fn new_from_collapsed<R: Rng>(
//         rng: &mut R,
//         collapsed: &CollapsedGrid2D,
//         patterns: Pattern2DCollection<SIZE_X, SIZE_Y>,
//         frequencies: &OverlappingPattern2DFrequencyHints<SIZE_X, SIZE_Y, Tile>,
//         adjacencies: &OverlappingPattern2DAdjacencyRules<SIZE_X, SIZE_Y, Tile>,
//     ) -> Result<Self, CollapsibleGridError2D> {
//         let mut option_data = PerOptionData2D::default();
//         option_data.populate(
//             &frequencies.get_all_weights_cloned(),
//             adjacencies.inner().clone(),
//         );
//         let pattern_ids: TypeIdSet = TypeIdSet::from_iter(patterns.iter_tile_types().copied());
//         let option_ids: TypeIdSet = TypeIdSet::from_iter(option_data.option_map.keys().copied());
//         let mut missing_ids = pattern_ids
//             .symmetric_difference(&option_ids)
//             .copied()
//             .collect::<Vec<_>>();
//         missing_ids.sort();
//         if !missing_ids.is_empty() {
//             return Err(CollapsibleGridError2D::new_missing(missing_ids));
//         }
//         let mut grid = GridMap2D::new(*collapsed.grid.size());
//         for tile in
//             Self::collapsed_into_collapsible_pattern(rng, collapsed, &patterns, &option_data)?
//         {
//             grid.insert_tile(tile);
//         }
//         Ok(Self {
//             pattern_grid: grid,
//             patterns,
//             option_data,
//             types: PhantomData,
//         })
//     }
//     pub fn retrieve_collapsed(&self) -> CollapsedGrid2D {
//         let mut out = CollapsedGrid2D::new(self.pattern_grid.size().clone());
//         for tile in self.pattern_grid.iter_tiles() {
//             if !tile.data().is_collapsed() {
//                 continue;
//             }
//             out.grid.insert_data(
//                 &tile.grid_position(),
//                 CollapsedTileData::new(
//                     self.option_data
//                         .get_tile_type_id(
//                             tile.data()
//                                 .collapsed_idx()
//                                 .expect("cannot get `collapse_idx` for uncollapsed tile"),
//                         )
//                         .expect("cannot get `tile_type_id` for uncollapsed tile"),
//                 ),
//             );
//         }
//         out
//     }
//     pub fn retrieve_ident<Builder: IdentTileBuilder<OutputTile>, OutputTile>(
//         &self,
//         builder: &Builder,
//     ) -> Result<GridMap2D<OutputTile>, CollapsibleGridError2D>
//     where
//         Builder: IdentTileBuilder<OutputTile>,
//         OutputTile: TypedData,
//     {
//         let mut out = GridMap2D::<OutputTile>::new(*self.pattern_grid.size());
//         for tile in self.pattern_grid.iter_tiles() {
//             if !tile.data().is_collapsed() {
//                 continue;
//             }
//             out.insert_data(
//                 &tile.grid_position(),
//                 builder.build_tile_unchecked(
//                     self.option_data
//                         .get_tile_type_id(
//                             tile.data()
//                                 .collapsed_idx()
//                                 .expect("cannot get `collapse_idx` for uncollapsed tile"),
//                         )
//                         .expect("cannot get `tile_type_id` for uncollapsed tile"),
//                 ),
//             );
//         }
//         Ok(out)
//     }
//     pub fn retrieve_ident_default<OutputTile>(&self) -> GridMap2D<OutputTile>
//     where
//         OutputTile: TypedData + IdDefault,
//     {
//         let mut out = GridMap2D::<OutputTile>::new(*self.pattern_grid.size());
//         for tile in self.pattern_grid.iter_tiles() {
//             if !tile.data().is_collapsed() {
//                 continue;
//             }
//             out.insert_data(
//                 &tile.grid_position(),
//                 OutputTile::tile_type_default(
//                     self.option_data
//                         .get_tile_type_id(
//                             tile.data()
//                                 .collapsed_idx()
//                                 .expect("cannot get `collapse_idx` for uncollapsed tile"),
//                         )
//                         .expect("cannot get `tile_type_id` for uncollapsed tile"),
//                 ),
//             );
//         }
//         out
//     }
//     #[doc = r" Returns all possitions in the internal grid holding either collapsed or uncollapsed tiles."]
//     pub fn retrieve_positions(&self, collapsed: bool) -> Vec<GridPosition2D> {
//         let func: fn(&CollapsibleTile2D) -> bool = if collapsed {
//             |d| d.is_collapsed()
//         } else {
//             |d| !d.is_collapsed()
//         };
//         self.pattern_grid
//             .indexed_iter()
//             .filter_map(|t| {
//                 if let Some(d) = t.1 {
//                     if func(d) {
//                         return Some(t.0);
//                     }
//                 }
//                 None
//             })
//             .collect()
//     }
//     pub fn remove_uncollapsed(&mut self) {
//         for t in self.pattern_grid.iter_mut() {
//             if let Some(d) = t {
//                 if d.is_collapsed() {
//                     continue;
//                 }
//                 t.take();
//             }
//         }
//     }
//     pub(crate) fn get_initial_propagate_items(
//         &self,
//         to_collapse: &[GridPosition2D],
//     ) -> Vec<PropagateItem2D> {
//         let mut out = Vec::new();
//         let mut cache = TypeIdMap::default();
//         let mut check_generated = HashSet::<GridPosition2D>::default();
//         let check_provided = HashSet::<GridPosition2D>::from_iter(to_collapse.iter().copied());
//         for pos_to_collapse in to_collapse {
//             for neighbour_tile in self
//                 .pattern_grid
//                 .get_neighbours(pos_to_collapse)
//                 .inner()
//                 .iter()
//                 .flatten()
//             {
//                 if !neighbour_tile.as_ref().is_collapsed()
//                     || check_provided.contains(&neighbour_tile.grid_position())
//                     || check_generated.contains(&neighbour_tile.grid_position())
//                 {
//                     continue;
//                 }
//                 check_generated.insert(neighbour_tile.grid_position());
//                 let collapsed_idx = neighbour_tile.as_ref().collapsed_idx().unwrap();
//                 for opt_to_remove in cache.entry(collapsed_idx as u64).or_insert_with(|| {
//                     (0..self.option_data.option_count)
//                         .filter(|option_idx| option_idx != &collapsed_idx)
//                         .collect::<Vec<usize>>()
//                 }) {
//                     out.push(PropagateItem2D::new(
//                         neighbour_tile.grid_position(),
//                         *opt_to_remove,
//                     ))
//                 }
//             }
//         }
//         out
//     }
//     #[doc = r" Removes options from tile neighbours after its collapse."]
//     pub(crate) fn purge_options_for_neighbours(
//         grid: &mut GridMap2D<CollapsibleTile2D>,
//         collapsed_option: usize,
//         collapsed_position: &GridPosition2D,
//         option_data: &PerOptionData2D,
//     ) {
//         for direction in Direction2D::ALL {
//             if let Some(mut tile) = grid.get_mut_neighbour_at(collapsed_position, &direction) {
//                 if tile.as_ref().is_collapsed() {
//                     continue;
//                 }
//                 let enabled = option_data.get_all_enabled_in_direction(collapsed_option, direction);
//                 for possible_option in tile
//                     .as_ref()
//                     .ways_to_be_option()
//                     .iter_possible()
//                     .collect::<Vec<_>>()
//                 {
//                     if !enabled.contains(&possible_option)
//                         && tile
//                             .data()
//                             .mut_ways_to_be_option()
//                             .purge_option(possible_option)
//                     {
//                         let weights = option_data.get_weights(possible_option);
//                         tile.data().remove_option(weights);
//                     }
//                 }
//             }
//         }
//     }
//     #[doc = r" Removes options from tile based of possible options for its neighbours."]
//     pub(crate) fn purge_incompatible_options(
//         grid: &mut GridMap2D<CollapsibleTile2D>,
//         position: &GridPosition2D,
//         option_data: &PerOptionData2D,
//     ) -> bool {
//         let num_options = option_data.option_count;
//         let mut possible_options = Vec::with_capacity(num_options);
//         possible_options.resize(num_options, true);
//         for direction in Direction2D::ALL {
//             if let Some(tile) = grid.get_neighbour_at(position, &direction) {
//                 if let Some(collapsed_idx) = tile.as_ref().collapsed_idx() {
//                     let enabled = option_data
//                         .get_all_enabled_in_direction(collapsed_idx, direction.opposite());
//                     for (option_idx, state) in possible_options.iter_mut().enumerate() {
//                         if *state && !enabled.contains(&option_idx) {
//                             *state = false;
//                         }
//                     }
//                 } else if tile.as_ref().num_possible_options() < option_data.possible_options_count
//                 {
//                     let mut possible_in_any: HashSet<usize> = HashSet::new();
//                     for neigbour_idx in tile.as_ref().ways_to_be_option().iter_possible() {
//                         possible_in_any.extend(
//                             option_data
//                                 .get_all_enabled_in_direction(neigbour_idx, direction.opposite())
//                                 .iter(),
//                         );
//                     }
//                     for (option_idx, state) in possible_options.iter_mut().enumerate() {
//                         if *state && !possible_in_any.contains(&option_idx) {
//                             *state = false;
//                         }
//                     }
//                 }
//             }
//         }
//         if !possible_options.iter().any(|state| *state) {
//             return false;
//         }
//         let tile = grid.get_mut_data_at_position(position).unwrap();
//         for (possible, (option_idx, weights)) in
//             possible_options.iter().zip(option_data.iter_weights())
//         {
//             if !possible && tile.mut_ways_to_be_option().purge_option(option_idx) {
//                 tile.remove_option(*weights);
//             }
//         }
//         true
//     }
//     fn collapsed_into_collapsible_pattern<R: Rng>(
//         rng: &mut R,
//         collapsed: &CollapsedGrid2D,
//         patterns: &Pattern2DCollection<SIZE_X, SIZE_Y>,
//         options: &PerOptionData2D,
//     ) -> Result<Vec<Tile2D<CollapsibleTile2D>>, CollapsibleGridError2D> {
//         let entrophy_uniform = EntrophyUniform::new();
//         let ways = &options.ways_to_be_option;
//         let mut out = Vec::new();
//         for position in collapsed.grid.get_all_positions() {
//             let mut possible_patterns = Vec::new();
//             let tile_type_id = collapsed
//                 .grid
//                 .get_tile_at_position(&position)
//                 .unwrap()
//                 .as_ref()
//                 .tile_type_id();
//             'pat_loop: for pattern in patterns.get_patterns_for_tile(tile_type_id) {
//                 for pos_to_check in OverlappingPattern2D::<SIZE_X, SIZE_Y>::secondary_tile_positions(&position) {
//                     if let Some(tile_type_id) = collapsed
//                         .grid
//                         .get_tile_at_position(&pos_to_check)
//                         .map(|t| t.as_ref().tile_type_id())
//                     {
//                         if tile_type_id != pattern.get_id_for_pos(&position, &pos_to_check) {
//                             continue 'pat_loop;
//                         }
//                     }
//                 }
//                 possible_patterns.push(
//                     options
//                         .get_tile_offset(pattern.pattern_id())
//                         .expect("cannot get pattern idx"),
//                 );
//             }
//             if possible_patterns.is_empty() {
//                 return Err(CollapsibleGridError2D::new_collapse(position));
//             }
//             let mut current_ways = ways.clone();
//             current_ways.purge_others(&possible_patterns);
//             let num_options = possible_patterns.len();
//             let mut weights = OptionWeights::default();
//             for pattern in possible_patterns {
//                 let w = options.get_weights(pattern);
//                 weights.0 += w.0;
//                 weights.1 += w.1;
//             }
//             out.push(Tile2D::new(
//                 position,
//                 CollapsibleTile2D::new_uncollapsed_tile(
//                     num_options,
//                     current_ways,
//                     weights,
//                     entrophy_uniform.sample(rng),
//                 ),
//             ));
//         }
//         Ok(out)
//     }
//     fn retrieve_tile_type_id(&self, tile: &impl AsRef<CollapsibleTile2D>) -> Option<u64> {
//         match tile.as_ref().collapsed_idx() {
//             Some(pattern_idx) => {
//                 let pattern_id = self.option_data.get_tile_type_id(pattern_idx).unwrap();
//                 let pattern = self.patterns.get_pattern(pattern_id).unwrap();
//                 Some(pattern.tile_type_id())
//             }
//             None => None,
//         }
//     }
// }