use crate::core::two_d::*;

use super::private::*;

pub struct GridMap2D<Data: TileData> {
    size: GridSize2D,
    tiles: Vec<Option<Data>>,
}

impl<Data: TileData> SealedGrid<Data, TwoDim> for GridMap2D<Data> {
    fn tiles(&self) -> &[Option<Data>] {
        &self.tiles
    }

    fn tiles_mut(&mut self) -> &mut [Option<Data>] {
        &mut self.tiles
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> &Option<Data> {
        self.tiles.get_unchecked(index)
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<Data> {
        self.tiles.get_unchecked_mut(index)
    }
}

impl<Data: TileData> GridMap<TwoDim, Data> for GridMap2D<Data> {
    fn new(size: <TwoDim as Dimensionality>::Size) -> Self {
        let count = size.max_tile_count();
        let mut tiles = Vec::with_capacity(count);
        for _ in 0..count {
            tiles.push(None);
        }
        Self { size, tiles }
    }

    fn size(&self) -> &GridSize2D {
        &self.size
    }
}

#[cfg(test)]
mod tests {
    use crate::two_d::*;
    use crate::core::map::tests::*;

    #[test]
    fn test_2d_grid_read_access() {
        test_grid_read_access::<2, TwoDim, GridMap2D<TestData<TwoDim>>>();
    }

    #[test]
    fn test_2d_grid_write_access() {
        test_grid_write_access::<2, TwoDim, GridMap2D<TestData<TwoDim>>>();
    }

    #[test]
    fn test_2d_remapped() {
        test_remapped::<2, TwoDim, GridMap2D<TestData<TwoDim>>>(GridPosition2D::new(5, 5));
    }

    #[test]
    fn test_2_neighbours() {
        test_neighbours::<TwoDim, GridMap2D<TestData<TwoDim>>>(GridSize2D::new(10, 10), &[
            NeighbourTestCase {
                pos: GridPosition2D::new(0, 0),
                direction: Direction2D::Down,
                expected: Some(GridPosition2D::new(0, 1)),
            },
            NeighbourTestCase {
                pos: GridPosition2D::new(0, 0),
                direction: Direction2D::Left,
                expected: None,
            },
            NeighbourTestCase {
                pos: GridPosition2D::new(9, 9),
                direction: Direction2D::Up,
                expected: Some(GridPosition2D::new(9, 8)),
            },
            NeighbourTestCase {
                pos: GridPosition2D::new(9, 9),
                direction: Direction2D::Right,
                expected: None,
            },
        ]);
    }

    #[test]
    fn test_2_all_neighbours() {
        test_all_neighbours::<TwoDim, GridMap2D<TestData<TwoDim>>>(GridSize2D::new(10, 10), &[
            AllNeighboursTestCase {
                pos: GridPosition2D::new(0, 0),
                expected: vec![
                    GridPosition2D::new(0, 1),
                    GridPosition2D::new(1, 0),
                ],
            },
            AllNeighboursTestCase {
                pos: GridPosition2D::new(5, 5),
                expected: vec![
                    GridPosition2D::new(4, 5),
                    GridPosition2D::new(5, 4),
                    GridPosition2D::new(6, 5),
                    GridPosition2D::new(5, 6),
                ],
            },
            AllNeighboursTestCase {
                pos: GridPosition2D::new(9, 9),
                expected: vec![
                    GridPosition2D::new(8, 9),
                    GridPosition2D::new(9, 8),
                ],
            },
        ]);
    }
}
