use std::collections::hash_map::Entry;
use std::collections::BTreeMap;

use grid_forge_core::id::{TypeIdMap, TypeIdSet};
use grid_forge_core::procgen_collapse::option::OptionWeights;

use crate::core::{Direction3D, DirectionTable3D};

grid_forge_core::__impl_collapse_adjacencies! {
    struct_name: Adjacencies3D,
    direction: Direction3D,
    direction_table: DirectionTable3D,
    direction_count: 6,
}

grid_forge_core::__impl_adjacency_table! {
    struct_name: AdjacencyTable3D,
    adjacencies: Adjacencies3D,
    direction: Direction3D,
}

grid_forge_core::__impl_ways_to_be_option! {
    struct_name: WaysToBeOption3D,
    direction: Direction3D,
    direction_table: DirectionTable3D,
    direction_count: 6,
}

grid_forge_core::__impl_per_option_data! {
    struct_name: PerOptionData3D,
    direction: Direction3D,
    direction_table: DirectionTable3D,
    adjacency_table: AdjacencyTable3D,
    ways_to_be_option: WaysToBeOption3D,
}
