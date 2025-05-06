pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use std::fmt::Debug;

    use crate::core::common::*;

    use super::private;

    /// Size of the grid in given [Dimensionality](crate::core::common::Dimensionality).
    /// 
    /// It creates bounds for the [`GridMap`], provides it with most of the computational methods to work with positions.
    pub trait GridSize<D: Dimensionality + ?Sized>: private::Sealed + Clone + Copy + Debug {

        /// Creates a new [`GridSize`] from a slice of coordinates.
        /// 
        /// # Panics
        /// Panics if the length of the slice is not equal to the [Dimensionality::N] of the space.
        fn from_slice(slice: &[u32]) -> Self;

        /// Checks if the position is valid for the grid.
        ///
        /// Position is valid if it could be contained within the [GridMap] of given [GridSize].
        fn is_position_valid(&self, position: &D::Pos) -> bool;

        /// Checks if it could be contained within other [`GridSize`].
        /// 
        /// Return `true` if the size is smaller or equal to the other size.
        fn is_contained_within(&self, other: &Self) -> bool;

        /// Returns all possible [`GridPosition`] in the [`GridMap`] of this [`GridSize`].
        fn get_all_possible_positions(&self) -> Vec<D::Pos>;

        /// Checks the distance from the border of the [`GridMap`] of this [`GridSize`].
        /// 
        /// Returns `None` if the position is not valid for the grid.
        fn distance_from_border(&self, position: &D::Pos) -> Option<u32>;

        fn distance_from_center(&self, position: &D::Pos) -> Option<u32>;

        /// Returns the center [`GridPosition`] of the [`GridMap`] of this [`GridSize`].
        /// 
        /// If the size is even in any dimension, the coordinate will be rounded down.
        fn center(&self) -> D::Pos;

        /// Returns the maximum number of tiles in the [`GridMap`] of this [`GridSize`].
        fn max_tile_count(&self) -> usize;

        /// Returns the offset of the [`GridPosition`] in the [`GridMap`] of this [`GridSize`].
        /// 
        /// It is mostly implementation detail - it is used for calculating the offset of the tile in the 
        /// internal container of the map.
        /// 
        /// See also [`pos_from_offset()`](GridSize::pos_from_offset).
        fn offset(&self, pos: &D::Pos) -> usize;

        /// Returns the [`GridPosition`] from the offset in the [`GridMap`] of this [`GridSize`].
        /// 
        /// It is mostly implementation detail - it is used for calculating the position from the offset
        /// of the tile in the internal container of the map.
        /// 
        /// See also [`offset()`](GridSize::offset).
        fn pos_from_offset(&self, offset: usize) -> D::Pos;
    }
}

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::core::common::*;

    pub type Dims1Dims2BoolTestCase<const DIM: usize> = ([u32; DIM], [u32; DIM], bool);

    pub type DimsCountTestCase<const DIM: usize> = ([u32; DIM], usize);

    pub type Dims1Dims2CountTestCase<const DIM: usize> = ([u32; DIM], [u32; DIM], u32);

    pub type Dims1Dims2TestCase<const DIM: usize> = ([u32; DIM], [u32; DIM]);

    /// Tests for `is_position_valid` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2BoolTestCase<DIM>], each case is a tuple of size, position, and expected result.
    pub fn test_is_position_valid<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2BoolTestCase<DIM>],
    ) {
        for (i, (size, pos, expected)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(pos);
            assert_eq!(size.is_position_valid(&pos), *expected, "test case {}", i);
        }
    }

    /// Tests for `is_contained_within` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2BoolTestCase<DIM>], each case is a tuple of size_1, size_2, and expected result.
    pub fn test_is_contained_within<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2BoolTestCase<DIM>],
    ) {
        for (i, (size_1, size_2, expected)) in test_cases.iter().enumerate() {
            let size_1 = D::Size::from_slice(size_1);
            let size_2 = D::Size::from_slice(size_2);
            assert_eq!(
                size_1.is_contained_within(&size_2),
                *expected,
                "test case {}",
                i
            );
        }
    }

    /// Tests for `get_all_possible_positions` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [DimsCountTestCase<DIM>], each case is a tuple of size and expected number of possible positions.
    pub fn test_get_all_possible_positions<const DIM: usize, D: Dimensionality>(
        test_cases: &[DimsCountTestCase<DIM>],
    ) {
        for (i, (size, expected)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let positions = size.get_all_possible_positions();
            assert_eq!(
                positions.len(),
                *expected,
                "test case {}: wrong number of positions",
                i
            );
            for pos in positions {
                assert!(
                    size.is_position_valid(&pos),
                    "test case {}: position {:?} is not valid",
                    i,
                    pos
                );
            }
        }
    }

    /// Tests for `distance_from_border` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2CountTestCase<DIM>], each case is a tuple of size, position, and expected result.
    pub fn test_distance_from_border<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2CountTestCase<DIM>],
    ) {
        for (i, (size, position, expected)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(position);
            assert_eq!(
                size.distance_from_border(&pos).unwrap(),
                *expected,
                "test case {}",
                i
            );
        }
    }

    /// Tests for `distance_from_center` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2CountTestCase<DIM>], each case is a tuple of size, position, and expected result.
    pub fn test_distance_from_center<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2CountTestCase<DIM>],
    ) {
        for (i, (size, pos, expected)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(pos);
            assert_eq!(
                size.distance_from_center(&pos).unwrap(),
                *expected,
                "test case {}",
                i
            );
        }
    }

    /// Tests for `center` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2TestCase<DIM>], each case is a tuple of size and expected result.
    pub fn test_center<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2TestCase<DIM>],
    ) {
        for (i, (size, pos)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(pos);
            assert_eq!(size.center(), pos, "test case {}", i);
        }
    }

    /// Tests for `max_tile_count` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [DimsCountTestCase<DIM>], each case is a tuple of size and expected number of possible positions.
    pub fn test_max_tile_count<const DIM: usize, D: Dimensionality>(
        test_cases: &[DimsCountTestCase<DIM>],
    ) {
        for (i, (size, n_pos)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            assert_eq!(size.max_tile_count(), *n_pos, "test case {}", i);
        }
    }

    /// Tests for `offset` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2CountTestCase<DIM>], each case is a tuple of size, position, and expected result.
    pub fn test_offset<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2CountTestCase<DIM>],
    ) {
        for (i, (size, pos, expected)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(pos);
            assert_eq!(size.offset(&pos), *expected as usize, "test case {}", i);
        }
    }

    /// Tests for `pos_from_offset` method.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [Dims1Dims2CountTestCase<DIM>], each case is a tuple of size, position, and expected result.
    pub fn test_pos_from_offset<const DIM: usize, D: Dimensionality>(
        test_cases: &[Dims1Dims2CountTestCase<DIM>],
    ) {
        for (i, (size, expected, offset)) in test_cases.iter().enumerate() {
            let size = D::Size::from_slice(size);
            let pos = D::Pos::from_slice(expected);
            assert_eq!(
                size.pos_from_offset(*offset as usize),
                pos,
                "test case {}",
                i
            );
        }
    }
}
