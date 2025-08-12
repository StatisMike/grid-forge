use crate::core::position::GridPosition2D;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct GridSize2D {
    x: u32,
    y: u32,
    x_usize: usize,
    center: GridPosition2D,
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
    pub fn from_slice(slice: &[u32]) -> Self {
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
    pub fn is_position_valid(&self, position: &GridPosition2D) -> bool {
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
    pub fn is_contained_within(&self, other: &Self) -> bool {
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
    pub fn get_all_possible_positions(&self) -> Vec<GridPosition2D> {
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
    pub fn distance_from_border(&self, position: &GridPosition2D) -> Option<u32> {
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

    pub fn distance_from_center(&self, position: &GridPosition2D) -> Option<u32> {
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
    pub fn center(&self) -> GridPosition2D {
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
    pub fn max_tile_count(&self) -> usize {
        (self.x * self.y) as usize
    }

    #[inline(always)]
    pub fn offset(&self, pos: &GridPosition2D) -> usize {
        pos.y() as usize * self.x_usize + pos.x() as usize
    }

    #[inline(always)]
    pub fn pos_from_offset(&self, offset: usize) -> GridPosition2D {
        let y = offset / self.x_usize;

        let x = offset % self.x_usize;

        GridPosition2D::new(x as u32, y as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{GridPosition2D, GridSize2D};

    grid_forge_core::__impl_grid_size_tests! {
        size: GridSize2D,
        position: GridPosition2D,
    }

    #[test]
    fn test_2d_offset() {
        const CASES: &[SizePosNumTestCase] = &[
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 0), 1),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 1), 10),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 1), 11),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(0, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(1, 0), 1),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(0, 1), 33),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(1, 1), 34),
        ];

        test_offset(CASES);
    }

    #[test]
    fn test_2d_pos_from_offset() {
        const CASES: &[SizePosNumTestCase] = &[
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 0), 1),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 1), 10),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 1), 11),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(0, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(1, 0), 1),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(0, 1), 33),
            SizePosNumTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(1, 1), 34),
        ];

        test_pos_from_offset(CASES);
    }

    #[test]
    fn test_2d_is_position_valid() {
        const CASES: &[SizePosBoolTestCase] = &[
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 0), true),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 0), true),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 1), true),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 1), true),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(12, 0), false),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 12), false),
            SizePosBoolTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(12, 12), false),
        ];

        test_is_position_valid(CASES);
    }

    #[test]
    fn test_2d_is_contained_within() {
        const CASES: &[SizeSizeBoolTestCase] = &[
            SizeSizeBoolTestCase::new(GridSize2D::new(10, 10), GridSize2D::new(5, 5), false),
            SizeSizeBoolTestCase::new(GridSize2D::new(10, 10), GridSize2D::new(10, 10), true),
            SizeSizeBoolTestCase::new(GridSize2D::new(10, 10), GridSize2D::new(11, 11), true),
        ];

        test_is_contained_within(CASES);
    }

    #[test]
    fn test_2d_get_all_possible_positions() {
        const CASES: &[PosCountTestCase] = &[
            PosCountTestCase::new(GridSize2D::new(10, 10), 100),
            PosCountTestCase::new(GridSize2D::new(100, 100), 10000),
            PosCountTestCase::new(GridSize2D::new(33, 33), 1089),
        ];

        test_get_all_possible_positions(CASES);
    }

    #[test]
    fn test_2d_max_tile_count() {
        const CASES: &[PosCountTestCase] = &[
            PosCountTestCase::new(GridSize2D::new(10, 10), 100),
            PosCountTestCase::new(GridSize2D::new(100, 100), 10000),
            PosCountTestCase::new(GridSize2D::new(33, 33), 1089),
        ];

        test_max_tile_count(CASES);
    }

    #[test]
    fn test_2d_center() {
        const CASES: &[SizePosTestCase] = &[
            SizePosTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(5, 5)),
            SizePosTestCase::new(GridSize2D::new(100, 100), GridPosition2D::new(50, 50)),
            SizePosTestCase::new(GridSize2D::new(33, 33), GridPosition2D::new(16, 16)),
            SizePosTestCase::new(GridSize2D::new(32, 32), GridPosition2D::new(16, 16)),
        ];

        test_center(CASES);
    }

    #[test]
    fn test_2d_distance_from_center() {
        const CASES: &[SizePosNumTestCase] = &[
            // These distances are not equal because of center rounding!
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 0), 5),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(9, 9), 4),
            // These distances are equal because there is no rounding!
            SizePosNumTestCase::new(GridSize2D::new(9, 9), GridPosition2D::new(0, 0), 4),
            SizePosNumTestCase::new(GridSize2D::new(9, 9), GridPosition2D::new(8, 8), 4),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 0), 4),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 1), 4),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 1), 4),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(5, 5), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 9), 4),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(7, 6), 1),
        ];

        test_distance_from_center(CASES);
    }

    #[test]
    fn test_2d_distance_from_border() {
        const CASES: &[SizePosNumTestCase] = &[
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 1), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(1, 1), 1),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(9, 0), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(0, 9), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(9, 9), 0),
            SizePosNumTestCase::new(GridSize2D::new(10, 10), GridPosition2D::new(7, 6), 2),
        ];

        test_distance_from_border(CASES);
    }
}
