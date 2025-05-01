//! Contains the core map functionality.
//!
//!

pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use super::private::*;

    use crate::core::common::*;

    pub trait GridMap<D: Dimensionality, Data: TileData>: SealedGrid<Data, D> + Sized {
        fn new(size: D::Size) -> Self;

        fn size(&self) -> &D::Size;

        fn get_data_at_position(&self, position: &D::Pos) -> Option<&Data> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            unsafe { self.get_unchecked(size.offset(&position)).as_ref() }
        }

        fn get_tile_at_position(&self, position: &D::Pos) -> Option<(D::Pos, &Data)> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            unsafe {
                self.get_unchecked(size.offset(&position))
                    .as_ref()
                    .map(|data| (*position, data))
            }
        }

        fn get_mut_data_at_position(&mut self, position: &D::Pos) -> Option<&mut Data> {
            let size = self.size().clone();
            if !size.is_position_valid(position) {
                return None;
            }
            let offset = self.size().offset(position);
            unsafe { self.get_unchecked_mut(offset).as_mut() }
        }

        fn get_mut_tile_at_position(&mut self, position: &D::Pos) -> Option<(D::Pos, &mut Data)> {
            if !self.size().is_position_valid(position) {
                return None;
            }
            let offset = self.size().offset(position);
            unsafe {
                self.get_unchecked_mut(offset)
                    .as_mut()
                    .map(|data| (*position, data))
            }
        }

        fn get_tiles_at_positions(&self, positions: &[D::Pos]) -> Vec<(D::Pos, &Data)> {
            positions
                .iter()
                .filter_map(|position| self.get_tile_at_position(position))
                .collect::<Vec<_>>()
        }

        fn insert_tile(&mut self, tile: (D::Pos, Data)) -> bool {
            if !self.size().is_position_valid(&tile.0) {
                return false;
            }
            let offset = self.size().offset(&tile.0);
            unsafe {
                self.get_unchecked_mut(offset).replace(tile.1);
            }
            true
        }

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

        fn remove_tile_at_position(&mut self, position: &D::Pos) -> bool {
            if !self.size().is_position_valid(position) {
                return false;
            }
            let offset = self.size().offset(position);

            unsafe {
                self.get_unchecked_mut(offset).take();
            }

            true
        }

        fn get_neighbours(&self, position: &D::Pos) -> Vec<(D::Pos, &Data)> {
            let mut result = Vec::with_capacity(D::Dir::N);
            for direction in D::Dir::all() {
                if let Some(pos) = direction.march_step(position, &self.size()) {
                    result.push(self.get_tile_at_position(&pos).unwrap());
                }
            }
            result
        }

        fn get_neighbour_at(
            &self,
            position: &D::Pos,
            direction: &D::Dir,
        ) -> Option<(D::Pos, &Data)> {
            if let Some(position) = direction.march_step(position, &self.size()) {
                return self.get_tile_at_position(&position);
            }
            None
        }

        fn get_mut_neighbour_at(
            &mut self,
            position: &D::Pos,
            direction: &D::Dir,
        ) -> Option<(D::Pos, &mut Data)> {
            if let Some(position) = direction.march_step(position, &self.size()) {
                return self.get_mut_tile_at_position(&position);
            }
            None
        }

        fn get_all_positions(&self) -> Vec<D::Pos> {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
                .collect()
        }

        fn get_all_empty_positions(&self) -> Vec<D::Pos> {
            self.indexed_iter()
                .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
                .collect()
        }

        fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>>
        where
            Data: 'a,
        {
            self.tiles_mut().iter_mut()
        }

        fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = (D::Pos, &'a Data)>
        where
            Data: 'a,
        {
            self.indexed_iter()
                .filter_map(|(pos, data)| data.as_ref().map(|d| (pos, d)))
        }

        fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = (D::Pos, &'a mut Data)>
        where
            Data: 'a,
        {
            self.indexed_iter_mut()
                .filter_map(|(pos, data)| data.as_mut().map(|d| (pos, d)))
        }

        fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (D::Pos, &'a Option<Data>)>
        where
            Data: 'a,
        {
            self.tiles()
                .iter()
                .enumerate()
                .map(move |(idx, t)| (self.size().pos_from_offset(idx), t))
        }

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

        fn drain_remapped(mut self, anchor_pos: D::Pos) -> Vec<(D::Pos, Data)> {
            self.indexed_iter_mut()
                .filter_map(|(pos, t)| {
                    if t.is_none() {
                        None
                    } else {
                        Some((anchor_pos + pos, t.take().unwrap()))
                    }
                })
                .collect()
        }

        fn drain(mut self) -> Vec<(D::Pos, Data)> {
            self.indexed_iter_mut()
                .filter_map(|(pos, t)| {
                    if t.is_none() {
                        None
                    } else {
                        Some((pos, t.take().unwrap()))
                    }
                })
                .collect()
        }

        fn fill_empty_using(&mut self, func: fn(D::Pos) -> Data) {
            for (pos, t) in self.indexed_iter_mut() {
                if t.is_none() {
                    t.replace(func(pos));
                }
            }
        }

        fn fill_empty_with_default(&mut self)
        where
            Data: Default,
        {
            let empty_positions = self.get_all_empty_positions();
            for pos in empty_positions {
                self.insert_data(&pos, Data::default());
            }
        }

        fn fill_empty_with(&mut self, data: Data)
        where
            Data: Clone,
        {
            for pos in self.get_all_empty_positions() {
                self.insert_data(&pos, data.clone());
            }
        }

        fn get_remapped(&self, anchor_pos: D::Pos) -> Vec<(D::Pos, Data)>
        where
            Data: Clone,
        {
            self.indexed_iter()
                .filter_map(|(pos, t)| {
                    if t.is_some() {
                        Some((anchor_pos + pos, t.clone().unwrap()))
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
            let (tile_pos, tile_data) = grid.get_tile_at_position(&pos).unwrap();
            assert_eq!(
                tile_pos, pos,
                "wrong position on position: {pos:?}; simple access"
            );
            assert_eq!(
                tile_data.offset,
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

        for (pos, tile) in grid.iter_tiles() {
            assert_eq!(
                tile.offset,
                size.offset(&pos),
                "wrong varia on position: {pos:?}; iter access"
            );
        }

        for (pos, tile) in grid.drain() {
            assert_eq!(
                tile.offset,
                size.offset(&pos),
                "wrong offset on position: {pos:?}; drain access"
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
            let tile = grid.get_mut_tile_at_position(&pos).unwrap();
            tile.1.varia = size.offset(&pos);
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

        for (pos, data) in grid.drain() {
            assert_eq!(
                data.varia,
                size.offset(&pos) * 2,
                "wrong offset on position: {pos:?}"
            );
        }
    }

    pub fn test_remapped<const DIM: usize, D: Dimensionality, G: GridMap<D, TestData<D>>>(
        remap_pos: D::Pos,
    ) {
        let mut grid = G::new(D::Size::from_slice(&[10; DIM]));
        set_up_grid::<D>(&mut grid);

        let size = *grid.size();
        for (remapped_pos, data) in grid.drain_remapped(remap_pos) {
            let original_pos = size.pos_from_offset(data.offset);
            assert_eq!(
                remapped_pos,
                original_pos + remap_pos,
                "wrong position on remapped position: {remapped_pos:?};"
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
                (Some((neighbour_pos, data)), Some(expected_pos)) => {
                    assert_eq!(neighbour_pos, *expected_pos, "wrong neighbour at position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                    assert_eq!(data.offset, size.offset(&neighbour_pos), "wrong offset on position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                },
                (Some((neighbour_pos, _)), None) =>
                    panic!("neigbour at position: {pos:?}; direction: {direction:?} should be None, but is: {neighbour_pos:?}. Case: {i}, Size: {size:?}"),
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
                .map(|tile| tile.0)
                .filter(|p| !expected.contains(p))
                .collect::<Vec<_>>();

            let missing = expected
                .iter()
                .filter(|p| !actual.iter().any(|tile| tile.0 == **p))
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
