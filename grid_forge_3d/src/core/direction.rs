use grid_forge_core::__impl_direction_table;

use super::{GridPosition3D, GridSize3D};

#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction3D {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Higher = 4,
    Lower = 5,
}

impl Direction3D {
    pub const COUNT: usize = 6;
    pub const PRIMARY: [Self; 3] = [Self::Left, Self::Up, Self::Higher];
    pub const ALL: [Self; 6] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Higher,
        Self::Lower,
    ];

    pub fn march_step(&self, from: &GridPosition3D, size: &GridSize3D) -> Option<GridPosition3D> {
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

    pub fn opposite(&self) -> Self {
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
    pub fn as_idx(&self) -> usize {
        *self as usize
    }

    pub fn from_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Up),
            1 => Some(Self::Down),
            2 => Some(Self::Left),
            3 => Some(Self::Right),
            4 => Some(Self::Higher),
            5 => Some(Self::Lower),
            _ => None,
        }
    }

    #[inline]
    pub fn primary() -> &'static [Self] {
        &[Self::Left, Self::Up, Self::Higher]
    }
}

__impl_direction_table! {
    direction_table: DirectionTable3D,
    direction: Direction3D,
    direction_count: 6,
}

#[cfg(test)]
mod tests {
    use super::*;
    grid_forge_core::__impl_direction_tests! {
        direction: Direction3D,
        direction_table: DirectionTable3D,
        dimension_count: 3,
        position_type: GridPosition3D,
        size_type: GridSize3D,
    }

    #[test]
    fn test_3d_march_step_10x10x10() {
        const CASES: &[MarchStepTestCase] = &[
            MarchStepTestCase::new(
                GridPosition3D::new(5, 5, 5),
                &[
                    Direction3D::Up,
                    Direction3D::Down,
                    Direction3D::Left,
                    Direction3D::Right,
                    Direction3D::Higher,
                    Direction3D::Lower,
                ],
                GridPosition3D::new(5, 5, 5),
                true,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(5, 5, 5),
                &[
                    Direction3D::Up,
                    Direction3D::Up,
                    Direction3D::Up,
                    Direction3D::Up,
                ],
                GridPosition3D::new(5, 1, 5),
                true,
            ),
        ];
        march_step_test(GridSize3D::new(10, 10, 10), CASES);
    }

    #[test]
    fn test_3d_march_step_3x3x3() {
        const CASES: &[MarchStepTestCase] = &[
            MarchStepTestCase::new(
                GridPosition3D::new(1, 1, 1),
                &[Direction3D::Up, Direction3D::Left, Direction3D::Higher],
                GridPosition3D::new(0, 0, 0),
                true,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(0, 1, 1),
                &[Direction3D::Left],
                GridPosition3D::new(0, 1, 1),
                false,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(2, 1, 1),
                &[Direction3D::Right],
                GridPosition3D::new(2, 1, 1),
                false,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(1, 0, 1),
                &[Direction3D::Up],
                GridPosition3D::new(1, 0, 1),
                false,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(1, 2, 1),
                &[Direction3D::Down],
                GridPosition3D::new(1, 2, 1),
                false,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(1, 1, 0),
                &[Direction3D::Higher],
                GridPosition3D::new(1, 1, 0),
                false,
            ),
            MarchStepTestCase::new(
                GridPosition3D::new(1, 1, 2),
                &[Direction3D::Lower],
                GridPosition3D::new(1, 1, 2),
                false,
            ),
        ];
        march_step_test(GridSize3D::new(3, 3, 3), CASES);
    }

    #[test]
    fn test_3d_direction_table() {
        const CASES: &[DirectionTableTestCase] = &[
            DirectionTableTestCase::new(&[
                (Direction3D::Up, 22),
                (Direction3D::Down, 33),
                (Direction3D::Left, 44),
                (Direction3D::Right, 55),
                (Direction3D::Higher, 66),
                (Direction3D::Lower, 77),
            ]),
            DirectionTableTestCase::new(&[(Direction3D::Up, 88), (Direction3D::Down, 99)]),
        ];

        direction_table_test(CASES);
    }
}
