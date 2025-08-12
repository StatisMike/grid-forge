use crate::core::*;

grid_forge_core::__impl_grid_trait! {
    grid_map_trait: Grid2D,

    direction: Direction2D,
    direction_table: DirectionTable2D,
    size: GridSize2D,
    position: GridPosition2D,
    tile: Tile2D,
    tile_ref: TileRef2D,
    tile_mut: TileMut2D,
    tile_container_trait: TileContainer2D,
    neighbours_count: 4,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::TileData],
}

grid_forge_core::__impl_grid_shared_trait! {
    grid_shared_trait: GridShared2D,
    grid_map_trait: Grid2D,

    position: GridPosition2D,
    tile_ref_shared: TileRefShared2D,
    tile_mut_shared: TileMutShared2D,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::id::TypedData],
}

grid_forge_core::__impl_grid! {

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub struct GridMap2D {}

    direction: Direction2D,
    direction_table: DirectionTable2D,
    size: GridSize2D,
    position: GridPosition2D,
    tile: Tile2D,
    tile_ref: TileRef2D,
    tile_mut: TileMut2D,
    tile_container_trait: TileContainer2D,
    neighbours_count: 4,

    grid_map_trait: Grid2D,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::TileData],
}

grid_forge_core::__impl_grid_with_shared! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub struct GridMapShared2D {}

    direction: Direction2D,
    direction_table: DirectionTable2D,
    size: GridSize2D,
    position: GridPosition2D,
    tile: Tile2D,
    tile_ref: TileRef2D,
    tile_mut: TileMut2D,
    tile_container_trait: TileContainer2D,
    neighbours_count: 4,

    grid_map_trait: Grid2D,
    grid_shared_trait: GridShared2D,

    tile_ref_shared: TileRefShared2D,
    tile_mut_shared: TileMutShared2D,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::id::TypedData],
}

grid_forge_core::__impl_shared_from_regular!(GridMapShared2D, GridMap2D);

#[cfg(test)]
mod tests {
    mod regular {
        use super::super::Grid2D as _;
        use crate::core::*;

        grid_forge_core::__impl_grid_tests!(
            grid: GridMap2D,
            size: GridSize2D,
            position: GridPosition2D,
            direction: Direction2D,
            direction_table: DirectionTable2D,
            tile_container_trait: TileContainer2D,
            dimension_count: 2,
        );

        #[test]
        fn test_2_neighbours() {
            test_neigbhours(
                GridSize2D::new(10, 10),
                &[
                    NeighbourTestCase::new(
                        GridPosition2D::new(0, 0),
                        Direction2D::Down,
                        Some(GridPosition2D::new(0, 1)),
                    ),
                    NeighbourTestCase::new(GridPosition2D::new(0, 0), Direction2D::Left, None),
                    NeighbourTestCase::new(
                        GridPosition2D::new(9, 9),
                        Direction2D::Up,
                        Some(GridPosition2D::new(9, 8)),
                    ),
                    NeighbourTestCase::new(GridPosition2D::new(9, 9), Direction2D::Right, None),
                ],
            );
        }

        #[test]
        fn test_2_all_neighbours() {
            test_all_neigbhours(
                GridSize2D::new(10, 10),
                &[
                    AllNeighboursTestCase::new(
                        GridPosition2D::new(0, 0),
                        DirectionTable2D::new([
                            None,
                            Some(GridPosition2D::new(0, 1)),
                            None,
                            Some(GridPosition2D::new(1, 0)),
                        ]),
                    ),
                    AllNeighboursTestCase::new(
                        GridPosition2D::new(5, 5),
                        DirectionTable2D::new([
                            Some(GridPosition2D::new(5, 4)),
                            Some(GridPosition2D::new(5, 6)),
                            Some(GridPosition2D::new(4, 5)),
                            Some(GridPosition2D::new(6, 5)),
                        ]),
                    ),
                    AllNeighboursTestCase::new(
                        GridPosition2D::new(9, 9),
                        DirectionTable2D::new([
                            Some(GridPosition2D::new(9, 8)),
                            None,
                            Some(GridPosition2D::new(8, 9)),
                            None,
                        ]),
                    ),
                ],
            );
        }
    }

    mod shared {
        use super::super::Grid2D as _;
        use super::super::GridShared2D as _;
        use crate::core::*;

        grid_forge_core::__impl_shared_grid_tests!(
            grid: GridMapShared2D,
            size: GridSize2D,
            position: GridPosition2D,
            direction: Direction2D,
            direction_table: DirectionTable2D,
            tile_container_trait: TileContainer2D,
            dimension_count: 2,
        );
    }
}
