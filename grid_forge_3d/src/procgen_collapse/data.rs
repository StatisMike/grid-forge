use crate::procgen_collapse::option::{PerOptionData3D, WaysToBeOption3D};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use crate::core::GridPosition3D;

use grid_forge_core::TileData;

use grid_forge_core::procgen_collapse::option::OptionWeights;

grid_forge_core::__impl_collapsible_tile_data! {
    struct_name: CollapsibleData3D,
    position: GridPosition3D,
    ways_to_be_option: WaysToBeOption3D,
    per_option_data: PerOptionData3D,
}
