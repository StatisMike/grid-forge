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

pub(crate) mod common {
    use std::fmt::Debug;

    pub use crate::core::direction::common::*;
    pub use crate::core::map::common::*;
    pub use crate::core::position::common::*;
    pub use crate::core::size::common::*;
    pub use crate::core::tile::common::*;

    /// Trait declaring the dimensionality of the grid.
    pub trait Dimensionality:
        super::private::Sealed + 'static + Debug + Clone + Copy
    {
        /// Number of dimensions
        const N: usize;

        /// Directions for neighboring cells
        type Dir: Direction<Self>;

        /// Size of the grid
        type Size: GridSize<Self>;

        /// Position of the tile in the grid
        type Pos: GridPosition<Self>;
    }
}

pub mod two_d;
pub mod three_d;

mod private {
    pub trait Sealed {}
}
