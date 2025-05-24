use crate::three_d::GridPosition3D;

super::macros::__impl_tile! {
    tile: Tile3D,
    tile_ref: TileRef3D,
    tile_mut: TileMut3D,
    tile_ref_shared: TileRefShared3D,
    tile_mut_shared: TileMutShared3D,
    position: GridPosition3D,
}
