use crate::two_d::GridPosition2D;

super::macros::__impl_tile! {
    tile: Tile2D,
    tile_ref: TileRef2D,
    tile_mut: TileMut2D,
    tile_ref_shared: TileRefShared2D,
    tile_mut_shared: TileMutShared2D,
    position: GridPosition2D,
}
