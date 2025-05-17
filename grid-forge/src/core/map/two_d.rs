use crate::two_d::*;

crate::core::map::macros::__impl_grid! {

    pub struct GridMap2D;

    module: two_d,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    size: GridSize2D,
    position: GridPosition2D,
    tile: Tile2D,
    tile_ref: TileRef2D,
    tile_mut: TileMut2D,
    neighbours_count: 4,
}

#[cfg(test)]
mod tests {
    use crate::two_d::*;

    crate::core::map::macros::__impl_grid_tests!(
        grid: GridMap2D,
        size: GridSize2D,
        position: GridPosition2D,
        direction: Direction2D,
        direction_table: DirectionTable2D,
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
