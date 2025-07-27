pub use grid_forge_core::TileData;

#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::core::{
    // Position and size
    GridPosition2D,
    GridSize2D,
    // Direction
    Direction2D,
    DirectionTable2D,
    // Grid map
    GridMap2D,
    GridMapShared2D,
    // Tiles
    TileContainer2D,
    Tile2D, TileRef2D, TileMut2D, TileRefShared2D, TileMutShared2D,
    // Traits
    Grid2D as _,
};

#[cfg(feature = "3d")]
#[doc(inline)]
pub use grid_forge_3d::core::{
    // Position and size
    GridPosition3D,
    GridSize3D,
    // Direction
    Direction3D,
    DirectionTable3D,
    // Grid map
    GridMap3D,
    GridMapShared3D,
    // Tiles
    TileContainer3D,
    Tile3D, TileRef3D, TileMut3D, TileRefShared3D, TileMutShared3D,
    // Traits
    Grid3D as _,
};