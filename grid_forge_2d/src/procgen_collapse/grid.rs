use crate::{core::{Direction2D, Grid2D as _, GridMap2D, GridPosition2D, TileContainer2D as _}, procgen_collapse::{option::PerOptionData2D, tile::CollapsibleTile2D}};
use grid_forge_core::procgen_collapse::queue::propagator::PropagateItem;
use std::collections::{HashMap, HashSet};
// use crate::procgen_collapse::tile::CommonCollapsibleTile2D;

grid_forge_core::__impl_collapsible_grid! {
    trait_name: CommonCollapsibleGrid2D,
    collapsible_data: CollapsibleTile2D,
    grid: GridMap2D,
    position: GridPosition2D,
    direction: Direction2D,
    per_option_data: PerOptionData2D,
}


// pub (crate) mod private {
//     pub trait Sealed {}
// }