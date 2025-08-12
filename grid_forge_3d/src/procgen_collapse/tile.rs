use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;
use std::marker::PhantomData;

use rand::Rng;

use grid_forge_core::id::{IdDefault, IdentTileBuilder, TypeIdMap, TypedData};
use grid_forge_core::procgen_collapse::data::CollapsedTileData;
use grid_forge_core::procgen_collapse::error::CollapseErrorKind;

use crate::procgen_collapse::{DebugSubscriber3D, HistoryItem3D, HistorySubscriber3D};
use crate::{
    core::{
        Direction3D, DirectionTable3D, Grid3D, GridMap3D, GridPosition3D, GridSize3D,
        TileContainer3D,
    },
    procgen_collapse::{
        data::CollapsibleData3D,
        option::{AdjacencyTable3D, PerOptionData3D},
        queue::{EntrophyQueue3D, PositionQueue3D, PropagateItem3D, Propagator3D},
        CollapseError3D, CollapsedGrid3D, CollapsibleGridError3D,
    },
};

grid_forge_core::__impl_collapsible_grid! {
    struct_name: CollapsibleTileGrid3D,
    collapsible_data: CollapsibleData3D,
    collapsed_grid: CollapsedGrid3D,
    propagate_item: PropagateItem3D,
    frequency_hints: FrequencyHints3D,
    adjacency_rules: TileAdjacencyRules3D,
    grid: GridMap3D,
    grid_size: GridSize3D,
    position: GridPosition3D,
    direction: Direction3D,
    per_option_data: PerOptionData3D,
    collapse_error: CollapseError3D,
    collapsible_error: CollapsibleGridError3D,
}

grid_forge_core::__impl_singular_adjacency_rules! {
    struct_name: TileAdjacencyRules3D,
    adjacency_table: AdjacencyTable3D,
    adjacencies: Adjacencies3D,
    direction: Direction3D,
}

grid_forge_core::__impl_singular_identity_analyzer! {
    struct_name: TileIdentityAnalyzer3D,
    adjacency_rules: TileAdjacencyRules3D,
    grid: Grid3D,
    position: GridPosition3D,
    direction: Direction3D,
}

grid_forge_core::__impl_singular_border_analyzer! {
    struct_name: TileBorderAnalyzer3D,
    adjacency_rules: TileAdjacencyRules3D,
    direction_table: DirectionTable3D,
    grid: Grid3D,
    position: GridPosition3D,
    direction: Direction3D,
}

grid_forge_core::__impl_singular_resolver! {
    struct_name: TileResolver3D,
    subscriber_trait: TileSubscriber3D,
    collapsible_data: CollapsibleData3D,
    collapsible_grid: CollapsibleTileGrid3D,
    propagate_item: PropagateItem3D,
    propagator: Propagator3D,
    entrophy_queue: EntrophyQueue3D,
    position_queue: PositionQueue3D,
    collapse_error: CollapseError3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_singular_subscriber_trait! {
    trait_name: TileSubscriber3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_singular_debug_subscriber! {
    struct_name: DebugSubscriber3D,
    trait_name: TileSubscriber3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_singular_collapse_history_subscriber! {
    struct_name: HistorySubscriber3D,
    history_item_name: HistoryItem3D,
    trait_name: TileSubscriber3D,
    position: GridPosition3D,
}

grid_forge_core::__impl_singular_frequency_hints! {
    struct_name: FrequencyHints3D,
    grid: Grid3D,
}
