pub mod data;
pub mod option;
pub mod pattern;
pub mod queue;
pub mod tile;

#[cfg(test)]
mod test;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;

use grid_forge_core::id::TypeIdSet;
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::error::CollapseErrorKind;

use crate::core::{GridMap2D, GridPosition2D, GridSize2D};

grid_forge_core::__impl_debug_subscriber! {
    struct_name: DebugSubscriber2D,
}

grid_forge_core::__impl_collapse_history_subscriber! {
    struct_name: HistorySubscriber2D,
    history_item_name: HistoryItem2D,
    position: GridPosition2D,
}

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
