use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};

use grid_forge_core::utils::OrderedFloat;

use crate::core::{Direction2D, Grid2D, GridPosition2D, TileContainer2D as _};
use crate::procgen_collapse::data::CollapsibleData2D;
use crate::procgen_collapse::option::PerOptionData2D;

grid_forge_core::__impl_entrophy_item! {
    struct_name: EntrophyItem2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_entrophy_queue! {
    struct_name: EntrophyQueue2D,
    entrophy_item: EntrophyItem2D,
    per_option_data: PerOptionData2D,
    grid: Grid2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_propagate_item! {
    struct_name: PropagateItem2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_propagator! {
    struct_name: Propagator2D,
    propagate_item: PropagateItem2D,
    collapsible_tile_data: CollapsibleData2D,
    per_option_data: PerOptionData2D,
    entrophy_queue: EntrophyQueue2D,
    direction: Direction2D,
    position: GridPosition2D,
    grid: Grid2D,
}

grid_forge_core::__impl_position_queue! {
    struct_name: PositionQueue2D,
    per_option_data: PerOptionData2D,
    queue_starting_point: CollapseStartingPoint2D,
    queue_direction: CollapseDirection2D,
    queue_ordering: CollapseOrdering2D,
    position: GridPosition2D,
    grid: Grid2D,
}

/// Enum defining the starting point of the collapse queue in the 2D grid.
#[derive(Default, Eq, PartialEq)]
pub enum CollapseStartingPoint2D {
    #[default]
    /// Starts at the `(0, 0)` position.
    UpLeft,
    /// Starts at the `(0, max)` position.
    UpRight,
    /// Starts at the `(max, 0)` position.
    DownLeft,
    /// Starts at the `(max, max)` position.
    DownRight,
}

/// Enum defining the direction in which the tiles will be collapsed in the 2D grid.
#[derive(Default, Eq, PartialEq)]
pub enum CollapseDirection2D {
    #[default]
    /// Collapses tiles in a rowwise fashion.
    Rowwise,
    /// Collapses tiles in a columnwise fashion.
    Columnwise,
}

pub struct CollapseOrdering2D;

impl CollapseOrdering2D {
    fn cmp_fun_default() -> fn(&GridPosition2D, &GridPosition2D) -> Ordering {
        CollapseOrdering2D::cmp_fun(
            CollapseStartingPoint2D::UpLeft,
            CollapseDirection2D::Rowwise,
        )
    }

    fn cmp_fun(
        point: CollapseStartingPoint2D,
        direction: CollapseDirection2D,
    ) -> fn(&GridPosition2D, &GridPosition2D) -> Ordering {
        match (point, direction) {
            (CollapseStartingPoint2D::UpLeft, CollapseDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| a.x().cmp(&b.x()))
                }
            }
            (CollapseStartingPoint2D::UpLeft, CollapseDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| a.y().cmp(&b.y()))
                }
            }
            (CollapseStartingPoint2D::UpRight, CollapseDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| a.y().cmp(&b.y()))
                }
            }
            (CollapseStartingPoint2D::UpRight, CollapseDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| b.x().cmp(&a.x()))
                }
            }
            (CollapseStartingPoint2D::DownLeft, CollapseDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| b.y().cmp(&a.y()).reverse())
                }
            }
            (CollapseStartingPoint2D::DownLeft, CollapseDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).reverse().then_with(|| b.x().cmp(&a.x()))
                }
            }
            (CollapseStartingPoint2D::DownRight, CollapseDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| b.y().cmp(&a.y()))
                }
            }
            (CollapseStartingPoint2D::DownRight, CollapseDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y()
                        .cmp(&b.y())
                        .reverse()
                        .then_with(|| a.x().cmp(&b.x()).reverse())
                }
            }
        }
    }
}
