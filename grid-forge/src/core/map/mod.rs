//! Contains the core map functionality.
//!
//!

pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use super::private::*;

    use crate::core::common::*;

    /// Collection of tiles in [`Dimensionality`] space.
    /// 
    /// Each kind of dimension has its own dedicated implementation of this trait,
    /// as well as its associated classes.
    pub trait GridMap<D: Dimensionality, Data: TileData>: SealedGrid<Data, D> + Sized {

        /// Type owning the tile data, retrieved or inserted into the [`GridMap`].
        type Tile: Tile<D, Data>;
        /// Reference to the tile data retrieved from the [`GridMap`].
        type TileRef<'a>: TileRef<'a, D, Data> where Data: 'a, Self: 'a;
        /// Mutable reference to the tile data retrieved from the [`GridMap`].
        type TileMut<'a>: TileMut<'a, D, Data> where Data: 'a, Self: 'a;

        /// Creates a new empty grid with the specified dimensions.
        ///
        /// All tile slots are initialized as empty. Use insertion methods to populate the grid.
        fn new(size: D::Size) -> Self;

        /// Returns the size of the [`GridMap`].
        fn size(&self) -> &D::Size;

        /// Gets a direct reference to data at a position if present and valid.
        ///
        /// # Returns
        /// - `Some(&Data)` if position is valid and contains data
        /// - `None` otherwise
        ///
        /// For a version that returns position+data together, see [`get_tile_at_position`](GridMap::get_tile_at_position).
        fn get_data_at_position(&self, position: &D::Pos) -> Option<&Data> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            unsafe { self.get_unchecked(size.offset(&position)).as_ref() }
        }

        /// Gets an immutable tile reference containing both position and data.
        ///
        /// # Returns
        /// - [`Some(TileRef)`](GridMap::TileRef) if position is valid and contains data
        /// - `None` otherwise
        ///
        /// For direct data access without position, see [`get_data_at_position`](GridMap::get_data_at_position).
        fn get_tile_at_position<'a>(&'a self, position: &D::Pos) -> Option<Self::TileRef<'a>> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            unsafe {
                self.get_unchecked(size.offset(&position))
                    .as_ref()
                    .map(|data| (*position, data).into())
            }
        }

        /// Returns the direct mutable reference to the data stored at the position.
        /// 
        /// # Returns
        /// - `Some(&mut Data)` if position is valid and contains data
        /// - `None` otherwise
        /// 
        /// For a version that returns position+data together, see [`get_mut_tile_at_position`](GridMap::get_mut_tile_at_position).
        fn get_mut_data_at_position(&mut self, position: &D::Pos) -> Option<&mut Data> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            let offset = self.size().offset(position);
            unsafe { self.get_unchecked_mut(offset).as_mut() }
        }

        /// Gets a mutable tile reference containing both position and data.
        /// 
        /// # Returns
        /// - [`Some(TileMut)`](GridMap::TileMut) if position is valid and contains data
        /// - `None` otherwise
        /// 
        /// For direct data access without position, see [`get_mut_data_at_position`](GridMap::get_mut_data_at_position).
        fn get_mut_tile_at_position<'a>(&'a mut self, position: &D::Pos) -> Option<Self::TileMut<'a>> {
            if !self.size().is_position_valid(position) {
                return None;
            }
            let offset = self.size().offset(position);
            unsafe {
                self.get_unchecked_mut(offset)
                    .as_mut()
                    .map(|data| (*position, data).into())
            }
        }

        /// Gets mutliple immutable tile references containing both position and data.
        ///
        /// # Returns
        /// `Vec` of [`TileRef`](GridMap::TileRef) for each provided position if all positions are valid and contain data
        fn get_tiles_at_positions<'a>(&'a self, positions: &[D::Pos]) -> Vec<Self::TileRef<'a>> {
            positions
                .iter()
                .filter_map(|position| self.get_tile_at_position(position))
                .collect::<Vec<_>>()
        }

        /// Inserts a [`Tile`](GridMap::Tile) into the grid using its stored position.
        /// 
        /// # Returns
        /// - `true` if insertion succeeded (valid position)
        /// - `false` if position is out of bounds
        fn insert_tile(&mut self, tile: Self::Tile) -> bool
        {
            if !self.size().is_position_valid(&tile.grid_position()) {
                return false;
            }
            let offset = self.size().offset(&tile.grid_position());
            unsafe {
                self.get_unchecked_mut(offset).replace(tile.into_data());
            }
            true
        }

        /// Inserts raw data at a specific position.
        ///
        /// See [`insert_tile`](GridMap::insert_tile) for version using tile type.
        /// 
        /// # Returns
        /// - `true` if insertion succeeded (valid position)
        /// - `false` if position is out of bounds
        fn insert_data(&mut self, position: &D::Pos, data: Data) -> bool {
            if !self.size().is_position_valid(position) {
                return false;
            }
            let offset = self.size().offset(position);
            unsafe {
                self.get_unchecked_mut(offset).replace(data);
            }
            true
        }

        /// Removes and returns the tile at a position if present.
        ///
        /// # Returns
        /// - [`Some(Tile)`](GridMap::Tile) if position was valid and contained data
        /// - `None` otherwise
        fn remove_tile_at_position(&mut self, position: &D::Pos) -> Option<Self::Tile>
        {
            if !self.size().is_position_valid(position) {
                return None;
            }
            let offset = self.size().offset(position);

            unsafe {
                self.get_unchecked_mut(offset)
                    .take()
                    .and_then(|t| Some((*position, t).into()))
            }
        }

        /// Retrieve all neigbours of the provided position.
        ///
        /// Returns the [`TileRef`](GridMap::TileRef) holding the data and position for each neighbour of the provided position.
        fn get_neighbours<'a>(&'a self, position: &D::Pos) -> Vec<Self::TileRef<'a>> {
            let mut result = Vec::with_capacity(D::Dir::N);
            for direction in D::Dir::all() {
                if let Some(pos) = direction.march_step(position, &self.size()) {
                    if let Some(tile) = self.get_tile_at_position(&pos) {
                        result.push(tile);
                    }
                }
            }
            result
        }

        /// Retrieve neighbour in given [Direction] of the provided position.
        /// 
        /// Returns the [`TileRef`](GridMap::TileRef) holding the data or `None` if there is no data in neighbour position,
        /// or neighbour position is outside of the grid size.
        fn get_neighbour_at<'a>(
            &'a self,
            position: &D::Pos,
            direction: &D::Dir,
        ) -> Option<Self::TileRef<'a>> {
            if let Some(position) = direction.march_step(position, &self.size()) {
                return self.get_tile_at_position(&position);
            }
            None
        }

        /// Mutably retrieve neighbour in given [Direction] of the provided position.
        /// 
        /// Returns the [`TileMut`](GridMap::TileMut) holding the data or `None` if there is no data in neighbour position,
        /// or neighbour position is outside of the grid size.
        fn get_mut_neighbour_at<'a>(
            &'a mut self,
            position: &D::Pos,
            direction: &D::Dir,
        ) -> Option<Self::TileMut<'a>> {
            if let Some(position) = direction.march_step(position, &self.size()) {
                return self.get_mut_tile_at_position(&position);
            }
            None
        }

        /// Returns all taken [positions](crate::core::common::GridPosition) in the [`GridMap`].
        fn get_all_positions(&self) -> Vec<D::Pos> {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
                .collect()
        }

        /// Returns iterator over all taken [positions](crate::core::common::GridPosition) in the [`GridMap`].
        fn iter_all_positions<'a>(&'a self) -> impl Iterator<Item = D::Pos>
        where
            Data: 'a,
        {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
        }

        /// Returns all empty [positions](crate::core::common::GridPosition) in the [`GridMap`].
        fn get_all_empty_positions(&self) -> Vec<D::Pos> {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
                .collect()
        }

        /// Returns iterator over all empty [positions](crate::core::common::GridPosition) in the [`GridMap`].
        fn iter_all_empty_positions<'a>(&'a self) -> impl Iterator<Item = D::Pos>
        where
            Data: 'a,
        {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
        }

        /// Returns iterator over all tile slots in the [`GridMap`].
        /// 
        /// For each position in the [`GridMap`] there is a slot, which can be either empty or filled with data.
        /// 
        /// To iterate over [`TileMut`](GridMap::TileMut) of all filled positions, use [`iter_mut_tiles`](GridMap::iter_mut_tiles).
        fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>>
        where
            Data: 'a,
        {
            self.tiles_mut().iter_mut()
        }

        /// Returns iterator over all [`TileRef`](GridMap::TileRef) in the [`GridMap`].
        fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = Self::TileRef<'a>>
        where
            Data: 'a,
        {
            self.indexed_iter()
                .filter_map(|(pos, data)| data.as_ref().map(|d| (pos, d).into()))
        }

        /// Returns iterator over all [`TileMut`](GridMap::TileMut) in the [`GridMap`].
        /// 
        /// To iterate mutably over all tile slots, use [`iter_mut`](GridMap::iter_mut).
        fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = Self::TileMut<'a>>
        where
            Data: 'a,
        {
            self.indexed_iter_mut()
                .filter_map(|(pos, data)| data.as_mut().map(|d| (pos, d).into()))
        }

        /// Returns iterator over all tile slots in the [`GridMap`] and their [position](crate::core::common::GridPosition).
        /// 
        /// For each position in the [`GridMap`] there is a slot, which can be either empty or filled with data.
        /// This iterator returns both empty and filled slots.
        /// 
        /// # Other iterators:
        /// - over slots and their positions mutably: [`indexed_iter_mut`](GridMap::indexed_iter_mut).
        /// - over [`TileRef`](GridMap::TileRef) of all filled positions: [`iter_tiles`](GridMap::iter_tiles).
        /// - over [`TileMut`](GridMap::TileMut) of all filled positions: [`iter_mut_tiles`](GridMap::iter_mut_tiles).
        fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (D::Pos, &'a Option<Data>)>
        where
            Data: 'a,
        {
            self.tiles()
                .iter()
                .enumerate()
                .map(move |(idx, t)| (self.size().pos_from_offset(idx), t))
        }

        /// Returns mutable iterator over all tile slots in the [`GridMap`] and their [position](crate::core::common::GridPosition).
        /// 
        /// For each position in the [`GridMap`] there is a slot, which can be either empty or filled with data.
        /// This iterator returns both empty and filled slots, with mutable access to the slot.
        /// 
        /// # Other iterators:
        /// - over slots and their positions: [`indexed_iter`](GridMap::indexed_iter).
        /// - over [`TileRef`](GridMap::TileRef) of all filled positions: [`iter_tiles`](GridMap::iter_tiles).
        /// - over [`TileMut`](GridMap::TileMut) of all filled positions: [`iter_mut_tiles`](GridMap::iter_mut_tiles).
        fn indexed_iter_mut<'a>(
            &'a mut self,
        ) -> impl Iterator<Item = (D::Pos, &'a mut Option<Data>)>
        where
            Data: 'a,
        {
            let size = self.size().clone();
            self.tiles_mut()
                .iter_mut()
                .enumerate()
                .map(move |(idx, t)| (size.pos_from_offset(idx), t))
        }

        /// Drains all filled slots and returns them as a vec of [`Tile`](GridMap::Tile) with position shift.
        /// 
        /// The position of the tiles is shifted by the provided anchor position. Map is consumed
        /// after the operation.
        fn drain_remapped(mut self, anchor_pos: D::Pos) -> Vec<Self::Tile> {
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

        /// Drains all filled slots and returns them as a vec of [`Tile`](GridMap::Tile).
        /// 
        /// Map is consumed after the operation.
        fn drain(mut self) -> Vec<Self::Tile> {
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

        /// Fills all empty slots using the provided function.
        fn fill_empty_using(&mut self, func: fn(D::Pos) -> Data) {
            for (pos, t) in self.indexed_iter_mut() {
                if t.is_none() {
                    t.replace(func(pos));
                }
            }
        }

        /// Fills all empty slots with default value of the data.
        fn fill_empty_with_default(&mut self)
        where
            Data: Default,
        {
            let empty_positions = self.get_all_empty_positions();
            for pos in empty_positions {
                self.insert_data(&pos, Data::default());
            }
        }

        /// Fills all empty slots with clones of the provided data.
        fn fill_empty_with(&mut self, data: Data)
        where
            Data: Clone,
        {
            for pos in self.get_all_empty_positions() {
                self.insert_data(&pos, data.clone());
            }
        }

        /// Creates cloned tiles with positions offset by an anchor.
        ///
        /// The original grid remains intact. For moving tiles while consuming the grid,
        /// see [`drain_remapped`](GridMap::drain_remapped).
        fn get_remapped(&self, anchor_pos: D::Pos) -> Vec<Self::Tile>
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
    }
}

pub(crate) mod private {
    use crate::core::common::*;

    pub trait SealedGrid<Data: TileData, D: Dimensionality> {
        fn tiles(&self) -> &[Option<Data>];
        unsafe fn get_unchecked(&self, index: usize) -> &Option<Data>;

        fn tiles_mut(&mut self) -> &mut [Option<Data>];
        unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<Data>;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::marker::PhantomData;

    use crate::core::common::*;

    pub struct TestData<D: Dimensionality> {
        offset: usize,
        varia: usize,
        phantom: PhantomData<D>,
    }

    impl<D: Dimensionality> TestData<D> {
        pub fn new(offset: usize) -> Self {
            Self {
                offset: offset,
                varia: 0,
                phantom: PhantomData,
            }
        }
    }

    pub fn set_up_grid<D: Dimensionality>(grid: &mut impl GridMap<D, TestData<D>>) {
        let size = *grid.size();
        for pos in size.get_all_possible_positions() {
            grid.insert_data(&pos, TestData::new(size.offset(&pos)));
        }
    }

    impl<D: Dimensionality> TileData for TestData<D> {}

    pub fn test_grid_read_access<
        const DIM: usize,
        D: Dimensionality,
        G: GridMap<D, TestData<D>>,
    >() {
        let mut grid = G::new(D::Size::from_slice(&[10; DIM]));
        set_up_grid::<D>(&mut grid);

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

    pub fn test_grid_write_access<
        const DIM: usize,
        D: Dimensionality,
        G: GridMap<D, TestData<D>>,
    >() {
        let mut grid = G::new(D::Size::from_slice(&[10; DIM]));
        set_up_grid::<D>(&mut grid);

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

    pub fn test_remapped<const DIM: usize, D: Dimensionality, G: GridMap<D, TestData<D>>>(
        remap_pos: D::Pos,
    ) {
        let mut grid = G::new(D::Size::from_slice(&[10; DIM]));
        set_up_grid::<D>(&mut grid);

        let size = *grid.size();
        for tile in grid.drain_remapped(remap_pos) {
            let original_pos = size.pos_from_offset(tile.data().offset);
            assert_eq!(
                tile.grid_position(),
                original_pos + remap_pos,
                "wrong position on remapped position: {:?};", tile.grid_position()
            );
        }
    }

    pub struct NeighbourTestCase<D: Dimensionality> {
        pub pos: D::Pos,
        pub direction: D::Dir,
        pub expected: Option<D::Pos>,
    }

    pub fn test_neighbours<D: Dimensionality, G: GridMap<D, TestData<D>>>(
        size: D::Size,
        cases: &[NeighbourTestCase<D>],
    ) {
        let mut grid = G::new(size);
        set_up_grid::<D>(&mut grid);

        for (
            i,
            NeighbourTestCase {
                pos,
                direction,
                expected,
            },
        ) in cases.iter().enumerate()
        {
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

    pub struct AllNeighboursTestCase<D: Dimensionality> {
        pub pos: D::Pos,
        pub expected: Vec<D::Pos>,
    }

    pub fn test_all_neighbours<D: Dimensionality, G: GridMap<D, TestData<D>>>(
        size: D::Size,
        cases: &[AllNeighboursTestCase<D>],
    ) {
        let mut grid = G::new(size);
        set_up_grid::<D>(&mut grid);

        for (i, AllNeighboursTestCase { pos, expected }) in cases.iter().enumerate() {
            let actual = grid.get_neighbours(pos);

            let unmatched = actual
                .iter()
                .map(|tile| tile.grid_position())
                .filter(|p| !expected.contains(p))
                .collect::<Vec<_>>();

            let missing = expected
                .iter()
                .filter(|p| !actual.iter().any(|tile| tile.grid_position() == **p))
                .collect::<Vec<_>>();

            if !unmatched.is_empty() {
                panic!("unmatched: {unmatched:?}. Case: {i}, Size: {size:?}");
            }
            if !missing.is_empty() {
                panic!("missing: {missing:?}. Case: {i}, Size: {size:?}");
            }
        }
    }
}
