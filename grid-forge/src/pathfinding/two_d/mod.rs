pub mod grid;
pub mod heuristics;
pub mod paths;

use crate::{pathfinding::common::paths::{AstarPath, RangeFinderPath}, two_d::TwoDim};

use super::common::{cost::CostType, grid::PathfinderError, tile::PathfinderTile};

pub type PathfinderTile2D<Cost: CostType> = PathfinderTile<TwoDim, Cost>;
pub type AstarPath2D = AstarPath<TwoDim>;
pub type RangeFinderPath2D = RangeFinderPath<TwoDim>;
pub type PathfinderError2D = PathfinderError<TwoDim>;