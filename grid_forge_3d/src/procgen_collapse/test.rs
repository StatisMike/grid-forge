use grid_forge_core::id::BasicTypedData;
use grid_forge_core::procgen_collapse::{data::CollapsedTileData, pattern::PatternTileData};

use crate::{
    core::{Direction3D, Grid3D as _, GridMap3D, GridPosition3D, GridSize3D, Tile3D},
    procgen_collapse::pattern::{CollapsiblePattern3DGrid, Pattern3DAnalyzer},
};

use super::*;
use crate::prelude::*;

#[test]
fn test_identity_analyzer_simple() {
    let mut grid = GridMap3D::new(GridSize3D::new(3, 3, 3));

    let data_outer = BasicTypedData(1);
    let data_inner = BasicTypedData(2);

    for position in grid.size().get_all_possible_positions() {
        let data = if position.coords() == [1, 1, 1] {
            data_inner.clone()
        } else {
            data_outer.clone()
        };
        grid.insert_data(&position, data);
    }

    let mut analyzer = tile::TileIdentityAnalyzer3D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // Inner tile should cannot be possible to be adjacent to itself but to outer tile.
    for direction in Direction3D::ALL {
        assert!(!rules.check_adjacency(&data_inner, &data_inner, direction));
        assert!(rules.check_adjacency(&data_inner, &data_outer, direction));
    }

    // Outer tile should be possible to be adjacent to both itself and inner tile.
    for direction in Direction3D::ALL {
        assert!(rules.check_adjacency(&data_outer, &data_inner, direction));
        assert!(rules.check_adjacency(&data_outer, &data_outer, direction));
    }
}

#[test]
fn test_identity_analyzer_diagonal_separation() {
    let mut grid = GridMap3D::new(GridSize3D::new(3, 3, 3));

    let data_a = BasicTypedData(1);
    let data_b = BasicTypedData(2);

    // Set up the diagonal pattern
    for position in grid.size().get_all_possible_positions() {
        let [x, y, z] = position.coords();
        let data = if x == 2 || y == 2 {
            data_b.clone()
        } else {
            data_a.clone()
        };
        grid.insert_data(&position, data);
    }

    let mut analyzer = tile::TileIdentityAnalyzer3D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    for direction in Direction3D::ALL {
        assert!(rules.check_adjacency(&data_a, &data_a, direction));
        assert!(rules.check_adjacency(&data_b, &data_b, direction));
    }

    for direction in [Direction3D::Left, Direction3D::Up] {
        assert!(rules.check_adjacency(&data_a, &data_a, direction));
        assert!(!rules.check_adjacency(&data_a, &data_b, direction));
        assert!(!rules.check_adjacency(&data_b, &data_a, direction.opposite()));
        assert!(rules.check_adjacency(&data_b, &data_b, direction));
    }
}

#[test]
fn test_identity_analyzer_checkerboard() {
    let mut grid = GridMap3D::new(GridSize3D::new(3, 3, 3));

    let data_a = BasicTypedData(1);
    let data_b = BasicTypedData(2);

    // Set up checkerboard pattern
    for position in grid.size().get_all_possible_positions() {
        let [x, y, z] = position.coords();
        let data = if (x + y) % 2 == 0 {
            data_a.clone()
        } else {
            data_b.clone()
        };
        grid.insert_data(&position, data);
    }

    let mut analyzer = tile::TileIdentityAnalyzer3D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // All adjacencies should be only between different tiles
    for direction in Direction3D::ALL {
        assert!(!rules.check_adjacency(&data_a, &data_a, direction));
        assert!(rules.check_adjacency(&data_a, &data_b, direction));
        assert!(rules.check_adjacency(&data_b, &data_a, direction));
        assert!(!rules.check_adjacency(&data_b, &data_b, direction));
    }
}

#[test]
fn test_border_analyzer_simple() {
    let mut grid = GridMap3D::new(GridSize3D::new(3, 3, 3));

    let data_outer = BasicTypedData(1);
    let data_inner = BasicTypedData(2);

    for position in grid.size().get_all_possible_positions() {
        let data = if position.coords() == [1, 1, 1] {
            data_inner.clone()
        } else {
            data_outer.clone()
        };
        grid.insert_data(&position, data);
    }

    let mut analyzer = tile::TileBorderAnalyzer3D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // Both tiles can be adjacent to each other in any direction (because of border rules inference)
    for direction in Direction3D::ALL {
        assert!(rules.check_adjacency(&data_outer, &data_inner, direction));
        assert!(rules.check_adjacency(&data_inner, &data_outer, direction));
        assert!(rules.check_adjacency(&data_outer, &data_outer, direction));
    }
}

#[test]
fn test_border_analyzer_three_tile() {
    let mut grid = GridMap3D::new(GridSize3D::new(3, 3, 3));

    let data_left = BasicTypedData(1);
    let data_middle = BasicTypedData(2);
    let data_right = BasicTypedData(3);

    for position in grid.size().get_all_possible_positions() {
        let data = match position.x() {
            0 => data_left.clone(),
            1 => data_middle.clone(),
            2 => data_right.clone(),
            _ => unreachable!(),
        };
        grid.insert_data(&position, data);
    }

    let mut analyzer = tile::TileBorderAnalyzer3D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // Every tile can be near itself in UP and DOWN directions.
    for direction in [Direction3D::Up, Direction3D::Down] {
        assert!(rules.check_adjacency(&data_left, &data_left, direction));
        assert!(rules.check_adjacency(&data_middle, &data_middle, direction));
        assert!(rules.check_adjacency(&data_right, &data_right, direction));
    }

    // Every tile CANNOT be near itself in LEFT and RIGHT directions.
    for direction in [Direction3D::Left, Direction3D::Right] {
        assert!(!rules.check_adjacency(&data_left, &data_left, direction));
        assert!(!rules.check_adjacency(&data_middle, &data_middle, direction));
        assert!(!rules.check_adjacency(&data_right, &data_right, direction));
    }

    // Tile A can be adjacent to Tile B only in RIGHT direction.
    assert!(rules.check_adjacency(&data_left, &data_middle, Direction3D::Right));
    assert!(!rules.check_adjacency(&data_left, &data_middle, Direction3D::Left));

    // Tile B can be adjacent to Tile A only in LEFT direction and Tile C in RIGHT direction
    assert!(rules.check_adjacency(&data_middle, &data_left, Direction3D::Left));
    assert!(!rules.check_adjacency(&data_middle, &data_left, Direction3D::Right));
    assert!(rules.check_adjacency(&data_middle, &data_right, Direction3D::Right));
    assert!(!rules.check_adjacency(&data_middle, &data_right, Direction3D::Left));

    // Tile C can be adjacent to Tile B only in LEFT direction.
    assert!(rules.check_adjacency(&data_right, &data_middle, Direction3D::Left));
    assert!(!rules.check_adjacency(&data_right, &data_middle, Direction3D::Right));

    // Tiles A and C cannot be adjacent to each other in any direction.
    for direction in Direction3D::ALL {
        assert!(!rules.check_adjacency(&data_left, &data_right, direction));
        assert!(!rules.check_adjacency(&data_right, &data_left, direction));
    }
}

// #[test]
// fn correct_adjacency_3D_2x2() {
//     let mut analyzer = Pattern3DAnalyzer::<2, 2, CollapsedTileData>::default();
//     let pattern_grid = analyzer.analyze(&test_grid_3D_2x2());

//     let adjacency_rules = analyzer.get_adjacency();

//     let p0000 = retrieve_pattern(&GridPosition3D::new(0, 0), &pattern_grid);
//     let p0101 = retrieve_pattern(&GridPosition3D::new(1, 0), &pattern_grid);
//     let p1111 = retrieve_pattern(&GridPosition3D::new(2, 0), &pattern_grid);

//     for dir in Direction3D::ALL {
//         assert!(
//             !adjacency_rules.is_valid_at_dir(p0000.1, dir, p1111.1),
//             "patterns are falsely compatible"
//         )
//     }

//     assert!(adjacency_rules.is_valid_at_dir(p0000.1, Direction3D::Right, p0101.1));
//     assert!(adjacency_rules.is_valid_at_dir(p0101.1, Direction3D::Left, p0000.1));
//     assert!(!adjacency_rules.is_valid_at_dir(p0000.1, Direction3D::Up, p0101.1));
//     assert!(!adjacency_rules.is_valid_at_dir(p0000.1, Direction3D::Down, p0101.1));
// }

// #[test]
// fn correct_adjacency_3D_3x3() {
//     let mut analyzer = Pattern3DAnalyzer::<3, 3, CollapsedTileData>::default();
//     let pattern_grid = analyzer.analyze(&test_grid_3D_3x3());
//     let adjacency_rules = analyzer.get_adjacency();

//     // Test some specific pattern combinations
//     // let p000_000_000 = retrieve_pattern(&GridPosition3D::new(0, 0), &pattern_grid);
//     let p111_111_111 = retrieve_pattern(&GridPosition3D::new(3, 0), &pattern_grid);
//     // let p111_111_100 = retrieve_pattern(&GridPosition3D::new(2, 3), &pattern_grid);
//     // let p111_100_010 = retrieve_pattern(&GridPosition3D::new(3, 3), &pattern_grid);
//     // let p100_100_100 = retrieve_pattern(&GridPosition3D::new(0, 3), &pattern_grid);

//     // Test some expected compatible patterns
//     assert!(adjacency_rules.is_valid_at_dir(p111_111_111.1, Direction3D::Down, p111_111_111.1));

//     // assert!(adjacency_rules.is_valid_at_dir(
//     //     p000111.1,
//     //     Direction3D::Down,
//     //     retrieve_pattern(&GridPosition3D::new(1, 2), &pattern_grid).1
//     // ));

//     // // Test some expected incompatible patterns
//     // for dir in Direction3D::ALL {
//     //     assert!(
//     //         !adjacency_rules.is_valid_at_dir(p000000.1, dir, p101010.1),
//     //         "Patterns should be incompatible in all directions"
//     //     );

//     //     assert!(
//     //         !adjacency_rules.is_valid_at_dir(p000111.1, dir, p111000.1),
//     //         "Patterns should be incompatible in all directions"
//     //     );
//     // }

//     // // Test corner cases
//     // assert!(adjacency_rules.is_valid_at_dir(
//     //     retrieve_pattern(&GridPosition3D::new(1, 1), &pattern_grid).1,
//     //     Direction3D::Right,
//     //     retrieve_pattern(&GridPosition3D::new(1, 2), &pattern_grid).1
//     // ));
// }


// /// ```text
// ///     0 1 2 3
// ///     -------
// /// 0 | 0 0 1 1
// /// 1 | 0 0 1 1
// /// 2 | 1 0 1 0
// /// 3 | 1 0 0 1
// /// ```
// fn test_grid_3D_2x2() -> GridMap3D<CollapsedTileData> {
//     let mut map = GridMap3D::new(GridSize3D::new(4, 4));
//     for tile in vec![
//         Tile3D::new(GridPosition3D::new(0, 0), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(0, 1), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(0, 2), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(0, 3), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(1, 0), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 1), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 2), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 3), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 0), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(2, 1), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(2, 2), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(2, 3), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(3, 0), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(3, 1), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(3, 2), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 3), CollapsedTileData::new(1)),
//     ] {
//         map.insert(tile);
//     }
//     map
// }

// /// ```text
// ///     0 1 2 3 4 5
// ///     -----------
// /// 0 | 0 0 0 1 1 1
// /// 1 | 0 0 0 1 1 1
// /// 2 | 0 0 0 1 1 1
// /// 3 | 1 0 0 1 0 0
// /// 4 | 1 0 0 0 1 0
// /// 5 | 1 0 0 0 0 1
// /// ```
// fn test_grid_3D_3x3() -> GridMap3D<CollapsedTileData> {
//     let mut map = GridMap3D::new(GridSize3D::new(6, 6));
//     for tile in vec![
//         Tile3D::new(GridPosition3D::new(0, 0), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 0), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 0), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 0), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(4, 0), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(5, 0), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(0, 1), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 1), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 1), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 1), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(4, 1), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(5, 1), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(0, 2), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(1, 2), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 2), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 2), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(4, 2), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(5, 2), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(0, 3), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(1, 3), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 3), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 3), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(4, 3), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(5, 3), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(0, 4), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(1, 4), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 4), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 4), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(4, 4), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(5, 4), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(0, 5), CollapsedTileData::new(1)),
//         Tile3D::new(GridPosition3D::new(1, 5), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(2, 5), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(3, 5), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(4, 5), CollapsedTileData::new(0)),
//         Tile3D::new(GridPosition3D::new(5, 5), CollapsedTileData::new(1)),
//     ] {
//         map.insert(tile);
//     }
//     map
// }

// fn retrieve_pattern<const SIZE_X: usize, const SIZE_Y: usize>(
//     position: &GridPosition3D,
//     map: &CollapsiblePattern3DGrid<SIZE_X, SIZE_Y>,
// ) -> (u64, u64) {
//     let Some(data) = map.inner().data_at(position) else {
//         panic!("Can't get tile at {position:?}");
//     };
//     let PatternTileData::WithPattern {
//         tile_type_id,
//         pattern_id,
//     } = data
//     else {
//         panic!("Can't get WithPattern tile data at {position:?}");
//     };
//     (*tile_type_id, *pattern_id)
// }