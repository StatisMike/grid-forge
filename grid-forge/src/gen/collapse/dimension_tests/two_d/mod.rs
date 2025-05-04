#[cfg(test)]
mod position_queue_tests {
    use super::super::test_ordering;
    use crate::r#gen::collapse::two_d::{PositionQueueDirection2D, PositionQueueStartingPoint2D};
    use crate::two_d::TwoDim;

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
