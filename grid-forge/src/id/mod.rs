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
