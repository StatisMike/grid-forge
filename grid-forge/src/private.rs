use crate::{dimensions::{Dimensionality, Directions, GridPositionTrait, GridSize}, GridTile, GridTileRef, GridTileRefMut, TileData};

pub trait SealedGrid<Data: TileData, D: Dimensionality> {

    fn tiles(&self) -> &[Option<Data>];
    fn tiles_mut(&mut self) -> &mut [Option<Data>];
}

pub trait SealedContainer {}

pub trait SealedRef {}

pub trait SealedRefMut {}