//! Grid map library for Rust.
//!
//! `grid_forge` provides a set of tools for working with grid maps. It allows to define your own tile struct and
//! menage it in a dedicated grid map struct for given _dimensionality_.
//!
//! For every _dimensionality_ it provides a two main types of grid maps, depending on your use case:
//! - regular grid map, in which each [`TileData`](crate::core::TileData) is placed on a single position. Data is not
//! shared between tiles.
//! - shared data grid map, in which besides [`TileData`](crate::core::TileData) each tile needs to be also of some
//! _type_ (signified by [`TypedData`](crate::id::TypedData) trait). This allows to also share some [`SharedData`](crate::id::SharedData)
//! between tiles. It allows for more efficient memory usage if there is distinct, varying _state_ data and static _type_ data.
//!
//! The library is designed to be used in some _dimensionality_, which can be specified by enabling one or more
//! of the following features. Enabling at least one of them is required.
//!
//! - `2d` - two dimensional, rectangular grid map
//! - `3d` - three dimensional, cubic grid map
//!
//! > Currently, only these two dimensionalities are supported. Support for hexagonal grid map is planned on further date.
//!
//! Additionally, the library provides additional features, which can be conditionally enabled:
//!
//! - `image` (`2d` only) - use [image](https://crates.io/crates/image) crate for IO operations for grid maps.
//! - `procgen` - procedural generation algorithms for grid maps:
//!     - _collapsible_ - WFC/model synthesis algorithm
//!     - _walker_ - random walker algorithm
//! - `pathfinding` - A* pathfinding and range algorithms

#[cfg(not(any(feature = "2d", feature = "3d")))]
compile_error!("Must enable at least one dimension feature (2d or 3d)");

pub mod core;
pub mod id;
pub mod prelude;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "procgen")]
pub mod procgen_collapse;
