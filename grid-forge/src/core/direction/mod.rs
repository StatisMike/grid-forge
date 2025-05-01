//! Contains the core direction traits and implementations.
//!
//! Their purpose is to allow:
//! - retrieving the neighbouring tiles in a map
//! - iterating over the directions in dimensionality
//! - fast lookup of some data bound to the specific direction.
//!
//! The core direction traits are:
//! - [Direction](crate::core::direction::common::Direction)
//! - [DirectionTable](crate::core::direction::common::DirectionTable)
//!
//! For a new dimensionality specification, new structs should be created
//! for the new [Dimensionality](crate::core::common::Dimensionality) implementor.

pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use std::{
        fmt::Debug,
        ops::{Index, IndexMut},
    };

    use crate::core::common::*;

    use super::private;

    /// Trait declaring the possible directions to move from tile to tile in specific [Dimensionality](crate::core::common::Dimensionality).
    pub trait Direction<D: Dimensionality + ?Sized>:
        private::SealedDir + Sized + Copy + Clone + Debug + PartialEq
    {
        const N: usize;

        /// Returns all directions in the dimensionality.
        ///
        /// Order is ascending based on their [as_idx()](Directions::as_idx()) return value.
        fn all() -> &'static [Self];

        /// Returns the primary directions in the dimensionality.
        ///
        /// These are the directions that tend into the beginning of the grid (all 0 coordinates).
        fn primary() -> &'static [Self];

        /// Marches the step in the given direction.
        ///
        /// Returns the next [GridPosition](crate::core::position::common::GridPositionTrait) in the given direction,
        /// taking into the account the confines of the specific [GridSize](crate::core::size::common::GridSize).
        ///
        /// Returns `None` if the step is not possible.
        ///
        /// # Examples
        /// ## 2D space
        /// ```
        /// use grid_forge::two_d::*;
        ///
        /// let size = GridSize2D::new(10, 10);
        /// let mut pos = GridPosition2D::new(5, 5);
        ///
        /// for dir in [Direction2D::Up, Direction2D::Left] {
        ///     pos = dir.march_step(&pos, &size).unwrap();
        /// }
        /// assert_eq!(pos, GridPosition2D::new(4, 4));
        ///
        /// // Size is not enough to march in that direction.
        /// let not_valid = Direction2D::Right.march_step(
        ///     &GridPosition2D::new(9,9),
        ///     &size
        /// );
        /// assert_eq!(not_valid, None);
        /// ```
        ///
        /// ## 3D space
        /// ```
        /// use grid_forge::three_d::*;
        ///
        /// let size = GridSize3D::new(10, 10, 10);
        /// let mut pos = GridPosition3D::new(5, 5, 5);
        ///
        /// for dir in [Direction3D::Up, Direction3D::Left, Direction3D::Higher] {
        ///     pos = dir.march_step(&pos, &size).unwrap();
        /// }
        /// assert_eq!(pos, GridPosition3D::new(4, 4, 4));
        ///
        /// // Size is not enough to march in that direction.
        /// let not_valid = Direction3D::Right.march_step(
        ///     &GridPosition3D::new(9,9,9),
        ///     &size
        /// );
        /// assert_eq!(not_valid, None);
        /// ```
        ///
        fn march_step(&self, from: &D::Pos, size: &D::Size) -> Option<D::Pos>;

        /// Returns the opposite direction.
        fn opposite(&self) -> Self;

        /// Returns the usize index for specific direction.
        fn as_idx(&self) -> usize;
    }

    /// Fast lookup table for the data bound to the specific direction.
    pub trait DirectionTable<D: Dimensionality, T>:
        private::Sealed + Index<D::Dir, Output = T> + IndexMut<D::Dir> + AsRef<[T]> + AsMut<[T]>
    {
        type Inner: AsRef<[T]> + AsMut<[T]>;

        fn new_array(values: Self::Inner) -> Self;
        fn inner(&self) -> &Self::Inner;
        fn from_slice(slice: &[T]) -> Self
        where
            T: Copy;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
    pub trait SealedDir {
        const FIRST: Self;
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use crate::core::common::*;

    pub struct MarchStepTestCase<const DIM: usize, D: Dimensionality> {
        pub grid_size: [u32; DIM],
        pub from_coords: [u32; DIM],
        pub dirs: &'static [<D as Dimensionality>::Dir],
        pub expected_coords: [u32; DIM],
        pub converged: bool,
    }

    pub struct DirectionTableTestCase<const DIRS: usize, D: Dimensionality>(
        pub &'static [(D::Dir, u32)],
    );

    pub fn march_step_test<const DIM: usize, D: Dimensionality>(
        test_cases: &[MarchStepTestCase<DIM, D>],
    ) {
        for (
            i,
            MarchStepTestCase {
                grid_size,
                from_coords,
                dirs,
                expected_coords,
                converged,
            },
        ) in test_cases.iter().enumerate()
        {
            let grid_size = D::Size::from_slice(grid_size);
            let mut current_pos = D::Pos::from_slice(from_coords);
            let expected = D::Pos::from_slice(expected_coords);
            let mut finished = true;
            for dir in dirs.iter() {
                match dir.march_step(&current_pos, &grid_size) {
                    Some(pos) => {
                        current_pos = pos;
                    }
                    None => {
                        finished = false;
                        break;
                    }
                }
            }
            assert_eq!(current_pos, expected, "test case {} non-expected ending", i);
            assert_eq!(
                finished, *converged,
                "test case {} non-expected convergence",
                i
            );
        }
    }

    pub fn direction_table_test<
        const DIM: usize,
        D: Dimensionality,
        Table: DirectionTable<D, u32, Inner = [u32; DIM], Output = u32> + Default,
    >(
        test_cases: &[DirectionTableTestCase<DIM, D>],
    ) {
        for (i, data) in test_cases.iter().enumerate() {
            let mut table = Table::default();
            for (dir, value) in data.0.iter() {
                table[*dir] = *value;
            }
            for (dir, value) in data.0.iter() {
                assert_eq!(table[*dir], *value, "test case {}", i);
            }
        }
    }
}
