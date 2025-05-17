/// Implements few traits and common methods for the given position type:
/// - `in_range` method;
/// - `filter_positions` method;
/// - [`PartialEq`] and [`Eq`];
/// - [`Hash`];
/// - [`Ord`] and [`PartialOrd`].
macro_rules! __impl_position {
    (
        position_type: $position_type:ident,
        coord_count: $coord_count:literal,
    ) => {
        use std::cmp::Ordering;
        use std::hash::Hash;

        impl $position_type {
            pub fn in_range(&self, other: &Self, range: u32) -> bool {
                let mut distance = 0;

                for i in 0..$coord_count {
                    distance += self.coords()[i].max(other.coords()[i])
                        - self.coords()[i].min(other.coords()[i]);
                    if distance > range {
                        return false;
                    }
                }

                true
            }

            pub fn filter_positions(pos: &mut Vec<Self>, to_filter: &[Self]) {
                pos.retain(|p| !to_filter.contains(p));
            }
        }

        impl PartialEq for $position_type {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..$coord_count {
                    if self.coords()[i] != other.coords()[i] {
                        return false;
                    }
                }
                true
            }
        }

        impl Eq for $position_type {}

        impl Hash for $position_type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.coords().hash(state);
            }
        }

        impl Ord for $position_type {
            fn cmp(&self, other: &Self) -> Ordering {
                match self.coords_sum().cmp(&other.coords_sum()) {
                    Ordering::Equal => {
                        // Compare each coordinate individually for tie-breaking
                        for i in 0..$coord_count {
                            let cmp = self.coords()[i].cmp(&other.coords()[i]);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                        Ordering::Equal
                    }
                    ord => ord,
                }
            }
        }

        impl PartialOrd for $position_type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

/// Implements a collection of utility test functions for verifying the correctness of operations
/// on a specific grid position type. This macro generates various test case structures and
/// associated utility functions to facilitate testing of comparison, ordering, addition,
/// subtraction, and rectangular area generation for position types.
///
/// *IT DOESN'T PRODUCE ANY TESTS BY ITSELF!* It is intended to be used in conjunction with the
/// `#[test]` attribute to generate test cases and run them as part of a test suite.
///
/// When implementing a new grid position type, use every generated test function to ensure
/// that the operations are implemented correctly. This will help catch any bugs or inconsistencies
/// in the implementation.
///
/// # Parameters
///
/// - `position_type`: The type of grid position to be tested.
/// - `coord_count`: The number of coordinates in the grid position type.
///
/// # Generated Structures and Functions
///
/// - **ComparisonTestCase**: A structure that represents a test case comparing two positions,
///   containing fields `a`, `b`, and the `expected` ordering result.
///
/// - **OrderingTestCase**: A structure holding a slice of sorted positions for testing order
///   constraints.
///
/// - **MathOpTestCase**: A structure that represents a test case for mathematical operations
///   (addition/subtraction), containing a slice of positions and an `expected` result.
///
/// - **GenerateRectTestCase**: A structure representing a test case for generating rectangular
///   areas between two positions.
///
/// - **Functions**: The macro also implements various functions to perform tests on the aforementioned
///   test case structures:
///   - `compare_test`: Asserts that the comparison between two positions matches the expected result.
///   - `order_test`: Asserts that a slice of positions is in non-decreasing order.
///   - `add_test` & `add_assign_test`: Asserts that the sum of a series of positions is equal to the
///     expected position, using the `+` operator and `+=` operator respectively.
///   - `sub_test`: Asserts that the subtraction of a series of positions results in the expected
///     position.
///   - `generate_rect_area_test`: Verifies that the generated rectangular area between two positions
///     includes all positions within the defined min/max bounds.
///
/// This macro is highly useful for automated testing of grid position operations to ensure geometric
/// and arithmetic correctness within a grid system.
macro_rules! __impl_position_tests {
    (
        position_type: $position_type:ty,
        coord_count: $coord_count:literal,
    ) => {
        use std::cmp::Ordering;

        struct ComparisonTestCase {
            a: $position_type,
            b: $position_type,
            expected: std::cmp::Ordering,
        }

        impl ComparisonTestCase {
            const fn new(
                a: $position_type,
                b: $position_type,
                expected: std::cmp::Ordering,
            ) -> Self {
                Self { a, b, expected }
            }
        }

        struct OrderingTestCase {
            sorted_positions: &'static [$position_type],
        }

        impl OrderingTestCase {
            const fn new(sorted_positions: &'static [$position_type]) -> Self {
                Self { sorted_positions }
            }
        }

        struct MathOpTestCase {
            positions: &'static [$position_type],
            expected: $position_type,
        }

        impl MathOpTestCase {
            const fn new(positions: &'static [$position_type], expected: $position_type) -> Self {
                Self {
                    positions,
                    expected,
                }
            }
        }

        struct GenerateRectTestCase {
            a: $position_type,
            b: $position_type,
        }

        impl GenerateRectTestCase {
            const fn new(a: $position_type, b: $position_type) -> Self {
                Self { a, b }
            }
        }

        fn compare_test(test_cases: &[ComparisonTestCase]) {
            for (i, test_case) in test_cases.iter().enumerate() {
                let a = test_case.a;
                let b = test_case.b;
                assert_eq!(a.cmp(&b), test_case.expected, "test case {}", i);
            }
        }

        fn order_test(test_cases: &[OrderingTestCase]) {
            for (i, test_case) in test_cases.iter().enumerate() {
                let mut previous = None;
                for pos in test_case.sorted_positions {
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
                    previous = Some(*pos);
                }
            }
        }

        fn add_test(test_cases: &[MathOpTestCase]) {
            for (
                i,
                MathOpTestCase {
                    positions,
                    expected,
                },
            ) in test_cases.iter().enumerate()
            {
                let mut position = <$position_type>::from_coords([0u32; $coord_count]);

                for var in positions.iter() {
                    position = position + *var;
                }
                assert_eq!(position, *expected, "test case {}", i);
            }
        }

        fn add_assign_test(test_cases: &[MathOpTestCase]) {
            for (
                i,
                MathOpTestCase {
                    positions,
                    expected,
                },
            ) in test_cases.iter().enumerate()
            {
                let mut position = <$position_type>::from_coords([0u32; $coord_count]);

                for var in positions.iter() {
                    position += *var;
                }
                assert_eq!(position, *expected, "test case {}", i);
            }
        }

        fn sub_test(test_cases: &[MathOpTestCase]) {
            for (
                i,
                MathOpTestCase {
                    positions,
                    expected,
                },
            ) in test_cases.iter().enumerate()
            {
                let mut position = <$position_type>::from_coords([0u32; $coord_count]);

                for var in positions.iter() {
                    position = position - *var;
                }
                assert_eq!(position, *expected, "test case {}", i);
            }
        }

        fn generate_rect_area_test(test_cases: &[GenerateRectTestCase]) {
            for (i, GenerateRectTestCase { a, b }) in test_cases.iter().enumerate() {
                let in_area = <$position_type>::generate_rect_area(&a, &b);

                let min_origin = a.min(b);
                let max_origin = a.max(b);

                for pos in in_area {
                    assert!(pos >= *min_origin && pos <= *max_origin, "test case {}", i);
                }
            }
        }
    };
}

pub(crate) use __impl_position;

#[cfg(test)]
pub(crate) use __impl_position_tests;
