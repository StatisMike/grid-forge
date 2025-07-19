use crate::core::{GridPosition2D, GridSize2D};

use std::error::Error;
use std::fmt::Display;
use grid_forge_core::procgen_collapse::{error::CollapseErrorKind};

grid_forge_core::__impl_collapse_error! {
    error_name: CollapseError2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_collapible_grid_error! {
    error_name: CollapsibleGridError2D,
    positon: GridPosition2D,
    size: GridSize2D,
}