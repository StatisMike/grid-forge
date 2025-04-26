use std::ops::Index;
use std::ops::IndexMut;

use super::common::*;

use super::private::*;
use crate::core::three_d::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Directions3D {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}
impl super::private::Sealed for Directions3D {}

impl Directions<ThreeDim> for Directions3D {
    const N: usize = 6;

    fn all() -> &'static [Self] {
        &[
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::Forward,
            Self::Backward,
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
            Self::Forward => {
                if from.z() == 0 {
                    return None;
                }
                (0i32, 0i32, -1i32)
            }
            Self::Backward => {
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
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }

    #[inline]
    fn as_idx(&self) -> usize {
        *self as usize
    }
}

pub struct DirectionTable3D<T> {
    table: [T; 6],
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

impl<T> Index<Directions3D> for DirectionTable3D<T> {
    type Output = T;

    fn index(&self, index: Directions3D) -> &Self::Output {
        &self.table[index.as_idx()]
    }
}

impl<T> IndexMut<Directions3D> for DirectionTable3D<T> {
    fn index_mut(&mut self, index: Directions3D) -> &mut Self::Output {
        &mut self.table[index.as_idx()]
    }
}
