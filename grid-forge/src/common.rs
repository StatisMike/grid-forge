//! Core structures and traits of the grid-forge.
//! 
//! Every kind of [`Dimensionality`](crate::core::common::Dimensionality) has its own implementation
//! of the core traits in its own exported module.
//! 
//! Dimensionality trait holds the concrete implementation of the core traits for the specific
//! dimensionality and informs rest of the crate logic.
//! 
//! Depending of the dimensionality:
//! - there is different number of dimensions, which informs the number of coordinates in the [`GridPosition`]
//! and number of bounds in the [`GridSize`].
//! - there is different set of [`Direction`] variants, which informs the number of neighbours for the
//! tile in the grid.
#[doc(inline)]
pub use crate::core::common::{
    Dimensionality, 
    Direction, DirectionTable,
    GridSize,
    GridPosition,
    GridMap,
    Tile, TileRef, TileMut, TileContainer, TileData,
};