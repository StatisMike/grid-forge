use std::any::Any;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::io::Write;
use std::marker::PhantomData;

use rand::Rng;

use grid_forge_core::id::{IdDefault, TypedData, IdentTileBuilder, TypeIdMap};
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::error::CollapseErrorKind;

use crate::procgen_collapse::{HistoryItem2D, HistorySubscriber2D, DebugSubscriber2D};
use crate::{
    core::{Direction2D, DirectionTable2D, Grid2D, GridPosition2D, GridSize2D, GridMap2D, TileContainer2D},
    procgen_collapse::{
        option::{AdjacencyTable2D, PerOptionData2D},
        queue::{EntrophyQueue2D, PositionQueue2D, PropagateItem2D, Propagator2D},
        data::CollapsibleData2D,
        CollapseError2D, CollapsedGrid2D, CollapsibleGridError2D,
    },
};


grid_forge_core::__impl_collapsible_grid! {
    struct_name: CollapsibleTileGrid2D,
    collapsible_data: CollapsibleData2D,
    collapsed_grid: CollapsedGrid2D,
    propagate_item: PropagateItem2D,
    frequency_hints: FrequencyHints2D,
    adjacency_rules: TileAdjacencyRules2D,
    grid: GridMap2D,
    grid_size: GridSize2D,
    position: GridPosition2D,
    direction: Direction2D,
    per_option_data: PerOptionData2D,
    collapse_error: CollapseError2D,
    collapsible_error: CollapsibleGridError2D,
}

grid_forge_core::__impl_singular_adjacency_rules! {
    struct_name: TileAdjacencyRules2D,
    adjacency_table: AdjacencyTable2D,
    adjacencies: Adjacencies2D,
    direction: Direction2D,
}

grid_forge_core::__impl_singular_identity_analyzer! {
    struct_name: SingularIdentityAnalyzer2D,
    adjacency_rules: TileAdjacencyRules2D,
    grid: Grid2D,
    position: GridPosition2D,
    direction: Direction2D,
}

grid_forge_core::__impl_singular_border_analyzer! {
    struct_name: TileBorderAnalyzer2D,
    adjacency_rules: TileAdjacencyRules2D,
    direction_table: DirectionTable2D,
    grid: Grid2D,
    position: GridPosition2D,
    direction: Direction2D,
}

grid_forge_core::__impl_singular_resolver! {
    struct_name: TileResolver2D,
    subscriber_trait: TileSubscriber2D,
    collapsible_data: CollapsibleData2D,
    collapsible_grid: CollapsibleTileGrid2D,
    propagate_item: PropagateItem2D,
    propagator: Propagator2D,
    entrophy_queue: EntrophyQueue2D,
    position_queue: PositionQueue2D,
    collapse_error: CollapseError2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_singular_subscriber_trait! {
    trait_name: TileSubscriber2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_singular_debug_subscriber! {
    struct_name: DebugSubscriber2D,
    trait_name: TileSubscriber2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_singular_collapse_history_subscriber! {
    struct_name: HistorySubscriber2D,
    history_item_name: HistoryItem2D,
    trait_name: TileSubscriber2D,
    position: GridPosition2D,
}

grid_forge_core::__impl_singular_frequency_hints! {
    struct_name: FrequencyHints2D,
    grid: Grid2D,
}