use crate::{
    core::three_d::*,
    core::two_d::{GridPosition2D, GridSize2D, Tile2D},
    core::common::*,
};

use super::{private::*, two_d::GridMap2D};

pub struct GridMap3D<Data: TileData> {
    size: GridSize3D,
    tiles: Vec<Option<Data>>,
}

impl<Data: TileData> SealedGrid<Data, ThreeDim> for GridMap3D<Data> {
    #[inline]
    fn tiles(&self) -> &[Option<Data>] {
        &self.tiles
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> &Option<Data> {
        self.tiles.get_unchecked(index)
    }

    #[inline]
    fn tiles_mut(&mut self) -> &mut [Option<Data>] {
        &mut self.tiles
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<Data> {
        self.tiles.get_unchecked_mut(index)
    }
}

impl<Data: TileData> GridMap<ThreeDim, Data> for GridMap3D<Data> {

    type Tile = Tile3D<Data>;
    type TileRef<'a> = TileRef3D<'a, Data> where Data: 'a;
    type TileMut<'a> = TileMut3D<'a, Data> where Data: 'a;

    fn new(size: GridSize3D) -> Self {
        let count = size.max_tile_count();
        let mut tiles = Vec::with_capacity(count);
        for _ in 0..count {
            tiles.push(None);
        }
        Self { size, tiles }
    }

    #[inline]
    fn size(&self) -> &GridSize3D {
        &self.size
    }
}

impl<Data: TileData> GridMap3D<Data> {
    pub fn insert_layer(&mut self, z: u32, layer: GridMap2D<Data>) {
        let layer_size = GridSize3D::from_2d(1, *layer.size());
        if !layer_size.is_contained_within(&self.size) {
            panic!("layer size is not contained within the grid size");
        }
        layer
            .drain()
            .into_iter()
            .map(|tile| (GridPosition3D::new(tile.grid_position().x(), tile.grid_position().y(), z), tile.into_data()))
            .for_each(|(pos, data)| {
                self.insert_tile(Tile3D::new(pos, data));
            });
    }

    pub fn remove_layer(&mut self, z: u32) -> GridMap2D<Data> {
        let layer_size = GridSize2D::new(self.size.x(), self.size.y());
        let mut out = GridMap2D::new(layer_size);
        let positions = self
            .iter_all_positions()
            .filter(|pos| pos.z() == z)
            .collect::<Vec<_>>();
        for pos in positions {
            let tile: Option<Tile3D<Data>> = self.remove_tile_at_position(&pos);
            if let Some(tile) = tile {
                out.insert_tile(Tile2D::new(
                    GridPosition2D::from_slice(&tile.0.coords()[0..2]),
                    tile.1,
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::core::map::tests::*;
    use crate::three_d::*;

    #[test]
    fn test_3d_grid_read_access() {
        test_grid_read_access::<3, ThreeDim, GridMap3D<TestData<ThreeDim>>>();
    }

    #[test]
    fn test_3d_grid_write_access() {
        test_grid_write_access::<3, ThreeDim, GridMap3D<TestData<ThreeDim>>>();
    }

    #[test]
    fn test_3d_remapped() {
        test_remapped::<3, ThreeDim, GridMap3D<TestData<ThreeDim>>>(GridPosition3D::new(5, 5, 5));
    }

    #[test]
    fn test_3d_neighbours() {
        test_neighbours::<ThreeDim, GridMap3D<TestData<ThreeDim>>>(
            GridSize3D::new(10, 10, 10),
            &[
                NeighbourTestCase {
                    pos: GridPosition3D::new(0, 0, 0),
                    direction: Direction3D::Down,
                    expected: Some(GridPosition3D::new(0, 1, 0)),
                },
                NeighbourTestCase {
                    pos: GridPosition3D::new(0, 0, 0),
                    direction: Direction3D::Left,
                    expected: None,
                },
                NeighbourTestCase {
                    pos: GridPosition3D::new(9, 9, 9),
                    direction: Direction3D::Up,
                    expected: Some(GridPosition3D::new(9, 8, 9)),
                },
                NeighbourTestCase {
                    pos: GridPosition3D::new(9, 9, 9),
                    direction: Direction3D::Right,
                    expected: None,
                },
            ],
        );
    }

    // #[test]
    // fn test_3d_all_neighbours() {
    //     test_all_neighbours::<TwoDim, GridMap2D<TestData<TwoDim>>>(
    //         GridSize2D::new(10, 10),
    //         &[
    //             AllNeighboursTestCase {
    //                 pos: GridPosition2D::new(0, 0),
    //                 expected: vec![GridPosition2D::new(0, 1), GridPosition2D::new(1, 0)],
    //             },
    //             AllNeighboursTestCase {
    //                 pos: GridPosition2D::new(5, 5),
    //                 expected: vec![
    //                     GridPosition2D::new(4, 5),
    //                     GridPosition2D::new(5, 4),
    //                     GridPosition2D::new(6, 5),
    //                     GridPosition2D::new(5, 6),
    //                 ],
    //             },
    //             AllNeighboursTestCase {
    //                 pos: GridPosition2D::new(9, 9),
    //                 expected: vec![GridPosition2D::new(8, 9), GridPosition2D::new(9, 8)],
    //             },
    //         ],
    //     );
    // }
}
