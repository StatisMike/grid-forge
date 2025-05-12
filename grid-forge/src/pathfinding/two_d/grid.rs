use crate::define_pathfinder_grid;

define_pathfinder_grid!{
    pub struct Pathfinder2D

    Dimension = {
        dimension_module: two_d,
        dimension_name: TwoDim,
        GridMap: GridMap2D,
        Position: GridPosition2D,
        Size: GridSize2D,
        PathfinderTile: PathfinderTile2D,
        Error: PathfinderError2D,
        AstarPath: AstarPath2D,
        RangePath: RangeFinderPath2D
    }
}
