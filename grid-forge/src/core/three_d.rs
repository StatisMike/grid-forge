//! 3D rectangular tile dimension.
//! 
//! # Overview
//! 
//! [`ThreeDim`] is its [`Dimensionality`](crate::common::Dimensionality) implementation.
//! 
//! - it has three dimensions, x, y and z. The [`GridPosition3D`] contains address to the cel in the grid
//! with x, y and z coordinates, and [`GridSize3D`] holds the size of the grid in x, y, z dimensions.
//! - [`Direction3D`] is a direction in 3D space, with six possible variants in three dimensional space.
//! - [`DirectionTable3D`] is fast lookup table for the data bound to the specific direction.
//! - [`GridMap3D`] is a [`GridMap`](crate::common::GridMap) implementation for 3D space.
//! 
//! There are three types of [`TileContainer`](crate::common::TileContainer) implemented for 3D space, containing
//! both the data and the position of the tile:
//! - [`Tile3D`] - container owning the data for a tile.
//! - [`TileRef3D`] - container with reference to the data for a tile.
//! - [`TileMut3D`] - container with mutable reference to the data for a tile.
//! 
//! # Examples
//! ```
//! // You can import whole `common` module for trait visibility and `three_d` for implementations.
//! use grid_forge::{common::*, three_d::*};
//! use std::collections::HashMap;
//! 
//! // Create a custom tile data.
//! struct CustomTileData {
//!     pub foo: String,
//!     pub bar: u32,
//!     pub baz: HashMap<u32, String>
//! };
//! 
//! // Implement a marker trait for the custom tile data.
//! impl TileData for CustomTileData {}
//! 
//! // Initialize a new empty grid map with the specified size.
//! let size = GridSize3D::new(10, 10, 10);
//! let mut map = GridMap3D::<CustomTileData>::new(size);
//! 
//! // Grid is now empty.
//! for pos in size.get_all_possible_positions() {
//!     assert!(map.get_data_at_position(&pos).is_none());
//! }
//! 
//! let mut tile_vec = Vec::new();
//! 
//! // Populate the grid with some data.
//! for pos in size.get_all_possible_positions() {
//! 
//!     let bar = pos.x() + pos.y() + pos.z();
//!     let foo = format!("foo{}", bar);
//!     let mut baz = HashMap::new();
//! 
//!     /// Each dimension is available as a separate field.
//!     baz.insert(pos.x(), format!("baz{}", pos.x()));
//!     baz.insert(pos.y(), format!("baz{}", pos.y()));
//!     baz.insert(pos.z(), format!("baz{}", pos.z()));
//! 
//!     let data = CustomTileData {
//!         foo, bar, baz
//!     };
//! 
//!     // Insert the tile into the grid. You can do it by passing the position and the data separately.
//!     assert!(map.insert_data(&pos, data), "failed to insert data");
//! 
//!     // The Tile3D can be removed from the grid.
//!     tile_vec.push(map.remove_tile_at_position(&pos).expect("failed to retrieve tile")); 
//! }
//! 
//! for tile in tile_vec {
//!     // Tile holds both the position and the data.
//!     let pos = tile.grid_position();
//!     let data = tile.into_data();
//! 
//!     // Alternatively, the Tile3D can be also inserted into the grid.
//!     assert!(map.insert_tile(Tile3D::new(pos, data)), "failed to insert tile");
//! }
//! 
//! // We can also iterate on the tile references.
//! for tile in map.iter_tiles() {
//! 
//!     /// Coords can be also retrieved as an array.
//!     let [x, y, z] = tile.grid_position().coords();
//! 
//!     let coord_sum = x + y + z;
//! 
//!     assert_eq!(coord_sum, tile.data().bar);
//!     assert_eq!(format!("foo{}", coord_sum), tile.data().foo);
//! 
//!     let baz_x = tile.data().baz.get(&x).expect("failed to retrieve baz");
//!     let baz_y = tile.data().baz.get(&y).expect("failed to retrieve baz");
//!     let baz_z = tile.data().baz.get(&z).expect("failed to retrieve baz");
//! 
//!     assert_eq!(&format!("baz{}", x), baz_x);
//!     assert_eq!(&format!("baz{}", y), baz_y);
//!     assert_eq!(&format!("baz{}", z), baz_z);
//! }
//! ```

#[doc(inline)]
pub use crate::core::{
    direction::three_d::{Direction3D, DirectionTable3D},
    map::three_d::GridMap3D,
    size::three_d::GridSize3D,
    position::three_d::GridPosition3D,
    tile::three_d::{Tile3D, TileRef3D, TileMut3D},
};

use crate::core::common::Dimensionality;

/// [Dimensionality] for basic 3D rectangular grid.
///
/// Its main types are:
/// - [Direction3D] as [Dimensionality::Dir].
/// - [GridSize3D] as [Dimensionality::Size].
/// - [GridPosition3D] as [Dimensionality::Pos].
///
#[derive(Debug, Copy, Clone)]
pub struct ThreeDim {}
impl super::private::Sealed for ThreeDim {}

impl Dimensionality for ThreeDim {
    const N: usize = 3;

    type Dir = Direction3D;
    type Size = GridSize3D;
    type Pos = GridPosition3D;
}