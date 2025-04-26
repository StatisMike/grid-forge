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
