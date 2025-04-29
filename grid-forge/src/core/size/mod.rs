pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use std::fmt::Debug;

    use crate::core::common::*;

    use super::private;

    pub trait GridSize<D: Dimensionality + ?Sized>: private::Sealed + Clone + Copy + Debug {
        fn from_slice(slice: &[u32]) -> Self;

        fn is_position_valid(&self, position: &D::Pos) -> bool;

        fn is_contained_within(&self, other: &Self) -> bool;

        fn get_all_possible_positions(&self) -> Vec<D::Pos>;

        fn distance_from_border(&self, position: &D::Pos) -> Option<u32>;

        fn distance_from_center(&self, position: &D::Pos) -> Option<u32>;

        fn center(&self) -> D::Pos;

        fn max_tile_count(&self) -> usize;

        fn offset(&self, pos: &D::Pos) -> usize;

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
