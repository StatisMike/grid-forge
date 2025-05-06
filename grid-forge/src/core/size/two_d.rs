use super::private::*;
use crate::core::two_d::*;
use crate::core::common::GridSize;

/// Size of the grid in the [`TwoDim`] dimensionality.
/// 
/// It creates bounds for the [`GridMap2D`] and provides it with most of the computational 
/// methods to work with [`GridPosition2D`].
#[derive(Debug, Clone, Copy)]
pub struct GridSize2D {
    x: u32,
    y: u32,
    x_usize: usize,
    center: GridPosition2D,
}

impl Sealed for GridSize2D {}

impl GridSize<TwoDim> for GridSize2D {

    /// Creates a new [`GridSize2D`] from a slice of coordinates.
    /// 
    /// # Panics
    /// Panics if the length of the slice is not equal to the [Dimensionality::N] of the space,
    /// which is 2.
    /// 
    /// ```should_panic
    /// # use grid_forge::common::GridSize;
    /// # use grid_forge::two_d::GridSize2D;
    /// // will panic!
    /// let size = GridSize2D::from_slice(&[10, 10, 10]);
    /// ```
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::GridSize2D;
    /// 
    /// let size = GridSize2D::from_slice(&[10, 10]);
    /// assert_eq!(size.x(), 10);
    /// assert_eq!(size.y(), 10);
    /// ```
    fn from_slice(slice: &[u32]) -> Self {
        let [x, y] = slice else {
            panic!("slice should have length 2")
        };
        Self::new(*x, *y)
    }

    /// Checks if the given position is valid for this size.
    /// 
    /// Position is valid if it is contained within the bounds of the size.
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::{GridSize2D, GridPosition2D};
    /// 
    /// let size = GridSize2D::new(10, 10);
    /// assert!(size.is_position_valid(&GridPosition2D::new(0, 0)));
    /// assert!(size.is_position_valid(&GridPosition2D::new(9, 9)));
    /// assert!(!size.is_position_valid(&GridPosition2D::new(10, 10)));
    /// ```
    #[inline]
    fn is_position_valid(&self, position: &GridPosition2D) -> bool {
        position.x() < self.x && position.y() < self.y
    }

    /// Check if this size is contained within other size.
    /// 
    /// One size is contained within another if the other size is larger than this.
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::GridSize2D;
    /// 
    /// let size = GridSize2D::new(10, 10);
    /// let other = GridSize2D::new(5, 5);
    /// assert!(!size.is_contained_within(&other));
    /// assert!(other.is_contained_within(&size));
    /// ```
    #[inline]
    fn is_contained_within(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    /// Returns a vector of all possible positions for this size.
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::{GridSize2D, GridPosition2D};
    /// 
    /// let size = GridSize2D::new(3, 3);
    /// let positions = size.get_all_possible_positions();
    /// assert_eq!(positions.len(), 9);
    /// 
    /// for pos in positions {
    ///     assert!(size.is_position_valid(&pos));
    /// }
    /// ```
    fn get_all_possible_positions(&self) -> Vec<GridPosition2D> {
        let mut out = Vec::with_capacity((self.x * self.y) as usize);

        for x in 0..self.x {
            for y in 0..self.y {
                out.push(GridPosition2D::new(x, y));
            }
        }

        out
    }

    /// Returns the distance from the border in grid of this [`GridSize2D`] to the given position.
    /// 
    /// If the position is not valid for this size, returns `None`.
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::{GridSize2D, GridPosition2D};
    /// 
    /// let size = GridSize2D::new(10, 10);
    /// assert_eq!(size.distance_from_border(&GridPosition2D::new(0, 0)), Some(0));
    /// assert_eq!(size.distance_from_border(&GridPosition2D::new(9, 9)), Some(0));
    /// assert_eq!(size.distance_from_border(&GridPosition2D::new(10, 10)), None);
    /// ```
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

    /// Returns the center position of this [`GridSize2D`].
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::{GridSize2D, GridPosition2D};
    /// 
    /// let size = GridSize2D::new(10, 10);
    /// assert_eq!(size.center(), GridPosition2D::new(5, 5));    
    /// ```
    #[inline]
    fn center(&self) -> GridPosition2D {
        self.center
    }

    /// Returns the maximum number of tiles that can be placed in this [`GridSize2D`].
    /// 
    /// # Examples
    /// ```
    /// use grid_forge::common::GridSize;
    /// use grid_forge::two_d::GridSize2D;
    /// 
    /// let size = GridSize2D::new(10, 10);
    /// assert_eq!(size.max_tile_count(), 100);
    /// ```
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
    pub const fn new(x: u32, y: u32) -> Self {
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
