#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::procgen_collapse::{
    data::CollapsibleData2D,
    option::{Adjacencies2D, AdjacencyTable2D, PerOptionData2D, WaysToBeOption2D},
    queue::{
        CollapseDirection2D, CollapseOrdering2D, CollapseStartingPoint2D, EntrophyQueue2D,
        PositionQueue2D,
    },
    CollapseError2D, CollapsedGrid2D, CollapsibleGridError2D, DebugSubscriber2D, HistoryItem2D, HistorySubscriber2D,
};

pub mod tile {

    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::tile::{
        CollapsibleTileGrid2D, FrequencyHints2D, TileIdentityAnalyzer2D, TileAdjacencyRules2D,
        TileBorderAnalyzer2D, TileResolver2D, TileSubscriber2D,
    };

    #[cfg(feature = "3d")]
    #[doc(inline)]
    pub use grid_forge_3d::procgen_collapse::tile::{
        CollapsibleTileGrid3D, FrequencyHints3D, TileIdentityAnalyzer3D, TileAdjacencyRules3D,
        TileBorderAnalyzer3D, TileResolver3D, TileSubscriber3D,
    };

}

pub mod pattern {
    #[cfg(feature = "2d")]
    #[doc(inline)]
    pub use grid_forge_2d::procgen_collapse::pattern::{
        CollapsiblePatternGrid2D, Pattern2DGrid, Pattern2D, Pattern2DAdjacencyRules, Pattern2DAnalyzer,
        Pattern2DFrequencyHints, Pattern2DResolver, Pattern2DSubscriber,
    };

    #[cfg(feature = "3d")]
    #[doc(inline)]
    pub use grid_forge_3d::procgen_collapse::pattern::{
        CollapsiblePatternGrid3D, Pattern3DGrid, Pattern3D, Pattern3DAdjacencyRules, Pattern3DAnalyzer,
        Pattern3DFrequencyHints, Pattern3DResolver, Pattern3DSubscriber,
    };
}
