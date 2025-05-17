use crate::core::three_d::GridPosition3D;

#[derive(Debug, Clone, Copy)]
pub struct GridSize3D {
    x: u32,
    y: u32,
    z: u32,
    x_usize: usize,
    xy_usize: usize,

    center: GridPosition3D,
}

impl GridSize3D {
    #[inline]
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            x,
            y,
            z,
            x_usize: x as usize,
            xy_usize: (x * y) as usize,
            center: GridPosition3D::new(x / 2, y / 2, z / 2),
        }
    }

    #[inline]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> u32 {
        self.z
    }

    pub fn from_2d(z: u32, size: super::two_d::GridSize2D) -> Self {
        Self::new(size.x(), size.y(), z)
    }

    pub fn from_slice(slice: &[u32]) -> Self {
        let [x, y, z] = slice else {
            panic!("slice should have length 3")
        };
        Self::new(*x, *y, *z)
    }

    #[inline]
    pub fn is_position_valid(&self, position: &GridPosition3D) -> bool {
        position.x() < self.x && position.y() < self.y && position.z() < self.z
    }

    #[inline]
    pub fn is_contained_within(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }

    pub fn get_all_possible_positions(&self) -> Vec<GridPosition3D> {
        let mut out = Vec::with_capacity((self.x * self.y * self.z) as usize);

        for x in 0..self.x {
            for y in 0..self.y {
                for z in 0..self.z {
                    out.push(GridPosition3D::new(x, y, z));
                }
            }
        }

        out
    }

    pub fn distance_from_border(&self, position: &GridPosition3D) -> Option<u32> {
        if !self.is_position_valid(position) {
            return None;
        }
        Some(
            *[
                position.x(),
                self.x - position.x() - 1,
                position.y(),
                self.y - position.y() - 1,
                position.z(),
                self.z - position.z() - 1,
            ]
            .iter()
            .min()
            .unwrap(),
        )
    }

    pub fn distance_from_center(&self, position: &GridPosition3D) -> Option<u32> {
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
            })
            .min(if self.center.z() < position.z() {
                position.z() - self.center.z()
            } else {
                self.center.z() - position.z()
            }),
        )
    }

    #[inline]
    pub fn center(&self) -> GridPosition3D {
        self.center
    }

    #[inline]
    pub fn max_tile_count(&self) -> usize {
        (self.x * self.y * self.z) as usize
    }

    #[inline(always)]
    pub fn offset(&self, pos: &GridPosition3D) -> usize {
        // Use precomputed values and leverage wrapping casts
        (pos.x() as usize)
            .wrapping_add((pos.y() as usize).wrapping_mul(self.x_usize))
            .wrapping_add((pos.z() as usize).wrapping_mul(self.xy_usize))
    }

    #[inline(always)]
    pub fn pos_from_offset(&self, offset: usize) -> GridPosition3D {
        let z = offset / self.xy_usize;
        let layer_remainder = offset % self.xy_usize;
        let y = layer_remainder / self.x_usize;
        let x = layer_remainder % self.x_usize;

        GridPosition3D::new(x as u32, y as u32, z as u32)
    }
}

#[cfg(test)]
mod tests {

    use crate::three_d::{GridPosition3D, GridSize3D};

    super::super::macros::__impl_grid_size_tests! {
        size: GridSize3D,
        position: GridPosition3D,
    }

    const POS_OFFSET_CASES: &[SizePosNumTestCase] = &[
        SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 0), 0),
        SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(1, 0, 0), 1),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(0, 1, 0),
            10,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(1, 1, 0),
            11,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(0, 0, 1),
            100,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(1, 0, 1),
            101,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(0, 1, 1),
            110,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(1, 1, 1),
            111,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(10, 10, 10),
            GridPosition3D::new(0, 1, 1),
            110,
        ),
        SizePosNumTestCase::new(GridSize3D::new(33, 33, 33), GridPosition3D::new(0, 0, 0), 0),
        SizePosNumTestCase::new(GridSize3D::new(33, 33, 33), GridPosition3D::new(1, 0, 0), 1),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(0, 1, 0),
            33,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(1, 1, 0),
            34,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(0, 0, 1),
            1089,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(1, 0, 1),
            1090,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(0, 1, 1),
            1122,
        ),
        SizePosNumTestCase::new(
            GridSize3D::new(33, 33, 33),
            GridPosition3D::new(0, 0, 2),
            2178,
        ),
    ];

    #[test]
    fn test_3d_offset() {
        test_offset(POS_OFFSET_CASES);
    }

    #[test]
    fn test_3d_pos_from_offset() {
        test_pos_from_offset(POS_OFFSET_CASES);
    }

    #[test]
    fn test_3d_is_position_valid() {
        const CASES: &[SizePosBoolTestCase] = &[
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 0, 0),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(1, 0, 0),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 1, 0),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(1, 1, 0),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 0, 1),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(1, 0, 1),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 1, 1),
                true,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(12, 0, 0),
                false,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 12, 0),
                false,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(12, 12, 0),
                false,
            ),
            SizePosBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridPosition3D::new(0, 0, 12),
                false,
            ),
        ];
        test_is_position_valid(CASES);
    }

    #[test]
    fn test_3d_is_contained_within() {
        const CASES: &[SizeSizeBoolTestCase] = &[
            SizeSizeBoolTestCase::new(GridSize3D::new(10, 10, 10), GridSize3D::new(5, 5, 5), false),
            SizeSizeBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridSize3D::new(10, 10, 10),
                true,
            ),
            SizeSizeBoolTestCase::new(
                GridSize3D::new(10, 10, 10),
                GridSize3D::new(11, 11, 11),
                true,
            ),
        ];

        test_is_contained_within(CASES);
    }

    const POSITION_COUNT_CASES: &[PosCountTestCase] = &[
        PosCountTestCase::new(GridSize3D::new(10, 10, 10), 1000),
        PosCountTestCase::new(GridSize3D::new(100, 100, 100), 1000000),
        PosCountTestCase::new(GridSize3D::new(33, 33, 33), 35937),
    ];

    #[test]
    fn test_3d_get_all_possible_positions() {
        test_get_all_possible_positions(POSITION_COUNT_CASES);
    }

    #[test]
    fn test_3d_max_tile_count() {
        test_max_tile_count(POSITION_COUNT_CASES);
    }

    #[test]
    fn test_3d_center() {
        const CASES: &[SizePosTestCase] = &[
            SizePosTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(5, 5, 5)),
            SizePosTestCase::new(
                GridSize3D::new(100, 100, 100),
                GridPosition3D::new(50, 50, 50),
            ),
            SizePosTestCase::new(GridSize3D::new(33, 33, 33), GridPosition3D::new(16, 16, 16)),
            SizePosTestCase::new(GridSize3D::new(32, 32, 32), GridPosition3D::new(16, 16, 16)),
        ];

        test_center(CASES);
    }

    #[test]
    fn test_3d_distance_from_center() {
        const CASES: &[SizePosNumTestCase] = &[
            // These distances are not equal because of center rounding!
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 0), 5),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 9, 9), 4),
            // These distances are equal because there is no rounding!
            SizePosNumTestCase::new(GridSize3D::new(9, 9, 9), GridPosition3D::new(0, 0, 0), 4),
            SizePosNumTestCase::new(GridSize3D::new(9, 9, 9), GridPosition3D::new(8, 8, 8), 4),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 9), 4),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 9, 9), 4),
        ];

        test_distance_from_center(CASES);
    }

    #[test]
    fn test_3d_distance_from_border() {
        const CASES: &[SizePosNumTestCase] = &[
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(1, 0, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 1, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(1, 1, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 1), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(1, 0, 1), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 1, 1), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(1, 1, 1), 1),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 0, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 9, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 9, 0), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 0, 9), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 0, 9), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(0, 9, 9), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(9, 9, 9), 0),
            SizePosNumTestCase::new(GridSize3D::new(10, 10, 10), GridPosition3D::new(7, 6, 5), 2),
        ];

        test_distance_from_border(CASES);
    }
}
