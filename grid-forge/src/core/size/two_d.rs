use super::private::*;
use crate::core::two_d::*;

#[derive(Debug, Clone, Copy)]
pub struct GridSize2D {
    x: u32,
    y: u32,
    x_usize: usize,
    center: GridPosition2D,
}
impl Sealed for GridSize2D {}

impl GridSize<TwoDim> for GridSize2D {
    fn from_slice(slice: &[u32]) -> Self {
        let [x, y] = slice else {
            panic!("slice should have length 2")
        };
        Self::new(*x, *y)
    }

    #[inline]
    fn is_position_valid(&self, position: &GridPosition2D) -> bool {
        position.x() < self.x && position.y() < self.y
    }

    #[inline]
    fn is_contained_within(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    fn get_all_possible_positions(&self) -> Vec<GridPosition2D> {
        let mut out = Vec::with_capacity((self.x * self.y) as usize);

        for x in 0..self.x {
            for y in 0..self.y {
                out.push(GridPosition2D::new(x, y));
            }
        }

        out
    }

    fn distance_from_border(&self, position: &GridPosition2D) -> Option<u32> {
        if !self.is_position_valid(position) {
            return None;
        }
        Some(
            *[
                position.x(),
                self.x - position.x() - 1,
                position.y(),
                self.y - position.y() - 1,
            ]
            .iter()
            .min()
            .unwrap(),
        )
    }

    fn distance_from_center(&self, position: &GridPosition2D) -> Option<u32> {
        if !self.is_position_valid(position) {
            return None;
        }
        Some(
            if self.center.x() < position.x() {
                position.x() - self.center.x()
            } else {
                self.center.x() - position.x()
            }
            .min(if self.center.y() < position.y() {
                position.y() - self.center.y()
            } else {
                self.center.y() - position.y()
            }),
        )
    }

    #[inline]
    fn center(&self) -> GridPosition2D {
        self.center
    }

    #[inline]
    fn max_tile_count(&self) -> usize {
        (self.x * self.y) as usize
    }

    #[inline(always)]
    fn offset(&self, pos: &GridPosition2D) -> usize {
        pos.y() as usize * self.x_usize + pos.x() as usize
    }

    #[inline(always)]
    fn pos_from_offset(&self, offset: usize) -> GridPosition2D {
        let y = offset / self.x_usize;

        let x = offset % self.x_usize;

        GridPosition2D::new(x as u32, y as u32)
    }
}

impl GridSize2D {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            x_usize: x as usize,
            center: GridPosition2D::new(x / 2, y / 2),
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use crate::core::size::tests::test_offset;

    use super::*;
    use crate::core::size::tests::*;

    #[test]
    fn test_2d_offset() {
        const CASES: &[Dims1Dims2CountTestCase<2>] = &[
            ([10, 10], [0, 0], 0),
            ([10, 10], [1, 0], 1),
            ([10, 10], [0, 1], 10),
            ([10, 10], [1, 1], 11),
            ([33, 33], [0, 0], 0),
            ([33, 33], [1, 0], 1),
            ([33, 33], [0, 1], 33),
            ([33, 33], [1, 1], 34),
        ];

        test_offset::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_pos_from_offset() {
        const CASES: &[Dims1Dims2CountTestCase<2>] = &[
            ([10, 10], [0, 0], 0),
            ([10, 10], [1, 0], 1),
            ([10, 10], [0, 1], 10),
            ([10, 10], [1, 1], 11),
            ([33, 33], [0, 0], 0),
            ([33, 33], [1, 0], 1),
            ([33, 33], [0, 1], 33),
            ([33, 33], [1, 1], 34),
        ];

        test_pos_from_offset::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_is_position_valid() {
        const CASES: &[Dims1Dims2BoolTestCase<2>] = &[
            ([10, 10], [0, 0], true),
            ([10, 10], [1, 0], true),
            ([10, 10], [0, 1], true),
            ([10, 10], [1, 1], true),
            ([10, 10], [12, 0], false),
            ([10, 10], [0, 12], false),
            ([10, 10], [12, 12], false),
        ];

        test_is_position_valid::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_is_contained_within() {
        const CASES: &[Dims1Dims2BoolTestCase<2>] = &[
            ([10, 10], [5, 5], false),
            ([10, 10], [10, 10], true),
            ([10, 10], [11, 11], true),
        ];

        test_is_contained_within::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_get_all_possible_positions() {
        const CASES: &[DimsCountTestCase<2>] =
            &[([10, 10], 100), ([100, 100], 10000), ([33, 33], 1089)];

        test_get_all_possible_positions::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_max_tile_count() {
        const CASES: &[DimsCountTestCase<2>] =
            &[([10, 10], 100), ([100, 100], 10000), ([33, 33], 1089)];

        test_max_tile_count::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_center() {
        const CASES: &[Dims1Dims2TestCase<2>] = &[
            ([10, 10], [5, 5]),
            ([100, 100], [50, 50]),
            ([33, 33], [16, 16]),
            ([32, 32], [16, 16]),
        ];

        test_center::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_distance_from_center() {
        const CASES: &[Dims1Dims2CountTestCase<2>] = &[
            // These distances are not equal because of center rounding!
            ([10, 10], [0, 0], 5),
            ([10, 10], [9, 9], 4),
            // These distances are equal because there is no rounding!
            ([9, 9], [0, 0], 4),
            ([9, 9], [8, 8], 4),
            ([10, 10], [1, 0], 4),
            ([10, 10], [0, 1], 4),
            ([10, 10], [1, 1], 4),
            ([10, 10], [5, 5], 0),
            ([10, 10], [0, 9], 4),
            ([10, 10], [7, 6], 1),
        ];

        test_distance_from_center::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_distance_from_border() {
        const CASES: &[Dims1Dims2CountTestCase<2>] = &[
            ([10, 10], [0, 0], 0),
            ([10, 10], [1, 0], 0),
            ([10, 10], [0, 1], 0),
            ([10, 10], [1, 1], 1),
            ([10, 10], [9, 0], 0),
            ([10, 10], [0, 9], 0),
            ([10, 10], [9, 9], 0),
            ([10, 10], [7, 6], 2),
        ];

        test_distance_from_border::<2, TwoDim>(CASES);
    }
}
