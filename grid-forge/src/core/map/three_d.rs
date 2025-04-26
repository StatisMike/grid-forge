use crate::core::three_d::*;

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
    fn new(size: <ThreeDim as Dimensionality>::Size) -> Self {
        let count = size.max_tile_count();
        let mut tiles = Vec::with_capacity(count);
        for _ in 0..count {
            tiles.push(None);
        }
        Self { size, tiles }
    }

    fn size(&self) -> &<ThreeDim as Dimensionality>::Size {
        &self.size
    }
}

impl <Data: TileData> GridMap3D<Data> {
    pub fn insert_layer(&mut self, z: u32, layer: GridMap2D<Data>) {
        let layer_size = GridSize3D::from_2d(1, *layer.size());
        if !layer_size.is_contained_within(&self.size) {
            panic!("layer size is not contained within the grid size");
        }
        layer
            .drain()
            .into_iter()
            .map(
                |(pos, data)| 
                (GridPosition3D::new(pos.x(), pos.y(), z), data),
            )
            .for_each(
                |(pos, data)| 
                {
                    self.insert_tile((pos, data));
                }
            );
    }
}
