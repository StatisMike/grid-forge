macro_rules! __impl_grid_size_tests {
    (
        size: $size_type:ty,
        position: $position_type:ty,
    ) => {
        struct SizePosBoolTestCase {
            size: $size_type,
            pos: $position_type,
            expected: bool,
        }

        impl SizePosBoolTestCase {
            const fn new(size: $size_type, pos: $position_type, expected: bool) -> Self {
                Self {
                    size,
                    pos,
                    expected,
                }
            }
        }

        /// Tests for `is_position_valid` method.
        ///
        /// Every [`SizePosBoolTestCase`] holds the grid size, position and the expected result.
        fn test_is_position_valid(test_cases: &[SizePosBoolTestCase]) {
            for (
                i,
                SizePosBoolTestCase {
                    size,
                    pos,
                    expected,
                },
            ) in test_cases.iter().enumerate()
            {
                assert_eq!(size.is_position_valid(&pos), *expected, "test case {}", i);
            }
        }

        struct SizeSizeBoolTestCase {
            size_1: $size_type,
            size_2: $size_type,
            expected: bool,
        }

        impl SizeSizeBoolTestCase {
            const fn new(size_1: $size_type, size_2: $size_type, expected: bool) -> Self {
                Self {
                    size_1,
                    size_2,
                    expected,
                }
            }
        }

        /// Tests for `is_contained_within` method.
        ///
        /// Every [`SizeSizeBoolTestCase`] holds two sizes and the expected result.
        fn test_is_contained_within(test_cases: &[SizeSizeBoolTestCase]) {
            for (
                i,
                SizeSizeBoolTestCase {
                    size_1,
                    size_2,
                    expected,
                },
            ) in test_cases.iter().enumerate()
            {
                assert_eq!(
                    size_1.is_contained_within(&size_2),
                    *expected,
                    "test case {}",
                    i
                );
            }
        }

        struct PosCountTestCase {
            size: $size_type,
            expected_count: usize,
        }

        impl PosCountTestCase {
            const fn new(size: $size_type, expected_count: usize) -> Self {
                Self {
                    size,
                    expected_count,
                }
            }
        }

        fn test_get_all_possible_positions(test_cases: &[PosCountTestCase]) {
            for (
                i,
                PosCountTestCase {
                    size,
                    expected_count,
                },
            ) in test_cases.iter().enumerate()
            {
                let positions = size.get_all_possible_positions();
                assert_eq!(positions.len(), *expected_count, "test case {}", i);
                for pos in positions {
                    assert!(size.is_position_valid(&pos), "test case {}", i);
                }
            }
        }

        fn test_max_tile_count(test_cases: &[PosCountTestCase]) {
            for (
                i,
                PosCountTestCase {
                    size,
                    expected_count,
                },
            ) in test_cases.iter().enumerate()
            {
                assert_eq!(size.max_tile_count(), *expected_count, "test case {}", i);
            }
        }

        struct SizePosNumTestCase {
            size: $size_type,
            pos: $position_type,
            number: usize,
        }

        impl SizePosNumTestCase {
            const fn new(size: $size_type, pos: $position_type, number: usize) -> Self {
                Self { size, pos, number }
            }
        }

        fn test_distance_from_border(test_cases: &[SizePosNumTestCase]) {
            for (i, SizePosNumTestCase { size, pos, number }) in test_cases.iter().enumerate() {
                assert_eq!(
                    size.distance_from_border(&pos).unwrap(),
                    *number as u32,
                    "test case {}",
                    i
                );
            }
        }

        fn test_distance_from_center(test_cases: &[SizePosNumTestCase]) {
            for (i, SizePosNumTestCase { size, pos, number }) in test_cases.iter().enumerate() {
                assert_eq!(
                    size.distance_from_center(&pos).unwrap(),
                    *number as u32,
                    "test case {}",
                    i
                );
            }
        }

        fn test_offset(test_cases: &[SizePosNumTestCase]) {
            for (i, SizePosNumTestCase { size, pos, number }) in test_cases.iter().enumerate() {
                assert_eq!(size.offset(&pos), *number, "test case {}", i);
            }
        }

        fn test_pos_from_offset(test_cases: &[SizePosNumTestCase]) {
            for (i, SizePosNumTestCase { size, pos, number }) in test_cases.iter().enumerate() {
                assert_eq!(size.pos_from_offset(*number), *pos, "test case {}", i);
            }
        }

        struct SizePosTestCase {
            size: $size_type,
            pos: $position_type,
        }

        impl SizePosTestCase {
            const fn new(size: $size_type, pos: $position_type) -> Self {
                Self { size, pos }
            }
        }

        fn test_center(test_cases: &[SizePosTestCase]) {
            for (i, SizePosTestCase { size, pos }) in test_cases.iter().enumerate() {
                assert_eq!(size.center(), *pos, "test case {}", i);
            }
        }
    };
}

#[cfg(test)]
pub(crate) use __impl_grid_size_tests;
