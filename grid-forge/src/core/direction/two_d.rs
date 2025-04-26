use std::ops::Index;
use std::ops::IndexMut;

use super::common::*;

use super::private::*;
use crate::core::two_d::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Directions2D {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Sealed for Directions2D {}

impl Directions<TwoDim> for Directions2D {
    const N: usize = 4;

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
}

pub struct DirectionTable2D<T> {
    table: [T; 4],
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
}

impl<T: Default> Default for DirectionTable2D<T> {
    fn default() -> Self {
        Self {
            table: [T::default(), T::default(), T::default(), T::default()],
        }
    }
}

impl<T> Index<Directions2D> for DirectionTable2D<T> {
    type Output = T;

    fn index(&self, index: Directions2D) -> &Self::Output {
        &self.table[index.as_idx()]
    }
}

impl<T> IndexMut<Directions2D> for DirectionTable2D<T> {
    fn index_mut(&mut self, index: Directions2D) -> &mut Self::Output {
        &mut self.table[index.as_idx()]
    }
}
