use crate::{common::{Dimensionality, GridPosition}, three_d::{GridPosition3D, ThreeDim}, two_d::{GridPosition2D, TwoDim}, utils::OrderedFloat};

pub trait DistanceHeuristic<D: Dimensionality> {
    fn distance(a: &D::Pos, b: &D::Pos) -> OrderedFloat;
}

pub struct ManhattanHeuristic<D: Dimensionality> {
    _marker: std::marker::PhantomData<D>,
}

impl DistanceHeuristic<TwoDim> for ManhattanHeuristic<TwoDim> {
    fn distance(a: &GridPosition2D, b: &GridPosition2D) -> OrderedFloat {
        let [x1, y1] = a.coords();
        let [x2, y2] = b.coords();
        ((x1.abs_diff(x2) + y1.abs_diff(y2)) as f32).into() 
    }
}

impl DistanceHeuristic<ThreeDim> for ManhattanHeuristic<ThreeDim> {
    fn distance(a: &GridPosition3D, b: &GridPosition3D) -> OrderedFloat {
        let [x1, y1, z1] = a.coords();
        let [x2, y2, z2] = b.coords();
        ((x1.abs_diff(x2) + y1.abs_diff(y2)+ z1.abs_diff(z2)) as f32).into()  
    }
}

pub struct EuclideanHeuristic<D: Dimensionality> {
    _marker: std::marker::PhantomData<D>,
}



// pub struct EuclideanCalculator<T: Pos> {
//     _marker: std::marker::PhantomData<T>,
// }

// impl DistanceCalculator<Pos2D> for EuclideanCalculator<Pos2D> {
//     fn distance(a: &Pos2D, b: &Pos2D) -> f32 {
//         let (x1, y1) = a;
//         let (x2, y2) = b;
//         ((x1 - x2).pow(2) + (y1 - y2).pow(2)).isqrt() as f32
//     }
// }

// impl DistanceCalculator<Pos3D> for EuclideanCalculator<Pos3D> {
//     fn distance(a: &Pos3D, b: &Pos3D) -> f32 {
//         let (x1, y1, z1) = a;
//         let (x2, y2, z2) = b;
//         ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt() as f32
//     }
// }

// pub struct ChebyshevCalculator<T: Pos> {
//     _marker: std::marker::PhantomData<T>,
// }

// impl DistanceCalculator<Pos2D> for ChebyshevCalculator<Pos2D> {
//     fn distance(a: &Pos2D, b: &Pos2D) -> f32 {
//         let (x1, y1) = a;
//         let (x2, y2) = b;
//         ((x1 - x2).abs().max((y1 - y2).abs())) as f32
//     }
// }

// impl DistanceCalculator<Pos3D> for ChebyshevCalculator<Pos3D> {
//     fn distance(a: &Pos3D, b: &Pos3D) -> f32 {
//         let (x1, y1, z1) = a;
//         let (x2, y2, z2) = b;
//         let dx = (x1 - x2).abs();
//         let dy = (y1 - y2).abs();
//         let dz = (z1 - z2).abs();
//         (dx.max(dy).max(dz)) as f32
//     }
// }