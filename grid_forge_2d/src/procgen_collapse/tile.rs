use crate::{procgen_collapse::option::{PerOptionData2D, WaysToBeOption2D}};
use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::Uniform;

use crate::core::{GridPosition2D};

use grid_forge_core::TileData;

use grid_forge_core::procgen_collapse::option::OptionWeights;

grid_forge_core::__impl_collapsible_tile_data! {
    struct_name: CollapsibleTile2D,
    position: GridPosition2D,
    ways_to_be_option: WaysToBeOption2D,
    per_option_data: PerOptionData2D,
}
