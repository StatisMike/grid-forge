pub mod data;
pub mod option;
pub mod pattern;
pub mod queue;
pub mod tile;

// TODO: Fix tests for 3D
// #[cfg(test)]
// mod test;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;

use grid_forge_core::id::TypeIdSet;
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::error::CollapseErrorKind;

use crate::core::{GridMap3D, GridPosition3D, GridSize3D};

grid_forge_core::__impl_debug_subscriber! {
    struct_name: DebugSubscriber3D,
}

grid_forge_core::__impl_collapse_history_subscriber! {
    struct_name: HistorySubscriber3D,
    history_item_name: HistoryItem3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_collapsed_grid! {
    struct_name: CollapsedGrid3D,
    grid: GridMap3D,
    size: GridSize3D,
}

grid_forge_core::__impl_collapse_error! {
    error_name: CollapseError3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_collapible_grid_error! {
    error_name: CollapsibleGridError3D,
    positon: GridPosition3D,
    size: GridSize3D,
}
