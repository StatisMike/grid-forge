#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::procgen_collapse::{
    option::{Adjacencies2D, AdjacencyTable2D, PerOptionData2D, WaysToBeOption2D},
    queue::{CollapseDirection2D, CollapseOrdering2D, CollapseStartingPoint2D, EntrophyQueue2D, PositionQueue2D},
    data::{CollapsibleData2D},
    CollapseError2D,
    CollapsibleGridError2D,
    CollapsedGrid2D,
};

pub mod singular {

    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::{
        DebugSubscriber2D, CollapseError2D, CollapsibleGridError2D
    };

    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::tile::{
        TileAdjacencyRules2D, TileBorderAnalyzer2D, SingularIdentityAnalyzer2D, TileResolver2D,
        TileSubscriber2D, FrequencyHints2D, 
        CollapsibleTileGrid2D,
    };

    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::pattern::{
        Pattern2DAnalyzer, Pattern2DResolver, Pattern2DSubscriber,
        Pattern2D, CollapsiblePattern2DGrid, Pattern2DFrequencyHints,
        Pattern2DAdjacencyRules,
    };
}
