use crate::{common::GridPosition as _, pathfinding::common::heuristics::DistanceHeuristic, three_d::{GridPosition3D, ThreeDim}, utils::OrderedFloat};

pub struct ManhattanHeuristic3D {}

impl DistanceHeuristic<ThreeDim> for ManhattanHeuristic3D {
    fn distance(a: &GridPosition3D, b: &GridPosition3D) -> OrderedFloat {
        let [x1, y1, z1] = a.coords();
        let [x2, y2, z2] = b.coords();
        ((x1.abs_diff(x2) + y1.abs_diff(y2) + z1.abs_diff(z2)) as f32).into()
    }
}

pub struct EuclideanCalculator3D {}

impl DistanceHeuristic<ThreeDim> for EuclideanCalculator3D {
    fn distance(a: &GridPosition3D, b: &GridPosition3D) -> OrderedFloat {
        let [x1, y1, z1] = a.coords();
        let [x2, y2, z2] = b.coords();
        ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)) as f32).into()
    }
}

pub struct ChebyshevCalculator3D {}

impl DistanceHeuristic<ThreeDim> for ChebyshevCalculator3D {
    fn distance(a: &GridPosition3D, b: &GridPosition3D) -> OrderedFloat {
        let [x1, y1, z1] = a.coords();
        let [x2, y2, z2] = b.coords();
        (x1.abs_diff(x2).max(y1.abs_diff(y2)).max(z1.abs_diff(z2)) as f32).into()
    }
}