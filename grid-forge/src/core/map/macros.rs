macro_rules! __impl_grid {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $struct_name:ident;

        module: $module:ident,
        direction: $direction_type:ty,
        direction_table: $direction_table:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        tile: $tile_type:ident,
        tile_ref: $tile_ref_type:ident,
        tile_mut: $tile_mut_type:ident,
        neighbours_count: $neighbour_size:literal,
    ) => {

        macro_rules! __tile_type { () => { $tile_type<Data> }; }
        macro_rules! __tile_ref_type { () => { $tile_ref_type<'a, Data> }; }
        macro_rules! __tile_mut_type { () => { $tile_mut_type<'a, Data> }; }

        $(#[$struct_meta])*
        #[derive(Debug)]
        $vis struct $struct_name<Data: crate::TileData> {
            size: $size_type,
            tiles: Vec<Option<Data>>,
            mut_accessed: Option<std::collections::HashSet<$position_type>>,
        }

        impl<Data: crate::TileData> $struct_name<Data> {

            /// Creates a new empty grid with the specified dimensions.
            ///
            /// All tile slots are initialized as empty. Use insertion methods to populate the grid.
            pub fn new(size: $size_type) -> Self {
                let count = size.max_tile_count();
                let mut tiles = Vec::new();
                for _ in 0..count {
                    tiles.push(None);
                }
                tiles.shrink_to_fit();
                Self { size, tiles, mut_accessed: None }
            }

            #[inline]
            pub fn size(&self) -> &$size_type {
                &self.size
            }

            /// Gets a direct reference to data at a position if present and valid.
            ///
            /// # Returns
            /// - `Some(&Data)` if position is valid and contains data
            /// - `None` otherwise
            ///
            /// For a version that returns position and data together, see [`get_tile_at_position()`](Self::get_tile_at_position).
            pub fn get_data_at_position(&self, position: &$position_type) -> Option<&Data> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                unsafe { self.tiles.get_unchecked(self.size.offset(&position)).as_ref() }
            }

            /// Gets an immutable tile reference containing both position and data.
            ///
            /// # Returns
            #[doc = "- [`Some(TileRef)`]"]
            #[doc = concat!("(", stringify!($tile_ref_type), ")")]
            #[doc = " if position is valid and contains data"]
            /// - `None` otherwise
            ///
            /// For direct data access without position, see [`get_data_at_position`](Self::get_data_at_position).
            pub fn get_tile_at_position<'a>(&'a self, position: &$position_type) -> Option<__tile_ref_type!()> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                unsafe {
                    self.tiles.get_unchecked(self.size.offset(&position))
                        .as_ref()
                        .map(|data| (*position, data).into())
                }
            }

            pub fn get_mut_data_at_position(&mut self, position: &$position_type) -> Option<&mut Data> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe { self.tiles.get_unchecked_mut(self.size.offset(position)).as_mut() }
            }

            pub fn get_mut_tile_at_position<'a>(&'a mut self, position: &$position_type) -> Option<__tile_mut_type!()> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe {
                    self
                        .tiles
                        .get_unchecked_mut(self.size.offset(&position))
                        .as_mut()
                        .map(|data| (*position, data).into())
                }
            }

            pub fn get_tiles_at_positions<'a>(&'a self, positions: &[$position_type]) -> Vec<__tile_ref_type!()> {
                positions
                    .iter()
                    .filter_map(|position| self.get_tile_at_position(position))
                    .collect::<Vec<_>>()
            }

            pub fn insert_tile(&mut self, tile: __tile_type!()) -> bool
            {
                if !self.size.is_position_valid(&tile.grid_position()) {
                    return false;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(tile.grid_position());
                }
                unsafe {
                    self.tiles.get_unchecked_mut(self.size.offset(&tile.grid_position())).replace(tile.into_data());
                }
                true
            }

            pub fn insert_data(&mut self, position: &$position_type, data: Data) -> bool {
                if !self.size.is_position_valid(position) {
                    return false;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe {
                    self.tiles.get_unchecked_mut(self.size.offset(&position)).replace(data);
                }
                true
            }

            pub fn remove_tile_at_position(&mut self, position: &$position_type) -> Option<__tile_type!()>
            {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                let offset = self.size.offset(position);

                unsafe {
                    self.tiles.get_unchecked_mut(offset)
                        .take()
                        .and_then(|t| Some((*position, t).into()))
                }
            }

            pub fn get_neighbours<'a>(&'a self, position: &$position_type) -> $direction_table<Option<__tile_ref_type!()>> {
                let mut result = $direction_table::new([const { None }; $neighbour_size]);
                for direction in <$direction_type>::ALL {
                    if let Some(pos) = direction.march_step(position, &self.size) {
                        if let Some(tile) = self.get_tile_at_position(&pos) {
                            result[direction] = Some(tile);
                        }
                    }
                }
                result
            }

            pub fn get_neighbour_at<'a>(
                &'a self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_ref_type!()> {
                if let Some(position) = direction.march_step(position, &self.size) {
                    return self.get_tile_at_position(&position);
                }
                None
            }

            pub fn get_mut_neighbour_at<'a>(
                &'a mut self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_mut_type!()> {
                if let Some(position) = direction.march_step(position, &self.size) {
                    return self.get_mut_tile_at_position(&position);
                }
                None
            }

            pub fn get_all_positions(&self) -> Vec<$position_type> {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
                    .collect()
            }

            pub fn iter_all_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + use<'a, Data>
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
            }

            pub fn get_all_empty_positions(&self) -> Vec<$position_type> {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
                    .collect()
            }

            pub fn iter_all_empty_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + use<'a, Data>
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
            }

            pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>>
            where
                Data: 'a,
            {
                self.tiles.iter_mut()
            }

            pub fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = __tile_ref_type!()>
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, data)| data.as_ref().map(|d| (pos, d).into()))
            }

            pub fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = __tile_mut_type!()>
            where
                Data: 'a,
            {
                self.indexed_iter_mut()
                    .filter_map(|(pos, data)| data.as_mut().map(|d| (pos, d).into()))
            }

            pub fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = ($position_type, &'a Option<Data>)>
            where
                Data: 'a,
            {
                self.tiles
                    .iter()
                    .enumerate()
                    .map(move |(idx, t)| (self.size().pos_from_offset(idx), t))
            }

            pub fn indexed_iter_mut<'a>(
                &'a mut self,
            ) -> impl Iterator<Item = ($position_type, &'a mut Option<Data>)>
            where
                Data: 'a,
            {
                let size = self.size().clone();
                self.tiles
                    .iter_mut()
                    .enumerate()
                    .map(move |(idx, t)| (size.pos_from_offset(idx), t))
            }

            pub fn drain_remapped(mut self, anchor_pos: $position_type) -> Vec<__tile_type!()> {
                self.indexed_iter_mut()
                    .filter_map(|(pos, t)| {
                        if t.is_none() {
                            None
                        } else {
                            Some((anchor_pos + pos, t.take().unwrap()).into())
                        }
                    })
                    .collect()
            }

            pub fn drain(mut self) -> Vec<__tile_type!()> {
                self.indexed_iter_mut()
                    .filter_map(|(pos, t)| {
                        if t.is_none() {
                            None
                        } else {
                            Some((pos, t.take().unwrap()).into())
                        }
                    })
                    .collect()
            }

            pub fn fill_empty_using(&mut self, func: fn($position_type) -> Data) {
                for (pos, t) in self.indexed_iter_mut() {
                    if t.is_none() {
                        t.replace(func(pos));
                    }
                }
            }

            pub fn fill_empty_with_default(&mut self)
            where
                Data: Default,
            {
                let empty_positions = self.get_all_empty_positions();
                for pos in empty_positions {
                    self.insert_data(&pos, Data::default());
                }
            }

            pub fn fill_empty_with(&mut self, data: Data)
            where
                Data: Clone,
            {
                for pos in self.get_all_empty_positions() {
                    self.insert_data(&pos, data.clone());
                }
            }

            pub fn get_remapped(&self, anchor_pos: $position_type) -> Vec<__tile_type!()>
            where
                Data: Clone,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| {
                        if t.is_some() {
                            Some((anchor_pos + pos, t.clone().unwrap()).into())
                        } else {
                            None
                        }
                    })
                    .collect()
            }

            pub fn mut_access_tracking(&mut self, enabled: bool) {
                self.mut_accessed = if enabled {
                    Some(std::collections::HashSet::new())
                } else {
                    None
                };
            }

            pub fn get_mut_accessed(&self) -> Vec<$position_type> {
                if let Some(mut_accessed) = &self.mut_accessed {
                    let mut mut_accessed = mut_accessed.iter().copied().collect::<Vec<_>>();
                    mut_accessed.sort();
                    return mut_accessed;
                } else {
                    return Vec::new();
                }
            }

            pub fn drain_mut_accessed(&mut self) -> Vec<$position_type> {
                let mut mut_accessed = self.mut_accessed.take().unwrap_or_default().into_iter().collect::<Vec<_>>();
                mut_accessed.sort();
                mut_accessed
            }


        }
    }
}

/// Provides a starter for test suite for a given grid type.
///
/// Generated symbols consists of:
/// - `TestData` - testing [`TileData`](crate::common::TileData) for the test suite.
/// - `set_up_grid(&mut grid)` - initializes the grid with `TestData` tiles.
/// - Two predefined tests:
///     - `test_grid_read_access` - tests reading access to the grid.
///     - `test_grid_write_access` - tests writing access to the grid.
/// - Structs and functions to write further tests:
///     - `NeighbourTestCase` and `AllNeighboursTestCase` - structs to test neighbours of the tiles.
///     - `test_neigbhours` and `test_all_neigbhours` - functions to test neighbours of the tiles.
///
///
/// # Arguments
/// * `grid` - Identifier of the grid for test suite.
/// * `size` - Type of the size of the grid.
/// * `dimension_count` - Number of dimensions in the grid.
macro_rules! __impl_grid_tests {
    (
        grid: $grid_type:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        direction: $direction_type:ty,
        direction_table: $direction_table_type:ident,
        dimension_count: $dimension_count:literal,
    ) => {

        use crate::TileData;

        /// Test data.
        ///
        /// It has `offset` `usize` field, containing the offset of the tile in the grid,
        /// and `varia` `usize` field, for testing mutability.
        struct TestData {
            offset: usize,
            varia: usize,
        }

        impl TestData {

            /// Creates a new test data with the specified offset. The `varia` field is initialized to 0.
            pub fn new(offset: usize) -> Self {
                Self {
                    offset: offset,
                    varia: 0,
                }
            }
        }

        impl TileData for TestData {}

        fn set_up_grid(grid: &mut $grid_type<TestData>) {
            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                grid.insert_data(&pos, TestData::new(size.offset(&pos)));
            }
        }

        #[test]
        fn test_grid_read_access() {
            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                let tile = grid.get_tile_at_position(&pos).unwrap();
                assert_eq!(
                    tile.grid_position(), pos,
                    "wrong position on position: {pos:?}; simple access"
                );
                assert_eq!(
                    tile.data().offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}; simple access"
                );
            }

            for pos in size.get_all_possible_positions() {
                let tile_data = grid.get_data_at_position(&pos).unwrap();
                assert_eq!(
                    tile_data.offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}; get_tile_at_position access"
                );
            }

            for tile in grid.iter_tiles() {
                assert_eq!(
                    tile.data().offset,
                    size.offset(&tile.grid_position()),
                    "wrong varia on position: {:?}; iter access", tile.grid_position()
                );
            }

            for tile in grid.drain() {
                assert_eq!(
                    tile.data().offset,
                    size.offset(&tile.grid_position()),
                    "wrong offset on position: {:?}; drain access", tile.grid_position()
                );
            }
        }

        #[test]
        fn test_grid_write_access() {
            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                let mut tile = grid.get_mut_tile_at_position(&pos).unwrap();
                tile.data().varia = size.offset(&pos);
            }

            for pos in size.get_all_possible_positions() {
                let tile = grid.get_data_at_position(&pos).unwrap();
                assert_eq!(
                    tile.offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}"
                );
                assert_eq!(tile.varia, tile.offset, "wrong varia on position: {pos:?}");
            }

            for pos in size.get_all_possible_positions() {
                let data = grid.get_mut_data_at_position(&pos).unwrap();
                data.varia *= 2;
            }

            for tile in grid.drain() {
                assert_eq!(
                    tile.data().varia,
                    size.offset(&tile.grid_position()) * 2,
                    "wrong offset on position: {:?}", tile.grid_position()
                );
            }
        }

        #[test]
        fn test_grid_write_access_with_tracking() {
            use std::collections::BTreeSet;

            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            // turn on tracking
            grid.mut_access_tracking(true);

            let size = *grid.size();
            let count_max = size.max_tile_count();

            // Get positions that should be tracked
            let positions_to_track = BTreeSet::from_iter(grid
                .get_all_positions()
                .into_iter()
                .filter(|pos| {
                    let offset = size.offset(pos);
                    offset < count_max / 2
                })
            );

            for pos in positions_to_track.iter() {
                grid.get_mut_tile_at_position(pos).unwrap();
            }

            let mut_accessed = BTreeSet::from_iter(grid.get_mut_accessed().iter().copied());

            for pos in positions_to_track.iter() {
                assert!(mut_accessed.contains(&pos), "pos: {pos:?} should be in mut_accessed");
            }
            for pos in mut_accessed.iter() {
                assert!(positions_to_track.contains(&pos), "pos: {pos:?} should be in positions_to_track");
            }

            let mut_accessed = BTreeSet::from_iter(grid.drain_mut_accessed().iter().copied());

            for pos in positions_to_track.iter() {
                assert!(mut_accessed.contains(&pos), "pos: {pos:?} should be in mut_accessed");
            }
            for pos in mut_accessed.iter() {
                assert!(positions_to_track.contains(&pos), "pos: {pos:?} should be in positions_to_track");
            }

            assert!(grid.get_mut_accessed().is_empty(), "mut_accessed should be empty");
        }

        struct NeighbourTestCase {
            pos: $position_type,
            direction: $direction_type,
            expected: Option<$position_type>,
        }

        impl NeighbourTestCase {
            const fn new(pos: $position_type, direction: $direction_type, expected: Option<$position_type>) -> Self {
                Self { pos, direction, expected }
            }
        }

        fn test_neigbhours(size: $size_type, cases: &[NeighbourTestCase]) {
            let mut grid = $grid_type::new(size);
            set_up_grid(&mut grid);

            for (
                i,
                NeighbourTestCase {
                    pos,
                    direction,
                    expected,
                },
            ) in cases.iter().enumerate() {
                let actual = grid.get_neighbour_at(pos, direction);
                match (actual, expected) {
                    (Some(neighbour), Some(expected_pos)) => {
                        assert_eq!(neighbour.grid_position(), *expected_pos, "wrong neighbour at position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                        assert_eq!(neighbour.data().offset, size.offset(&neighbour.grid_position()), "wrong offset on position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                    },
                    (Some(neighbour), None) =>
                        panic!("neigbour at position: {pos:?}; direction: {direction:?} should be None, but is: {neighbour_pos:?}. Case: {i}, Size: {size:?}", neighbour_pos = neighbour.grid_position()),
                    (None, Some(expected_pos)) =>
                        panic!("neigbour at position: {pos:?}; direction: {direction:?} should be {expected_pos:?}, but is None. Case: {i}, Size: {size:?}"),
                    (None, None) => {}

                }
            }
        }

        struct AllNeighboursTestCase {
             pos: $position_type,
            expected: $direction_table_type<Option<$position_type>>,
        }

        impl AllNeighboursTestCase {
            pub const fn new(pos: $position_type, expected: $direction_table_type<Option<$position_type>>) -> Self {
                Self { pos, expected }
            }
        }

        fn test_all_neigbhours(size: $size_type, cases: &[AllNeighboursTestCase]) {
            let mut grid = $grid_type::new(size);
            set_up_grid(&mut grid);

            for (
                i,
                AllNeighboursTestCase {
                    pos,
                    expected,
                },
            ) in cases.iter().enumerate() {
                let neighbours = grid.get_neighbours(pos);

                for direction in <$direction_type>::ALL {
                    let expected = &expected[direction];
                    let actual = &neighbours[direction].as_ref().and_then(|tile| Some(tile.grid_position()));

                    if expected.is_none() {
                        assert!(actual.is_none(), "neigbour at position: {pos:?}; direction: {direction:?} should be None, but is: {actual:?}. Case: {i}, Size: {size:?}");
                        continue;
                    } else {
                        assert!(actual.is_some(), "neigbour at position: {pos:?}; direction: {direction:?} should be {expected:?}, but is None. Case: {i}, Size: {size:?}");
                        assert_eq!(actual.unwrap(), expected.unwrap(), "neigbour at position: {pos:?}; direction: {direction:?} should be {expected:?}, but is {actual:?}. Case: {i}, Size: {size:?}");
                    }
                }
            }
        }
    }
}

pub(crate) use __impl_grid;

#[cfg(test)]
pub(crate) use __impl_grid_tests;
