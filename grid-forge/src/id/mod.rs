use crate::core::common::TileData;

mod builders;
mod collection;

pub use builders::*;
pub use collection::*;

pub trait IdentifiableTileData
where
    Self: TileData,
{
    fn tile_type_id(&self) -> u64;
}

pub trait IdDefault: IdentifiableTileData {
    fn tile_type_default(tile_type_id: u64) -> Self;
}

/// Basic tile struct that implements [`IdentifiableTileData`], holding only the most basic information.
#[derive(Clone, Copy, Debug)]
pub struct BasicIdentTileData {
    tile_type_id: u64,
}

impl TileData for BasicIdentTileData {}

impl IdentifiableTileData for BasicIdentTileData {
    fn tile_type_id(&self) -> u64 {
        self.tile_type_id
    }
}

impl IdDefault for BasicIdentTileData {
    fn tile_type_default(tile_type_id: u64) -> Self {
        BasicIdentTileData { tile_type_id }
    }
}
