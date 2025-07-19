use crate::{core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, GridSize2D, TileContainer2D as _}, procgen_collapse::{option::PerOptionData2D, tile::CollapsibleTile2D}};
use grid_forge_core::procgen_collapse::queue::propagator::PropagateItem;
use grid_forge_core::id::TypedData;
use grid_forge_core::id::TypeIdSet;
use grid_forge_core::procgen_collapse::tile::CollapsedTileData;
use grid_forge_core::id::IdentTileBuilder;
use grid_forge_core::id::IdDefault;

use crate::procgen_collapse::error::CollapseError2D;

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

// use crate::procgen_collapse::tile::CommonCollapsibleTile2D;

grid_forge_core::__impl_collapsed_grid! {
    struct_name: CollapsedGrid2D,
    grid: GridMap2D,
    size: GridSize2D,
}

grid_forge_core::__impl_collapsible_grid! {
    struct_name: CollapsibleTileGrid2D,
    collapsible_data: CollapsibleTile2D,
    collapsed_grid: CollapsedGrid2D,
    grid: GridMap2D,
    position: GridPosition2D,
    direction: Direction2D,
    per_option_data: PerOptionData2D,
    error: CollapseError2D,
}
