pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use std::{
        fmt::Debug,
        ops::{Add, AddAssign, Index, Sub},
    };

    use crate::core::common::*;

    use super::private;

    pub trait GridPositionTrait<D>
    where
        D: Dimensionality + ?Sized,
        Self: private::Sealed
            + Ord
            + PartialOrd
            + Add<Output = Self>
            + Sub<Output = Self>
            + AddAssign
            + Copy
            + Clone
            + Debug
            + Sized
            + Send
            + Sync,
    {
        type Coords: AsRef<[u32]> + Index<usize, Output = u32>;

        /// Returns the coordinates of the position.
        ///
        /// It is guranteed that the length of the returned slice is equal to the [Dimensionality::N] of the space.
        fn coords(&self) -> Self::Coords;

        fn from_coords(coords: Self::Coords) -> Self;

        fn from_slice(slice: &[u32]) -> Self;

        fn in_range(&self, other: &Self, range: u32) -> bool {
            let mut distance = 0;

            for i in 0..D::N {
                distance += self.coords()[i].max(other.coords()[i])
                    - self.coords()[i].min(other.coords()[i]);
                if distance > range {
                    return false;
                }
            }

            true
        }

        fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self>;

        fn filter_positions(pos: &mut Vec<Self>, to_filter: &[Self]) {
            pos.retain(|p| !to_filter.contains(p));
        }
    }
}

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
pub(crate) mod tests {
    use std::cmp::Ordering;

    use crate::core::common::*;

    pub type ComparisonTestCase<const DIM: usize> = ([u32; DIM], [u32; DIM], Ordering);

    pub type OrderingTestCase<const DIM: usize> = &'static [[u32; DIM]];

    pub type MathOpTestCase<const DIM: usize> = (&'static [[u32; DIM]], [u32; DIM]);

    pub type GenerateRectTestCase<const DIM: usize> = ([u32; DIM], [u32; DIM]);

    /// Tests for comparison of positions.
    ///
    /// Positions should be compared by each dimension separately. If first dimension is equal, then the
    /// second dimension is compared, and so on.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [ComparisonTestCase<DIM>], each case is a tuple of two positions and expected result.
    pub fn compare_test<const DIM: usize, D: Dimensionality>(
        test_cases: &[ComparisonTestCase<DIM>],
    ) {
        for (i, (a, b, expected)) in test_cases.iter().enumerate() {
            let a = D::Pos::from_slice(a);
            let b = D::Pos::from_slice(b);
            assert_eq!(a.cmp(&b), *expected, "test case {}", i);
        }
    }

    /// Tests for ordering of positions.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [OrderingTestCase<DIM>], each case is a tuple of positions and expected result.
    pub fn order_test<const DIM: usize, D: Dimensionality>(test_cases: &[OrderingTestCase<DIM>]) {
        for (i, coords) in test_cases.iter().enumerate() {
            let mut positions = coords
                .iter()
                .map(|c| D::Pos::from_slice(c))
                .collect::<Vec<_>>();
            positions.sort();
            let mut previous = None;
            for pos in positions {
                if let Some(previous) = previous {
                    assert_ne!(
                        pos.cmp(&previous),
                        Ordering::Less,
                        "test case {}; pos: {:?}, previous: {:?}",
                        i,
                        pos,
                        previous
                    );
                }
                previous = Some(pos);
            }
        }
    }

    /// Tests for addition of positions.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [MathOpTestCase<DIM>], each case is a tuple of additions to be performed and expected result.
    pub fn add_test<const DIM: usize, D: Dimensionality>(test_cases: &[MathOpTestCase<DIM>]) {
        for (i, (to_add, expected)) in test_cases.iter().enumerate() {
            let mut position = D::Pos::from_slice(&[0u32; DIM]);

            for var in to_add.iter() {
                position = position + D::Pos::from_slice(var);
            }
            assert_eq!(position, D::Pos::from_slice(expected), "test case {}", i);
        }
    }

    /// Tests for addition of positions.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [MathOpTestCase<DIM>], each case is a tuple of additions to be performed and expected result.
    pub fn add_assign_test<const DIM: usize, D: Dimensionality>(
        test_cases: &[MathOpTestCase<DIM>],
    ) {
        for (i, (to_add, expected)) in test_cases.iter().enumerate() {
            let mut position = D::Pos::from_slice(&[0u32; DIM]);

            for var in to_add.iter() {
                position += D::Pos::from_slice(var);
            }
            assert_eq!(position, D::Pos::from_slice(expected), "test case {}", i);
        }
    }

    /// Tests for subtraction of positions.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [MathOpTestCase<DIM>], each case is a tuple of subtractions to be performed and expected result.
    pub fn sub_test<const DIM: usize, D: Dimensionality>(test_cases: &[MathOpTestCase<DIM>]) {
        for (i, (to_add, expected)) in test_cases.iter().enumerate() {
            let mut position = D::Pos::from_slice(&[0u32; DIM]);

            for var in to_add.iter() {
                position = position - D::Pos::from_slice(var);
            }
            assert_eq!(position, D::Pos::from_slice(expected), "test case {}", i);
        }
    }

    /// Tests for generation of rectangle area.
    ///
    /// # Arguments
    /// - `test_cases` - slice of [GenerateRectTestCase<DIM>], each case is a tuple of two 'corners' of the area.
    pub fn generate_rect_area_test<const DIM: usize, D: Dimensionality>(
        test_cases: &[GenerateRectTestCase<DIM>],
    ) {
        for (i, (a, b)) in test_cases.iter().enumerate() {
            let a = D::Pos::from_slice(a);
            let b = D::Pos::from_slice(b);
            let in_area = D::Pos::generate_rect_area(&a, &b);

            let min_origin = a.min(b);
            let max_origin = a.max(b);

            for pos in in_area {
                assert!(pos >= min_origin && pos <= max_origin, "test case {}", i);
            }
        }
    }
}
