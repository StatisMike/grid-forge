#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::procgen_collapse::{
    option::{Adjacencies2D, AdjacencyTable2D, PerOptionData2D, WaysToBeOption2D},
    queue::{CollapseDirection2D, CollapseOrdering2D, CollapseStartingPoint2D, EntrophyQueue2D, PositionQueue2D},
    tile::{CollapsibleTile2D},
    CollapseError2D,
    CollapsibleGridError2D,
    CollapsedGrid2D,
};

pub mod singular {
    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::singular::{
        SingularAdjacencyRules2D, SingularBorderAnalyzer2D, SingularIdentityAnalyzer2D, SingularResolver2D,
        SingularSubscriber2D, DebugSubscriber, FrequencyHints2D, 
        CollapsibleTileGrid2D,
    };
}
