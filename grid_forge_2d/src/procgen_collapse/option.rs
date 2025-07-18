use crate::core::{Direction2D, DirectionTable2D};
use grid_forge_core::procgen_collapse::option::OptionWeights;
use std::collections::hash_map::Entry;
use std::collections::BTreeMap;
use grid_forge_core::id::TypeIdMap;
use grid_forge_core::id::TypeIdSet;

grid_forge_core::__impl_collapse_adjacencies! {
    struct_name: Adjacencies2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    direction_count: 4,
}

grid_forge_core::__impl_adjacency_table!{
    struct_name: AdjacencyTable2D,
    adjacencies: Adjacencies2D,
    direction: Direction2D,
}

grid_forge_core::__impl_ways_to_be_option!{
    struct_name: WaysToBeOption2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    direction_count: 4,
}

grid_forge_core::__impl_per_option_data!{
    struct_name: PerOptionData2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    adjacency_table: AdjacencyTable2D,
    ways_to_be_option: WaysToBeOption2D,
}