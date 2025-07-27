pub mod option;
pub mod tile;
// pub mod grid;
// pub mod error;
pub mod queue;
pub mod singular;

use crate::core::{GridMap2D, GridPosition2D, GridSize2D};
use grid_forge_core::id::TypeIdSet;
use grid_forge_core::procgen_collapse::tile::CollapsedTileData;

use grid_forge_core::procgen_collapse::error::CollapseErrorKind;
use std::error::Error;
use std::fmt::Display;

grid_forge_core::__impl_collapsed_grid! {
    struct_name: CollapsedGrid2D,
    grid: GridMap2D,
    size: GridSize2D,
}

grid_forge_core::__impl_collapse_error! {
    error_name: CollapseError2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_collapible_grid_error! {
    error_name: CollapsibleGridError2D,
    positon: GridPosition2D,
    size: GridSize2D,
}
