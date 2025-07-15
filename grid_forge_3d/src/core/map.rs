use super::*;

#[cfg(feature = "2d")]
use grid_forge_2d::core::{GridPosition2D, GridSize2D, Tile2D, GridMap2D, Grid2D as _};

use grid_forge_core::TileData;

grid_forge_core::__impl_grid_trait! {
    grid_map_trait: Grid3D,

    direction: Direction3D,
    direction_table: DirectionTable3D,
    size: GridSize3D,
    position: GridPosition3D,
    tile: Tile3D,
    tile_ref: TileRef3D,
    tile_mut: TileMut3D,
    tile_container_trait: TileContainer3D,
    neighbours_count: 6,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::TileData],
}

grid_forge_core::__impl_grid! {

    pub struct GridMap3D {}

    direction: Direction3D,
    direction_table: DirectionTable3D,
    size: GridSize3D,
    position: GridPosition3D,
    tile: Tile3D,
    tile_ref: TileRef3D,
    tile_mut: TileMut3D,
    tile_container_trait: TileContainer3D,
    neighbours_count: 6,

    grid_map_trait: Grid3D,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::TileData],
}

grid_forge_core::__impl_grid_with_shared! {
    pub struct GridMapShared3D {}

    direction: Direction3D,
    direction_table: DirectionTable3D,
    size: GridSize3D,
    position: GridPosition3D,
    tile: Tile3D,
    tile_ref: TileRef3D,
    tile_mut: TileMut3D,
    tile_container_trait: TileContainer3D,
    neighbours_count: 6,

    grid_map_trait: Grid3D,

    tile_ref_shared: TileRefShared3D,
    tile_mut_shared: TileMutShared3D,

    generic_params: [Data],
    where_clause: [Data: grid_forge_core::id::TypedData],
}

grid_forge_core::__impl_shared_from_regular!(GridMapShared3D, GridMap3D);

#[cfg(feature = "2d")]
impl<Data: TileData> GridMap3D<Data> {
    pub fn insert_layer(&mut self, z: u32, layer: GridMap2D<Data>) {
        use grid_forge_2d::core::TileContainer2D as _;
        let layer_size = GridSize3D::from_2d(1, *layer.size());
        if !layer_size.is_contained_within(&self.size) {
            panic!("layer size is not contained within the grid size");
        }
        layer
            .drain()
            .into_iter()
            .map(|tile| {
                (
                    GridPosition3D::new(tile.grid_position().x(), tile.grid_position().y(), z),
                    tile.into_data(),
                )
            })
            .for_each(|(pos, data)| {
                self.insert_tile(Tile3D::new(pos, data));
            });
    }

    pub fn remove_layer(&mut self, z: u32) -> GridMap2D<Data> {
        let layer_size = GridSize2D::new(self.size.x(), self.size.y());
        let mut out = GridMap2D::new(layer_size);
        let positions = self
            .iter_all_positions()
            .filter(|pos| pos.z() == z)
            .collect::<Vec<_>>();
        for pos in positions {
            let tile: Option<Tile3D<Data>> = self.remove_tile_at_position(&pos);
            if let Some(tile) = tile {
                out.insert_tile(Tile2D::new(
                    GridPosition2D::new(tile.0.x(), tile.0.y()),
                    tile.1,
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::Grid3D as _;

    grid_forge_core::__impl_grid_tests!(
        grid: GridMap3D,
        size: GridSize3D,
        position: GridPosition3D,
        direction: Direction3D,
        direction_table: DirectionTable3D,
        tile_container_trait: TileContainer2D,
        dimension_count: 3,
    );

    #[test]
    fn test_3d_neighbours() {
        test_neigbhours(
            GridSize3D::new(10, 10, 10),
            &[
                NeighbourTestCase::new(
                    GridPosition3D::new(0, 0, 0),
                    Direction3D::Down,
                    Some(GridPosition3D::new(0, 1, 0)),
                ),
                NeighbourTestCase {
                    pos: GridPosition3D::new(0, 0, 0),
                    direction: Direction3D::Left,
                    expected: None,
                },
                NeighbourTestCase {
                    pos: GridPosition3D::new(9, 9, 9),
                    direction: Direction3D::Up,
                    expected: Some(GridPosition3D::new(9, 8, 9)),
                },
                NeighbourTestCase {
                    pos: GridPosition3D::new(9, 9, 9),
                    direction: Direction3D::Right,
                    expected: None,
                },
            ],
        );
    }

    #[test]
    fn test_3d_all_neighbours() {
        test_all_neigbhours(
            GridSize3D::new(10, 10, 10),
            &[
                AllNeighboursTestCase::new(
                    GridPosition3D::new(0, 0, 0),
                    DirectionTable3D::new([
                        None,
                        Some(GridPosition3D::new(0, 1, 0)),
                        None,
                        Some(GridPosition3D::new(1, 0, 0)),
                        None,
                        Some(GridPosition3D::new(0, 0, 1)),
                    ]),
                ),
                AllNeighboursTestCase::new(
                    GridPosition3D::new(5, 5, 5),
                    DirectionTable3D::new([
                        Some(GridPosition3D::new(5, 4, 5)),
                        Some(GridPosition3D::new(5, 6, 5)),
                        Some(GridPosition3D::new(4, 5, 5)),
                        Some(GridPosition3D::new(6, 5, 5)),
                        Some(GridPosition3D::new(5, 5, 4)),
                        Some(GridPosition3D::new(5, 5, 6)),
                    ]),
                ),
                AllNeighboursTestCase::new(
                    GridPosition3D::new(9, 9, 9),
                    DirectionTable3D::new([
                        Some(GridPosition3D::new(9, 8, 9)),
                        None,
                        Some(GridPosition3D::new(8, 9, 9)),
                        None,
                        Some(GridPosition3D::new(9, 9, 8)),
                        None,
                    ]),
                ),
            ],
        );
    }
}
