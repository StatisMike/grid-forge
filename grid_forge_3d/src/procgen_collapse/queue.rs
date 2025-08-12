use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};

use grid_forge_core::utils::OrderedFloat;

use crate::core::{Direction3D, Grid3D, GridPosition3D, TileContainer3D as _};
use crate::procgen_collapse::data::CollapsibleData3D;
use crate::procgen_collapse::option::PerOptionData3D;

grid_forge_core::__impl_entrophy_item! {
    struct_name: EntrophyItem3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_entrophy_queue! {
    struct_name: EntrophyQueue3D,
    entrophy_item: EntrophyItem3D,
    per_option_data: PerOptionData3D,
    grid: Grid3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_propagate_item! {
    struct_name: PropagateItem3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_propagator! {
    struct_name: Propagator3D,
    propagate_item: PropagateItem3D,
    collapsible_tile_data: CollapsibleData3D,
    per_option_data: PerOptionData3D,
    entrophy_queue: EntrophyQueue3D,
    direction: Direction3D,
    position: GridPosition3D,
    grid: Grid3D,
}

grid_forge_core::__impl_position_queue! {
    struct_name: PositionQueue3D,
    per_option_data: PerOptionData3D,
    queue_starting_point: CollapseStartingPoint3D,
    queue_direction: CollapseDirection3D,
    queue_ordering: CollapseOrdering3D,
    position: GridPosition3D,
    grid: Grid3D,
}

/// Enum defining the starting point of the collapse queue in the 3D grid.
#[derive(Default, Eq, PartialEq)]
pub enum CollapseStartingPoint3D {
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

/// Enum defining the direction in which the tiles will be collapsed in the 3D grid.
#[derive(Default, Eq, PartialEq)]
pub enum CollapseDirection3D {
    #[default]
    /// Collapses tiles in a rowwise fashion.
    Rowwise,
    /// Collapses tiles in a columnwise fashion.
    Columnwise,
}

pub struct CollapseOrdering3D;

impl CollapseOrdering3D {
    fn cmp_fun_default() -> fn(&GridPosition3D, &GridPosition3D) -> Ordering {
        CollapseOrdering3D::cmp_fun(
            CollapseStartingPoint3D::UpLeft,
            CollapseDirection3D::Rowwise,
        )
    }

    fn cmp_fun(
        point: CollapseStartingPoint3D,
        direction: CollapseDirection3D,
    ) -> fn(&GridPosition3D, &GridPosition3D) -> Ordering {
        match (point, direction) {
            (CollapseStartingPoint3D::UpLeft, CollapseDirection3D::Rowwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| a.x().cmp(&b.x()))
                }
            }
            (CollapseStartingPoint3D::UpLeft, CollapseDirection3D::Columnwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| a.y().cmp(&b.y()))
                }
            }
            (CollapseStartingPoint3D::UpRight, CollapseDirection3D::Columnwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| a.y().cmp(&b.y()))
                }
            }
            (CollapseStartingPoint3D::UpRight, CollapseDirection3D::Rowwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| b.x().cmp(&a.x()))
                }
            }
            (CollapseStartingPoint3D::DownLeft, CollapseDirection3D::Columnwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| b.y().cmp(&a.y()).reverse())
                }
            }
            (CollapseStartingPoint3D::DownLeft, CollapseDirection3D::Rowwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.y().cmp(&b.y()).reverse().then_with(|| b.x().cmp(&a.x()))
                }
            }
            (CollapseStartingPoint3D::DownRight, CollapseDirection3D::Columnwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| b.y().cmp(&a.y()))
                }
            }
            (CollapseStartingPoint3D::DownRight, CollapseDirection3D::Rowwise) => {
                |a: &GridPosition3D, b: &GridPosition3D| -> Ordering {
                    a.y()
                        .cmp(&b.y())
                        .reverse()
                        .then_with(|| a.x().cmp(&b.x()).reverse())
                }
            }
        }
    }
}
