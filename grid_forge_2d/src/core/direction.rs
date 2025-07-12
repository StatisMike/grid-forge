use super::position::GridPosition2D;
use super::size::GridSize2D;

/// Direction in the 2D space.
///
/// These are all the directions that are possible on the rectangular 2D grid.
/// Diagonal directions are not taken into account.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction2D {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction2D {
    /// Number of directions in the 2D space.
    pub const COUNT: usize = 4;

    /// Primary directions in the 2D space.
    ///
    /// Primary directions are the directions that tend into the beginning
    /// of the grid (all 0 coordinates).
    pub const PRIMARY: [Self; 2] = [Self::Left, Self::Up];

    /// All directions in the 2D space.
    ///
    /// Order is ascending based on their [`as_idx()`](Self::as_idx()) return value.
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    /// Marches the step in the given direction.
    ///
    /// Returns the next [GridPosition](crate::core::position::common::GridPositionTrait) in the given direction,
    /// taking into the account the confines of the specific [GridSize](crate::core::size::common::GridSize).
    ///
    /// Returns `None` if the step is not possible.
    ///
    /// # Examples
    /// ```
    /// use grid_forge::two_d::{GridPosition2D, GridSize2D, Direction2D};
    ///
    /// let size = GridSize2D::new(10, 10);
    /// let mut pos = GridPosition2D::new(5, 5);
    ///
    /// for dir in [Direction2D::Up, Direction2D::Left] {
    ///     pos = dir.march_step(&pos, &size).unwrap();
    /// }
    /// assert_eq!(pos, GridPosition2D::new(4, 4));
    ///
    /// // Size is not enough to march in that direction.
    /// let not_valid = Direction2D::Right.march_step(
    ///     &GridPosition2D::new(9,9),
    ///     &size
    /// );
    /// assert_eq!(not_valid, None);
    /// ```
    pub fn march_step(&self, from: &GridPosition2D, size: &GridSize2D) -> Option<GridPosition2D> {
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

    /// Returns the opposite direction.
    #[inline]
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Returns the usize index for specific direction.
    #[inline]
    pub fn as_idx(&self) -> usize {
        *self as usize
    }

    /// Returns the [`Direction2D`] from the given index.
    #[inline]
    pub fn from_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Up),
            1 => Some(Self::Down),
            2 => Some(Self::Left),
            3 => Some(Self::Right),
            _ => None,
        }
    }
}

grid_forge_core::__impl_direction_table! {
    direction_table: DirectionTable2D,
    direction: Direction2D,
    direction_count: 4,
}

#[cfg(test)]
mod tests {

    use super::*;

    grid_forge_core::__impl_direction_tests! {
        direction: Direction2D,
        direction_table: DirectionTable2D,
        dimension_count: 2,
        position_type: GridPosition2D,
        size_type: GridSize2D,
    }

    #[test]
    fn test_2d_march_step_10x10() {
        const CASES: &[MarchStepTestCase] = &[
            MarchStepTestCase::new(
                GridPosition2D::new(5, 5),
                &[
                    Direction2D::Up,
                    Direction2D::Down,
                    Direction2D::Left,
                    Direction2D::Right,
                ],
                GridPosition2D::new(5, 5),
                true,
            ),
            MarchStepTestCase::new(
                GridPosition2D::new(5, 5),
                &[
                    Direction2D::Up,
                    Direction2D::Up,
                    Direction2D::Up,
                    Direction2D::Up,
                ],
                GridPosition2D::new(5, 1),
                true,
            ),
        ];
        march_step_test(GridSize2D::new(10, 10), CASES);
    }

    #[test]
    fn test_2d_march_step_2x2() {
        const CASES: &[MarchStepTestCase] = &[
            MarchStepTestCase::new(
                GridPosition2D::new(1, 1),
                &[Direction2D::Up, Direction2D::Left],
                GridPosition2D::new(0, 0),
                true,
            ),
            MarchStepTestCase::new(
                GridPosition2D::new(0, 0),
                &[Direction2D::Up],
                GridPosition2D::new(0, 0),
                false,
            ),
        ];
        march_step_test(GridSize2D::new(2, 2), CASES);
    }

    #[test]
    fn test_2d_direction_table() {
        const CASES: &[DirectionTableTestCase] = &[
            DirectionTableTestCase::new(&[
                (Direction2D::Up, 22),
                (Direction2D::Down, 33),
                (Direction2D::Left, 44),
                (Direction2D::Right, 55),
            ]),
            DirectionTableTestCase::new(&[(Direction2D::Up, 66), (Direction2D::Down, 77)]),
        ];

        direction_table_test(CASES);
    }
}
