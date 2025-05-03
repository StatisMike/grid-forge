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

    /// Trait declaring the number of dimensions in the space of the grid.
    pub trait Dimensionality:
        super::private::Sealed + 'static + Debug + Clone + Copy + Default
    {
        /// Number of dimensions
        const N: usize;

        /// Directions for neighboring cells
        type Dir: Direction<Self>;

        /// Size of the grid
        type Size: GridSize<Self>;

        /// Position of the tile in the grid
        type Pos: GridPositionTrait<Self>;
    }
}

pub(crate) mod two_d {
    pub use super::common::*;

    pub use crate::core::direction::two_d::*;
    pub use crate::core::map::two_d::*;
    pub use crate::core::position::two_d::*;
    pub use crate::core::size::two_d::*;
    pub use crate::core::tile::two_d::*;

    #[derive(Debug, Copy, Clone, Default)]
    pub struct TwoDim {}
    impl super::private::Sealed for TwoDim {}

    impl Dimensionality for TwoDim {
        const N: usize = 2;

        type Dir = Direction2D;
        type Size = GridSize2D;
        type Pos = GridPosition2D;
    }
}

pub(crate) mod three_d {
    pub use super::common::*;

    pub use crate::core::direction::three_d::*;
    pub use crate::core::map::three_d::*;
    pub use crate::core::position::three_d::*;
    pub use crate::core::size::three_d::*;
    pub use crate::core::tile::three_d::*;

    /// Three-dimensional space.
    #[derive(Debug, Copy, Clone, Default)]
    pub struct ThreeDim {}
    impl super::private::Sealed for ThreeDim {}

    impl Dimensionality for ThreeDim {
        const N: usize = 3;

        type Dir = Direction3D;
        type Size = GridSize3D;
        type Pos = GridPosition3D;
    }
}

mod private {
    pub trait Sealed {}
}
