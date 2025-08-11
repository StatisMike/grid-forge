#[doc(hidden)]
#[macro_export]
macro_rules! __impl_direction_table {
    (
        direction_table: $direction_table_type:ident,
        direction: $direction_type:ident,
        direction_count: $direction_count:literal,

    ) => {
        use std::ops::Index;
        use std::ops::IndexMut;

        /// Fast lookup table for the data indexable by the direction.
        #[derive(Debug, Clone)]
        pub struct $direction_table_type<T> {
            inner: [T; $direction_count],
        }

        impl<T> $direction_table_type<T> {
            pub const fn new(inner: [T; $direction_count]) -> Self {
                Self { inner }
            }

            pub fn inner(&self) -> &[T; $direction_count] {
                &self.inner
            }

            /// Immutable iterator over data in table.
            /// 
            /// See also: [`iter_mut`](Self::iter_mut) and [`into_iter`](Self::into_iter).
            pub fn iter(&self) -> impl Iterator<Item = ($direction_type, &T)> {
                $direction_type::ALL
                    .into_iter()
                    .zip(self.inner.iter())
            }

            /// Mutable iterator over data in table.
            /// 
            /// See also: [`iter`](Self::iter) and [`into_iter`](Self::into_iter).
            pub fn iter_mut(&mut self) -> impl Iterator<Item = ($direction_type, &mut T)> {
                $direction_type::ALL
                    .into_iter()
                    .zip(self.inner.iter_mut())
            }

            /// Consuming iterator over data in table.
            /// 
            /// See also: [`iter`](Self::iter) and [`into_iter`](Self::into_iter).
            pub fn into_iter(self) -> impl Iterator<Item = ($direction_type, T)> {
                $direction_type::ALL
                    .into_iter()
                    .zip(self.inner.into_iter())
            }
        }

        impl<T: Default> Default for $direction_table_type<T> {
            fn default() -> Self {
                Self {
                    inner: core::array::from_fn(|_| T::default()),
                }
            }
        }

        impl<T> Index<$direction_type> for $direction_table_type<T> {
            type Output = T;

            fn index(&self, index: $direction_type) -> &Self::Output {
                &self.inner[index.as_idx()]
            }
        }

        impl<T> IndexMut<$direction_type> for $direction_table_type<T> {
            fn index_mut(&mut self, index: $direction_type) -> &mut Self::Output {
                &mut self.inner[index.as_idx()]
            }
        }

        impl<T> AsRef<[T]> for $direction_table_type<T> {
            fn as_ref(&self) -> &[T] {
                self.inner.as_ref()
            }
        }

        impl<T> AsMut<[T]> for $direction_table_type<T> {
            fn as_mut(&mut self) -> &mut [T] {
                self.inner.as_mut()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_direction_tests {
    (
        direction: $direction_type:ty,
        direction_table: $direction_table_type:ident,
        dimension_count: $dimension_count:literal,
        position_type: $position_type:ty,
        size_type: $size_type:ty,
    ) => {
        #[test]
        fn test_direction_required_elements() {
            let _count: usize = <$direction_type>::COUNT;
            let _primary = <$direction_type>::PRIMARY;
            let _all: &'static [$direction_type] = &<$direction_type>::ALL;

            let first = <$direction_type>::ALL[0];
            first.march_step(
                &<$position_type>::from_coords([0; $dimension_count]),
                &<$size_type>::from_slice(&[1; $dimension_count]),
            );

            for dir in <$direction_type>::ALL {
                let _opposite = dir.opposite();
                let _as_idx: usize = dir.as_idx();
            }

            for idx in 0..<$direction_type>::COUNT {
                let from_idx = <$direction_type>::from_idx(idx);
                assert!(from_idx.is_some());
            }
        }

        /// Test case for the 'march_step' test.
        struct MarchStepTestCase {
            from: $position_type,
            dirs: &'static [$direction_type],
            expected: $position_type,
            to_the_end: bool,
        }

        impl MarchStepTestCase {
            pub const fn new(
                from: $position_type,
                dirs: &'static [$direction_type],
                expected: $position_type,
                to_the_end: bool,
            ) -> Self {
                Self {
                    from,
                    dirs,
                    expected,
                    to_the_end,
                }
            }
        }

        fn march_step_test(size: $size_type, cases: &[MarchStepTestCase]) {
            for (
                i,
                MarchStepTestCase {
                    from,
                    dirs,
                    expected,
                    to_the_end,
                },
            ) in cases.iter().enumerate()
            {
                let mut current_pos = *from;
                let mut finished = true;
                for dir in dirs.iter() {
                    match dir.march_step(&current_pos, &size) {
                        Some(pos) => {
                            current_pos = pos;
                        }
                        None => {
                            finished = false;
                            break;
                        }
                    }
                }
                assert_eq!(
                    current_pos, *expected,
                    "test case {} non-expected ending",
                    i
                );
                assert_eq!(
                    finished, *to_the_end,
                    "test case {} non-expected to the end state",
                    i
                );
            }
        }

        struct DirectionTableTestCase(&'static [($direction_type, u32)]);

        impl DirectionTableTestCase {
            const fn new(table: &'static [($direction_type, u32)]) -> Self {
                Self(table)
            }
        }

        fn direction_table_test(cases: &[DirectionTableTestCase]) {
            let mut table = <$direction_table_type<u32>>::default();

            for (i, data) in cases.iter().enumerate() {
                for (dir, value) in data.0.iter() {
                    table[*dir] = *value;
                }

                for (dir, value) in data.0.iter() {
                    assert_eq!(table[*dir], *value, "test case {}", i);
                }
            }
        }
    };
}
