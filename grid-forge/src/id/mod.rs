use crate::core::common::TileData;

mod builders;
mod collection;

pub use builders::*;
pub use collection::*;

/// Identifiable tile data trait.
/// 
/// Its implementation makes the specific [`TileData`] identifiable and discernable from other tile instances in regards to *tile
/// type*. Some exemplar tile types could be :
/// - terrain type: grass, dirt, rock, etc.
/// - specific underlying visual representation: 2D sprite, 3D mesh, etc.
/// 
/// General rules of tile types when implementing this trait should be:
///
/// - its [`GridPosition`](crate::common::GridPosition) **should not be ever taken into account**. Tile of these same type could 
///   be placed on different positions on the [`GridMap`](crate::common::GridMap).
/// - other properties of the tile (such as visual representation) *can* be taken into account depending on your specific
///   needs.
/// 
/// Procedural generation algorithms rely on this trait to identify the tiles to be placed on the generated map.
/// 
/// # Example
/// ```
/// use grid_forge::common::TileData;
/// use grid_forge::id::TypedData;
/// 
/// enum TileType {
///     Grass,
///     Dirt,
///     Rock,
/// }
/// 
/// struct TerrainTile {
///     tile_type: TileType,
/// }
/// 
/// impl TileData for TerrainTile {}
/// 
/// impl TypedData for TerrainTile {
///     fn tile_type_id(&self) -> u64 {
///         match self.tile_type {
///             TileType::Grass => 0,
///             TileType::Dirt => 1,
///             TileType::Rock => 2,
///         }
///     }
/// }
/// ```
pub trait TypedData
where
    Self: TileData,
{
    fn tile_type_id(&self) -> u64;
}

/// Default tile data trait.
///
/// Its implementation makes a default [`TileData`] instantiable from its specific `tile_type_id`.
pub trait IdDefault: TypedData {
    fn tile_type_default(tile_type_id: u64) -> Self;
}

/// Basic tile struct that implements [`TypedData`], holding only the `tile_type_id`.
#[derive(Clone, Copy, Debug)]
pub struct BasicTypedData(u64);

impl TileData for BasicTypedData {}

impl TypedData for BasicTypedData {
    fn tile_type_id(&self) -> u64 {
        self.0
    }
}

impl IdDefault for BasicTypedData {
    fn tile_type_default(tile_type_id: u64) -> Self {
        BasicTypedData(tile_type_id) 
    }
}
