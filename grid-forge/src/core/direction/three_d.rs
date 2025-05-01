use std::ops::Index;
use std::ops::IndexMut;

use super::common::*;

use super::private::*;
use crate::core::three_d::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction3D {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Higher = 4,
    Lower = 5,
}
impl super::private::SealedDir for Direction3D {
    const FIRST: Self = Self::Up;
}

impl Direction<ThreeDim> for Direction3D {
    const N: usize = 6;

    fn all() -> &'static [Self] {
        &[
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::Higher,
            Self::Lower,
        ]
    }

    fn march_step(&self, from: &GridPosition3D, size: &GridSize3D) -> Option<GridPosition3D> {
        let (x_dif, y_dif, z_dif) = match self {
            Self::Up => {
                if from.y() == 0 {
                    return None;
                }
                (0i32, -1i32, 0i32)
            }
            Self::Down => {
                if from.y() + 1 == size.y() {
                    return None;
                }
                (0i32, 1i32, 0i32)
            }
            Self::Left => {
                if from.x() == 0 {
                    return None;
                }
                (-1i32, 0i32, 0i32)
            }
            Self::Right => {
                if from.x() + 1 == size.x() {
                    return None;
                }
                (1i32, 0i32, 0i32)
            }
            Self::Higher => {
                if from.z() == 0 {
                    return None;
                }
                (0i32, 0i32, -1i32)
            }
            Self::Lower => {
                if from.z() + 1 == size.z() {
                    return None;
                }
                (0i32, 0i32, 1i32)
            }
        };
        let (x, y, z) = (
            (x_dif.wrapping_add_unsigned(from.x())) as u32,
            (y_dif.wrapping_add_unsigned(from.y())) as u32,
            (z_dif.wrapping_add_unsigned(from.z()) as u32),
        );

        Some(GridPosition3D::new(x, y, z))
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Higher => Self::Lower,
            Self::Lower => Self::Higher,
        }
    }

    #[inline]
    fn as_idx(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn primary() -> &'static [Self] {
        &[Self::Left, Self::Up, Self::Higher]
    }
}

#[derive(Clone, Debug)]
pub struct DirectionTable3D<T> {
    table: [T; 6],
}

impl<T> DirectionTable3D<T> {
    pub const fn new(table: [T; 6]) -> Self {
        Self { table }
    }
}

impl<T> Sealed for DirectionTable3D<T> {}
impl<T> DirectionTable<ThreeDim, T> for DirectionTable3D<T> {
    type Inner = [T; 6];

    fn new_array(values: [T; 6]) -> Self {
        Self { table: values }
    }

    fn inner(&self) -> &[T; 6] {
        &self.table
    }

    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy,
    {
        let table = [slice[0], slice[1], slice[2], slice[3], slice[4], slice[5]];
        Self { table }
    }
}

impl<T: Default> Default for DirectionTable3D<T> {
    fn default() -> Self {
        Self {
            table: [
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
            ],
        }
    }
}

impl<T> Index<Direction3D> for DirectionTable3D<T> {
    type Output = T;

    fn index(&self, index: Direction3D) -> &Self::Output {
        &self.table[index.as_idx()]
    }
}

impl<T> IndexMut<Direction3D> for DirectionTable3D<T> {
    fn index_mut(&mut self, index: Direction3D) -> &mut Self::Output {
        &mut self.table[index.as_idx()]
    }
}

impl<T> AsRef<[T]> for DirectionTable3D<T> {
    fn as_ref(&self) -> &[T] {
        self.table.as_ref()
    }
}

impl<T> AsMut<[T]> for DirectionTable3D<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.table.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::direction::tests::*;
    use crate::core::three_d::*;

    #[test]
    fn test_3d_march_step() {
        const CASES: &[MarchStepTestCase<3, ThreeDim>] = &[
            MarchStepTestCase {
                grid_size: [10, 10, 10],
                from_coords: [5, 5, 5],
                dirs: &[
                    Direction3D::Up,
                    Direction3D::Down,
                    Direction3D::Left,
                    Direction3D::Right,
                    Direction3D::Higher,
                    Direction3D::Lower,
                ],
                expected_coords: [5, 5, 5],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [10, 10, 10],
                from_coords: [5, 5, 5],
                dirs: &[
                    Direction3D::Up,
                    Direction3D::Up,
                    Direction3D::Up,
                    Direction3D::Up,
                ],
                expected_coords: [5, 1, 5],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [1, 1, 1],
                dirs: &[Direction3D::Up, Direction3D::Left, Direction3D::Higher],
                expected_coords: [0, 0, 0],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [0, 1, 1],
                dirs: &[Direction3D::Left],
                expected_coords: [0, 1, 1],
                converged: false,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [2, 1, 1],
                dirs: &[Direction3D::Right],
                expected_coords: [2, 1, 1],
                converged: false,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [1, 0, 1],
                dirs: &[Direction3D::Up],
                expected_coords: [1, 0, 1],
                converged: false,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [1, 2, 1],
                dirs: &[Direction3D::Down],
                expected_coords: [1, 2, 1],
                converged: false,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [1, 1, 0],
                dirs: &[Direction3D::Higher],
                expected_coords: [1, 1, 0],
                converged: false,
            },
            MarchStepTestCase {
                grid_size: [3, 3, 3],
                from_coords: [1, 1, 2],
                dirs: &[Direction3D::Lower],
                expected_coords: [1, 1, 2],
                converged: false,
            },
        ];
        march_step_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_direction_table() {
        const CASES: &[DirectionTableTestCase<6, ThreeDim>] = &[
            DirectionTableTestCase(&[
                (Direction3D::Up, 22),
                (Direction3D::Down, 33),
                (Direction3D::Left, 44),
                (Direction3D::Right, 55),
                (Direction3D::Higher, 66),
                (Direction3D::Lower, 77),
            ]),
            DirectionTableTestCase(&[(Direction3D::Up, 88), (Direction3D::Down, 99)]),
        ];

        direction_table_test::<6, ThreeDim, DirectionTable3D<u32>>(CASES);
    }
}
