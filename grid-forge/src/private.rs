use crate::{map::dimensions::Dimensionality, tile::TileData};

pub trait SealedGrid<Data: TileData, D: Dimensionality> {

    fn tiles(&self) -> &[Option<Data>];
    unsafe fn get_unchecked(&self, index: usize) -> &Option<Data>;
    
    fn tiles_mut(&mut self) -> &mut [Option<Data>];
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<Data>;
}

pub trait SealedContainer {}

pub trait SealedRef {}

pub trait SealedRefMut {}