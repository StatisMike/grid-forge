use grid_forge_core::id::BasicTypedData;

use super::*;
use crate::prelude::*;

#[test]
fn test_identity_analyzer_simple() {

    let mut grid = GridMap2D::new(GridSize2D::new(3,3));

    let data_outer = BasicTypedData(1);
    let data_inner = BasicTypedData(2);

    for position in grid.size().get_all_possible_positions() {
        let data = if position.coords() == [1, 1] { data_inner.clone() } else { data_outer.clone() }; 
        grid.insert_data(&position, data);
    }

    let mut analyzer = singular::SingularIdentityAnalyzer2D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // Inner tile should cannot be possible to be adjacent to itself but to outer tile.
    for direction in Direction2D::ALL {
        assert!(!rules.check_adjacency(&data_inner, &data_inner, direction)); 
        assert!(rules.check_adjacency(&data_inner, &data_outer, direction));
    }

    // Outer tile should be possible to be adjacent to both itself and inner tile.
    for direction in Direction2D::ALL {
        assert!(rules.check_adjacency(&data_outer, &data_inner, direction));
        assert!(rules.check_adjacency(&data_outer, &data_outer, direction));
    }
    
}

#[test]
fn test_identity_analyzer_diagonal_separation() {
    let mut grid = GridMap2D::new(GridSize2D::new(3,3));

    let data_a = BasicTypedData(1);
    let data_b = BasicTypedData(2);

    // Set up the diagonal pattern
    for position in grid.size().get_all_possible_positions() {
        let [x, y] = position.coords();
        let data = if x == 2 || y == 2 { data_b.clone() } else { data_a.clone() };
        grid.insert_data(&position, data);
    }

    let mut analyzer = singular::SingularIdentityAnalyzer2D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();
 
    for direction in Direction2D::ALL {
        assert!(rules.check_adjacency(&data_a, &data_a, direction));
        assert!(rules.check_adjacency(&data_b, &data_b, direction));
    }

    for direction in [Direction2D::Left, Direction2D::Up] {
        assert!(rules.check_adjacency(&data_a, &data_a, direction));
        assert!(!rules.check_adjacency(&data_a, &data_b, direction));
        assert!(!rules.check_adjacency(&data_b, &data_a, direction.opposite()));
        assert!(rules.check_adjacency(&data_b, &data_b, direction));
    }
}

#[test]
fn test_identity_analyzer_checkerboard() {
    let mut grid = GridMap2D::new(GridSize2D::new(3,3));

    let data_a = BasicTypedData(1);
    let data_b = BasicTypedData(2);

    // Set up checkerboard pattern
    for position in grid.size().get_all_possible_positions() {
        let [x, y] = position.coords();
        let data = if (x + y) % 2 == 0 { data_a.clone() } else { data_b.clone() };
        grid.insert_data(&position, data);
    }

    let mut analyzer = singular::SingularIdentityAnalyzer2D::default();
    analyzer.analyze(&grid);
    let rules = analyzer.adjacency_rules();

    // All adjacencies should be only between different tiles
    for direction in Direction2D::ALL {
        assert!(!rules.check_adjacency(&data_a, &data_a, direction));
        assert!(rules.check_adjacency(&data_a, &data_b, direction));
        assert!(rules.check_adjacency(&data_b, &data_a, direction));
        assert!(!rules.check_adjacency(&data_b, &data_b, direction));
    }
}