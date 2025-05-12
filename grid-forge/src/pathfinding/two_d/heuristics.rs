use crate::{common::GridPosition, pathfinding::common::heuristics::DistanceHeuristic, two_d::{GridPosition2D, TwoDim}, utils::OrderedFloat};

pub struct ManhattanHeuristic2D {}

impl DistanceHeuristic<TwoDim> for ManhattanHeuristic2D {
    fn distance(a: &GridPosition2D, b: &GridPosition2D) -> OrderedFloat {
        let [x1, y1] = a.coords();
        let [x2, y2] = b.coords();
        ((x1.abs_diff(x2) + y1.abs_diff(y2)) as f32).into() 
    }
}

pub struct EuclideanCalculator2D {}

impl DistanceHeuristic<TwoDim> for EuclideanCalculator2D {
    fn distance(a: &GridPosition2D, b: &GridPosition2D) -> OrderedFloat {
        let [x1, y1] = a.coords();
        let [x2, y2] = b.coords();
        ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).into() 
    }
}

pub struct ChebyshevCalculator2D {}

impl DistanceHeuristic<TwoDim> for ChebyshevCalculator2D {
    fn distance(a: &GridPosition2D, b: &GridPosition2D) -> OrderedFloat {
        let [x1, y1] = a.coords();
        let [x2, y2] = b.coords();
        (x1.abs_diff(x2).max(y1.abs_diff(y2)) as f32).into()
    }
}