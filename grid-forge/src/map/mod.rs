
use grid::Grid;

use crate::{private, GridTile, TileData};
use crate::tile::{GridTileRef, GridTileRefMut};
use crate::dimensions::*;

pub mod dimensions;

pub mod two_dim {
    use crate::TileData;
    use crate::dimensions::two_dim::*;
    use super::*;

    pub struct GridMap2D<Data: TileData> {
        size: GridSize2D,
        tiles: Vec<Option<Data>>,
    }

    impl <Data: TileData>private::SealedGrid<Data, TwoDim> for GridMap2D<Data> {
    
        fn tiles(&self) -> &[Option<Data>] {
            &self.tiles
        }
    
        fn tiles_mut(&mut self) -> &mut [Option<Data>] {
            &mut self.tiles
        }
        
    }

    impl <Data: TileData> GridMap<TwoDim,Data> for GridMap2D<Data> {
        fn new(size: <TwoDim as Dimensionality>::Size) -> Self {
            let count = size.tile_count();
            let mut tiles = Vec::with_capacity(count);
            for _ in 0..count {
                tiles.push(None);
            }
            Self { size, tiles }
        }

        fn size(&self) -> &GridSize2D    {
            &self.size
        }
    }
}

pub mod three_dims {
    use crate::TileData;
    use crate::dimensions::three_dims::*;
    use super::*;


    type PosData3D<Data> = (GridPosition3D, Data); 
    type PosDataRef3D<'a, Data> = (GridPosition3D, &'a Data);
    type PosDataMutRef3D<'a, Data> = (GridPosition3D, &'a mut Data);

    pub struct GridMap3D<Data: TileData> {
        size: GridSize3D,
        tiles: Vec<Option<Data>>,
    }

    impl <Data: TileData> private::SealedGrid<Data, ThreeDim> for GridMap3D<Data> {
        fn tiles(&self) -> &[Option<Data>] {
            &self.tiles
        }
    
        fn tiles_mut(&mut self) -> &mut [Option<Data>] {
            &mut self.tiles
        }
    }

    impl <Data: TileData> GridMap<ThreeDim,Data> for GridMap3D<Data> {
        fn new(size: <ThreeDim as Dimensionality>::Size) -> Self {
            let count = size.tile_count();
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
}


pub trait GridMap<D: Dimensionality, Data: TileData>: private::SealedGrid<Data, D> + Sized 
{
    fn new(size: D::Size) -> Self;

    fn size(&self) -> &D::Size;

    fn get_tile_at_position(&self, position: &D::Pos) -> Option<(D::Pos, &Data)> {
        let size = self.size().clone();
        if !size.is_position_valid(position) {
            return None;
        }
        self.tiles()
                .get(size.offset(&position))
                .unwrap()
                .as_ref()
                .map(|data| (*position, data))
                

    }

    fn get_mut_tile_at_position(&mut self, position: &D::Pos) -> Option<(D::Pos, &mut Data)> {
        if !self.size().is_position_valid(position) {
            return None;
        }
        let offset = self.size().offset(position);
        self.tiles_mut()
                .get_mut(offset)
                .unwrap()
                .as_mut()
                .map(|data| (*position, data))
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
        self.tiles_mut().get_mut(offset).unwrap().replace(tile.1);
        true
    }

    fn insert_data(&mut self, position: &D::Pos, data: Data) -> bool {
        if !self.size().is_position_valid(position) {
            return false;
        }
        let offset = self.size().offset(position);

        self.tiles_mut().get_mut(offset).unwrap().replace(data);
        true
    }

    fn remove_tile_at_position(&mut self, position: &D::Pos) -> bool {
        if !self.size().is_position_valid(position) {
            return false;
        }
        let offset = self.size().offset(position);

        if let Some(tile) = self.tiles_mut().get_mut(offset) {
            *tile = None;
        }
        true
    }

    fn get_neighbours(&self, position: &D::Pos) -> Vec<(D::Pos, &Data)> {
        D::Dir::all()
        .iter()
        .filter_map(|direction| self.get_neighbour_at(position, direction))
        .collect::<Vec<_>>()
    }

    fn get_neighbour_at(&self, position: &D::Pos, direction: &D::Dir) -> Option<(D::Pos, &Data)> {
        if let Some(position) = direction.march_step(position, &self.size()) {
            return self.get_tile_at_position(&position);
        }
        None
    }

    fn get_mut_neighbour_at(&mut self, position: &D::Pos, direction: &D::Dir) -> Option<(D::Pos, &mut Data)> {
        if let Some(position) = direction.march_step(position, &self.size()) {
            return self.get_mut_tile_at_position(&position);
        }
        None
    }

    fn get_all_positions(&self) -> Vec<D::Pos> {
        self
        .indexed_iter()
        .filter_map(|(pos, t)| {
            if t.is_some() {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
    }

    fn get_all_empty_positions(&self) -> Vec<D::Pos> {
        self
        .indexed_iter()
        .filter_map(|(pos, t)| {
            if t.is_none() {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>> where Data: 'a {
        self.tiles_mut().iter_mut()
    }

    fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = (D::Pos, &'a Data)> where Data: 'a {
        self.indexed_iter()
            .filter_map(|(pos, data)| data.as_ref().map(|d| (pos, d)))
    }

    fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = (D::Pos, &'a mut Data)> where Data: 'a {
        self.indexed_iter_mut()
            .filter_map(|(pos, data)| data.as_mut().map(|d| (pos, d)))
    }

    fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (D::Pos, &'a Option<Data>)> where Data: 'a {
        self.tiles()
            .iter()
            .enumerate()
            .map(move |(idx, t)| (self.size().pos_from_offset(idx), t))
    }

    fn indexed_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (D::Pos, &'a mut Option<Data>)> where Data: 'a {
        let size = self.size().clone();
        self
        .tiles_mut()
        .iter_mut()
        .enumerate()
        .map(move |(idx, t)| {
            (size.pos_from_offset(idx), t)
        })
    }

    fn drain_remapped(mut self, anchor_pos: D::Pos) -> Vec<(D::Pos, Data)> {
        self
            .indexed_iter_mut()
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
        self
            .indexed_iter_mut()
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
    where Data: Default
    {
        let empty_positions = self.get_all_empty_positions();
        for pos in empty_positions {
            self.insert_data(&pos, Data::default());
        }
    }

    fn fill_empty_with(&mut self, data: Data)
    where Data: Clone
    {
        for pos in self.get_all_empty_positions() {
            self.insert_data(&pos, data.clone());
        }
    }

    fn get_remapped(&self, anchor_pos: D::Pos) -> Vec<(D::Pos, Data)> 
    where Data: Clone
    {
        self
            .indexed_iter()
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