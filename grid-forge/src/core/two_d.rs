//! 2D rectangular tile dimension.
//! 
//! # Overview
//! 
//! [`TwoDim`] is its [`Dimensionality`](crate::common::Dimensionality) implementation.
//! 
//! - it has two dimensions, x and y. The [`GridPosition2D`] contains address to the cel in the grid
//! with x and y coordinates, and [`GridSize2D`] holds the size of the grid in x and y dimensions.
//! - [`Direction2D`] is a direction in 2D space, with four possible variants in two dimension space.
//! - [`DirectionTable2D`] is fast lookup table for the data bound to the specific direction.
//! - [`GridMap2D`] is a [`GridMap`](crate::common::GridMap) implementation for 2D space.
//! 
//! There are three types of [`TileContainer`](crate::common::TileContainer) implemented for 2D space, containing
//! both the data and the position of the tile:
//! - [`Tile2D`] - container owning the data for a tile.
//! - [`TileRef2D`] - container with reference to the data for a tile.
//! - [`TileMut2D`] - container with mutable reference to the data for a tile.
//! 
//! # Examples
//! ```
//! // You can import whole `common` module for trait visibility and `two_d` for implementation.
//! use grid_forge::{common::*, two_d::*};
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
//! let size = GridSize2D::new(10, 10);
//! let mut map = GridMap2D::<CustomTileData>::new(size);
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
//!     let bar = pos.x() + pos.y();
//!     let foo = format!("foo{}", bar);
//!     let mut baz = HashMap::new();
//!     baz.insert(pos.x(), format!("baz{}", pos.x()));
//!     baz.insert(pos.y(), format!("baz{}", pos.y()));
//! 
//!     let data = CustomTileData {
//!         foo, bar, baz
//!     };
//! 
//!     // Insert the tile into the grid. You can do it by passing the position and the data separately.
//!     assert!(map.insert_data(&pos, data), "failed to insert data");
//! 
//!     // The Tile2D can be removed from the grid.
//!     tile_vec.push(map.remove_tile_at_position(&pos).expect("failed to retrieve tile")); 
//! }
//! 
//! for tile in tile_vec {
//!     // Tile holds both the position and the data.
//!     let pos = tile.grid_position();
//!     let data = tile.into_data();
//! 
//!     // Alternatively, the Tile2D can be also inserted into the grid.
//!     assert!(map.insert_tile(Tile2D::new(pos, data)), "failed to insert tile");
//! }
//! 
//! // We can also iterate on the tile references.
//! for tile in map.iter_tiles() {
//!     let x = tile.grid_position().x();
//!     let y = tile.grid_position().y();
//!     let coord_sum = x + y;
//! 
//!     assert_eq!(coord_sum, tile.data().bar);
//!     assert_eq!(format!("foo{}", coord_sum), tile.data().foo);
//! 
//!     let baz_x = tile.data().baz.get(&x).expect("failed to retrieve baz");
//!     let baz_y = tile.data().baz.get(&y).expect("failed to retrieve baz");
//! 
//!     assert_eq!(&format!("baz{}", x), baz_x);
//!     assert_eq!(&format!("baz{}", y), baz_y);
//! }
//! ```

#[doc(inline)]
pub use crate::core::{
    direction::two_d::{Direction2D, DirectionTable2D},
    map::two_d::GridMap2D,
    size::two_d::GridSize2D,
    position::two_d::GridPosition2D,
    tile::two_d::{Tile2D, TileRef2D, TileMut2D},
};

use crate::core::common::Dimensionality;

/// [Dimensionality] for basic 2D rectangular grid.
///
/// Its main types are:
/// - [Direction2D] as [Dimensionality::Dir].
/// - [GridSize2D] as [Dimensionality::Size].
/// - [GridPosition2D] as [Dimensionality::Pos].
///
#[derive(Debug, Copy, Clone)]
pub struct TwoDim {}
impl super::private::Sealed for TwoDim {}

impl Dimensionality for TwoDim {
    const N: usize = 2;

    type Dir = Direction2D;
    type Size = GridSize2D;
    type Pos = GridPosition2D;
}