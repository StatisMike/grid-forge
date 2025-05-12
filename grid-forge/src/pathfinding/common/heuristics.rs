use crate::{common::Dimensionality, utils::OrderedFloat};

pub trait DistanceHeuristic<D: Dimensionality> {
    fn distance(a: &D::Pos, b: &D::Pos) -> OrderedFloat;
}