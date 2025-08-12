pub use grid_forge_core::TileData;

#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::core::{
    // Direction
    Direction2D,
    DirectionTable2D,
    // Traits
    Grid2D as _,
    GridShared2D as _,
    // Grid map
    GridMap2D,
    GridMapShared2D,
    // Position and size
    GridPosition2D,
    GridSize2D,
    Tile2D,
    // Tiles
    TileContainer2D,
    TileMut2D,
    TileMutShared2D,
    TileRef2D,
    TileRefShared2D,
};

#[cfg(feature = "3d")]
#[doc(inline)]
pub use grid_forge_3d::core::{
    // Direction
    Direction3D,
    DirectionTable3D,
    // Traits
    Grid3D as _,
    GridShared3D as _,
    // Grid map
    GridMap3D,
    GridMapShared3D,
    // Position and size
    GridPosition3D,
    GridSize3D,
    Tile3D,
    // Tiles
    TileContainer3D,
    TileMut3D,
    TileMutShared3D,
    TileRef3D,
    TileRefShared3D,
};
