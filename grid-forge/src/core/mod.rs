//! Core structures and traits.
//!
//! Every kind of [`Dimensionality`](crate::core::common::Dimensionality) has its own implementation
//! of the core structures and traits in its own exported module.
//!
//! It needs to define all the traits defined in the [`common`](crate::core::common) module, as well as
//! the [`Dimensionality`](crate::core::common::Dimensionality) trait itself.

pub(crate) mod direction;
mod map;
mod position;
mod size;
mod tile;

pub use tile::common::{TileContainer, TileData};
pub mod three_d;
pub mod two_d;
