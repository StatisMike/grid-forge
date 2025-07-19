use crate::core::{Grid2D, GridPosition2D};
use crate::procgen_collapse::option::PerOptionData2D;
use grid_forge_core::utils::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use crate::procgen_collapse::tile::CollapsibleTile2D;
use std::marker::PhantomData;
use rand::Rng;

grid_forge_core::__impl_entrophy_item! {
    struct_name: EntrophyItem2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_entrophy_queue! {
    struct_name: EntrophyQueue2D,
    entrophy_item: EntrophyItem2D,
    collapsible_tile_data: CollapsibleTile2D,
    per_option_data: PerOptionData2D,
    grid: Grid2D,
    position: GridPosition2D,
}