pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use std::{
        fmt::Debug,
        ops::{Index, IndexMut},
    };

    use crate::core::common::*;

    use super::private;

    pub trait Directions<D: Dimensionality + ?Sized>:
        private::Sealed + Sized + Copy + Clone + Debug
    {
        const N: usize;

        fn all() -> &'static [Self];

        fn march_step(&self, from: &D::Pos, size: &D::Size) -> Option<D::Pos>;

        fn opposite(&self) -> Self;

        fn as_idx(&self) -> usize;
    }

    pub trait DirectionTable<D: Dimensionality, T>:
        private::Sealed + Index<D::Dir> + IndexMut<D::Dir>
    {
        type Inner: AsRef<[T]> + AsMut<[T]>;

        fn new_array(values: Self::Inner) -> Self;
        fn inner(&self) -> &Self::Inner;
    }
}

mod private {
    pub trait Sealed {}
}
