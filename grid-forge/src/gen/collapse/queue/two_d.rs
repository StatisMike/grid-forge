use std::cmp::Ordering;

use crate::{core::two_d::TwoDim, two_d::GridPosition2D};

use super::position::private::{
    PositionQueueDirection, PositionQueueProcession, PositionQueueStartingPoint,
};

/// Enum defining the starting point of the collapse wave.
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueStartingPoint2D {
    #[default]
    /// Starts at the `(0, 0)` position.
    UpLeft,
    /// Starts at the `(0, max)` position.
    UpRight,
    /// Starts at the `(max, 0)` position.
    DownLeft,
    /// Starts at the `(max, max)` position.
    DownRight,
}

impl PositionQueueStartingPoint<TwoDim> for PositionQueueStartingPoint2D {}

/// Enum defining the direction in which the tiles will be collapsed.
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueDirection2D {
    #[default]
    /// Collapses tiles in a rowwise fashion.
    Rowwise,
    /// Collapses tiles in a columnwise fashion.
    Columnwise,
}

impl PositionQueueDirection<TwoDim> for PositionQueueDirection2D {}

pub struct PositionQueueProcession2D;

impl PositionQueueProcession<TwoDim> for PositionQueueProcession2D {
    type StartingPoint = PositionQueueStartingPoint2D;
    type Direction = PositionQueueDirection2D;

    fn cmp_fun(
        point: PositionQueueStartingPoint2D,
        direction: PositionQueueDirection2D,
    ) -> fn(&GridPosition2D, &GridPosition2D) -> Ordering {
        match (point, direction) {
            (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| a.x().cmp(&b.x()))
                }
            }
            (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| b.x().cmp(&a.x()))
                }
            }
            (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| b.y().cmp(&a.y()).reverse())
                }
            }
            (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).reverse().then_with(|| b.x().cmp(&a.x()))
                }
            }
            (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| b.y().cmp(&a.y()))
                }
            }
            (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y()
                        .cmp(&b.y())
                        .reverse()
                        .then_with(|| a.x().cmp(&b.x()).reverse())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{gen::collapse::two_d::*, two_d::TwoDim};

    use super::super::test::test_ordering;

    #[test]
    fn test_ordering_2d_up_left_rowwise() {
        const CASES: &[[u32; 2]] = &[
            [0, 0],
            [1, 0],
            [2, 0],
            [0, 1],
            [1, 1],
            [2, 1],
            [0, 2],
            [1, 2],
            [2, 2],
        ];

        test_ordering::<2, TwoDim>(
            PositionQueueStartingPoint2D::UpLeft,
            PositionQueueDirection2D::Rowwise,
            CASES,
        );
    }

    #[test]
    fn test_ordering_2d_up_right_rowwise() {
        const CASES: &[[u32; 2]] = &[
            [2, 0],
            [1, 0],
            [0, 0],
            [2, 1],
            [1, 1],
            [0, 1],
            [2, 2],
            [1, 2],
            [0, 2],
        ];

        test_ordering::<2, TwoDim>(
            PositionQueueStartingPoint2D::UpRight,
            PositionQueueDirection2D::Rowwise,
            CASES,
        );
    }

    #[test]
    fn test_ordering_2d_down_left_rowwise() {
        const CASES: &[[u32; 2]] = &[
            [0, 2],
            [1, 2],
            [2, 2],
            [0, 1],
            [1, 1],
            [2, 1],
            [0, 0],
            [1, 0],
            [2, 0],
        ];

        test_ordering::<2, TwoDim>(
            PositionQueueStartingPoint2D::DownLeft,
            PositionQueueDirection2D::Rowwise,
            CASES,
        );
    }

    #[test]
    fn test_ordering_2d_down_right_rowwise() {
        const CASES: &[[u32; 2]] = &[
            [2, 2],
            [1, 2],
            [0, 2],
            [2, 1],
            [1, 1],
            [0, 1],
            [2, 0],
            [1, 0],
            [0, 0],
        ];

        test_ordering::<2, TwoDim>(
            PositionQueueStartingPoint2D::DownRight,
            PositionQueueDirection2D::Rowwise,
            CASES,
        );
    }
}
