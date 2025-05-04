use crate::{
    core::three_d::*,
    two_d::{GridPosition2D, GridSize2D, Tile2D},
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
            .map(|(pos, data)| (GridPosition3D::new(pos.x(), pos.y(), z), data))
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
