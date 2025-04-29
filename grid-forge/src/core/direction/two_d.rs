use std::ops::Index;
use std::ops::IndexMut;

use super::common::*;

use super::private::*;
use crate::core::two_d::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Direction2D {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl SealedDir for Direction2D {
    const FIRST: Self = Self::Up;
}

impl Direction<TwoDim> for Direction2D {
    const N: usize = 4;

    #[inline]
    fn all() -> &'static [Self] {
        &[Self::Up, Self::Down, Self::Left, Self::Right]
    }

    fn march_step(&self, from: &GridPosition2D, size: &GridSize2D) -> Option<GridPosition2D> {
        let (x_dif, y_dif) = match self {
            Self::Up => {
                if from.y() == 0 {
                    return None;
                }
                (0i32, -1i32)
            }
            Self::Down => {
                if from.y() + 1 == size.y() {
                    return None;
                }
                (0i32, 1i32)
            }
            Self::Left => {
                if from.x() == 0 {
                    return None;
                }
                (-1i32, 0i32)
            }
            Self::Right => {
                if from.x() + 1 == size.x() {
                    return None;
                }
                (1i32, 0i32)
            }
        };
        let (x, y) = (
            (x_dif.wrapping_add_unsigned(from.x())) as u32,
            (y_dif.wrapping_add_unsigned(from.y())) as u32,
        );
        Some(GridPosition2D::new(x, y))
    }

    #[inline]
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    #[inline]
    fn as_idx(&self) -> usize {
        *self as usize
    }
    
    #[inline]
    fn primary() -> &'static [Self] {
        &[Self::Left, Self::Up]
    }
}

pub struct DirectionTable2D<T> {
    table: [T; 4],
}

impl <T>DirectionTable2D<T> {
    pub const fn new(table: [T; 4]) -> Self {
        Self { table }
    }
}

impl<T> Sealed for DirectionTable2D<T> {}
impl<T> DirectionTable<TwoDim, T> for DirectionTable2D<T> {
    type Inner = [T; 4];

    fn new_array(values: [T; 4]) -> Self {
        Self { table: values }
    }

    fn inner(&self) -> &[T; 4] {
        &self.table
    }

    fn from_slice(slice: &[T]) -> Self where T: Copy {
        let table = [slice[0], slice[1], slice[2], slice[3]];
        Self { table }
    }
}

impl<T: Default> Default for DirectionTable2D<T> {
    fn default() -> Self {
        Self {
            table: [T::default(), T::default(), T::default(), T::default()],
        }
    }
}

impl<T> Index<Direction2D> for DirectionTable2D<T> {
    type Output = T;

    fn index(&self, index: Direction2D) -> &Self::Output {
        &self.table[index.as_idx()]
    }
}

impl<T> IndexMut<Direction2D> for DirectionTable2D<T> {
    fn index_mut(&mut self, index: Direction2D) -> &mut Self::Output {
        &mut self.table[index.as_idx()]
    }
}

#[cfg(test)]
mod tests {
    use crate::core::direction::tests::*;
    use crate::core::two_d::*;

    #[test]
    fn test_2d_march_step() {
        const CASES: &[MarchStepTestCase<2, TwoDim>] = &[
            MarchStepTestCase {
                grid_size: [10, 10],
                from_coords: [5, 5],
                dirs: &[
                    Direction2D::Up,
                    Direction2D::Down,
                    Direction2D::Left,
                    Direction2D::Right,
                ],
                expected_coords: [5, 5],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [10, 10],
                from_coords: [5, 5],
                dirs: &[
                    Direction2D::Up,
                    Direction2D::Up,
                    Direction2D::Up,
                    Direction2D::Up,
                ],
                expected_coords: [5, 1],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [2, 2],
                from_coords: [1, 1],
                dirs: &[Direction2D::Up, Direction2D::Left],
                expected_coords: [0, 0],
                converged: true,
            },
            MarchStepTestCase {
                grid_size: [2, 2],
                from_coords: [0, 0],
                dirs: &[Direction2D::Up],
                expected_coords: [0, 0],
                converged: false,
            },
        ];
        march_step_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_direction_table() {
        const CASES: &[DirectionTableTestCase<4, TwoDim>] = &[
            DirectionTableTestCase(&[
                (Direction2D::Up, 22),
                (Direction2D::Down, 33),
                (Direction2D::Left, 44),
                (Direction2D::Right, 55),
            ]),
            DirectionTableTestCase(&[(Direction2D::Up, 66), (Direction2D::Down, 77)]),
        ];

        direction_table_test::<4, TwoDim, DirectionTable2D<u32>>(CASES);
    }
}
