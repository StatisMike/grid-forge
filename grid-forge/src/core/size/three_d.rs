use super::private::*;
use crate::core::three_d::*;

#[derive(Debug, Clone, Copy)]
pub struct GridSize3D {
    x: u32,
    y: u32,
    z: u32,
    x_usize: usize,
    xy_usize: usize,

    center: GridPosition3D,
}
impl Sealed for GridSize3D {}

impl GridSize<ThreeDim> for GridSize3D {
    fn from_slice(slice: &[u32]) -> Self {
        let [x, y, z] = slice else {
            panic!("slice should have length 3")
        };
        Self::new(*x, *y, *z)
    }

    #[inline]
    fn is_position_valid(&self, position: &GridPosition3D) -> bool {
        position.x() < self.x && position.y() < self.y && position.z() < self.z
    }

    #[inline]
    fn is_contained_within(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }

    fn get_all_possible_positions(&self) -> Vec<GridPosition3D> {
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

    fn distance_from_border(&self, position: &GridPosition3D) -> Option<u32> {
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

    fn distance_from_center(&self, position: &GridPosition3D) -> Option<u32> {
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
    fn center(&self) -> GridPosition3D {
        self.center
    }

    #[inline]
    fn max_tile_count(&self) -> usize {
        (self.x * self.y * self.z) as usize
    }

    #[inline(always)]
    fn offset(&self, pos: &GridPosition3D) -> usize {
        // Use precomputed values and leverage wrapping casts
        (pos.x() as usize)
            .wrapping_add((pos.y() as usize).wrapping_mul(self.x_usize))
            .wrapping_add((pos.z() as usize).wrapping_mul(self.xy_usize))
    }

    #[inline(always)]
    fn pos_from_offset(&self, offset: usize) -> GridPosition3D {
        let z = offset / self.xy_usize;
        let layer_remainder = offset % self.xy_usize;
        let y = layer_remainder / self.x_usize;
        let x = layer_remainder % self.x_usize;

        GridPosition3D::new(x as u32, y as u32, z as u32)
    }
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
}

#[cfg(test)]
mod tests {

    use crate::core::size::tests::*;
    use crate::core::three_d::*;

    #[test]
    fn test_3d_offset() {
        const CASES: &[Dims1Dims2CountTestCase<3>] = &[
            ([10, 10, 10], [0, 0, 0], 0),
            ([10, 10, 10], [1, 0, 0], 1),
            ([10, 10, 10], [0, 1, 0], 10),
            ([10, 10, 10], [1, 1, 0], 11),
            ([10, 10, 10], [0, 0, 1], 100),
            ([10, 10, 10], [1, 0, 1], 101),
            ([10, 10, 10], [0, 1, 1], 110),
            ([10, 10, 10], [1, 1, 1], 111),
            ([10, 10, 10], [0, 1, 1], 110),
            ([33, 33, 33], [0, 0, 0], 0),
            ([33, 33, 33], [1, 0, 0], 1),
            ([33, 33, 33], [0, 1, 0], 33),
            ([33, 33, 33], [1, 1, 0], 34),
            ([33, 33, 33], [0, 0, 1], 1089),
            ([33, 33, 33], [1, 0, 1], 1090),
            ([33, 33, 33], [0, 1, 1], 1122),
            ([33, 33, 33], [0, 0, 2], 2178),
        ];

        test_offset::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_pos_from_offset() {
        const CASES: &[Dims1Dims2CountTestCase<3>] = &[
            ([10, 10, 10], [0, 0, 0], 0),
            ([10, 10, 10], [1, 0, 0], 1),
            ([10, 10, 10], [0, 1, 0], 10),
            ([10, 10, 10], [1, 1, 0], 11),
            ([10, 10, 10], [0, 0, 1], 100),
            ([10, 10, 10], [1, 0, 1], 101),
            ([10, 10, 10], [0, 1, 1], 110),
            ([10, 10, 10], [1, 1, 1], 111),
            ([10, 10, 10], [0, 1, 1], 110),
            ([33, 33, 33], [0, 0, 0], 0),
            ([33, 33, 33], [1, 0, 0], 1),
            ([33, 33, 33], [0, 1, 0], 33),
            ([33, 33, 33], [1, 1, 0], 34),
            ([33, 33, 33], [0, 0, 1], 1089),
            ([33, 33, 33], [1, 0, 1], 1090),
            ([33, 33, 33], [0, 1, 1], 1122),
            ([33, 33, 33], [0, 0, 2], 2178),
        ];

        test_pos_from_offset::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_is_position_valid() {
        const CASES: &[Dims1Dims2BoolTestCase<3>] = &[
            ([10, 10, 10], [0, 0, 0], true),
            ([10, 10, 10], [1, 0, 0], true),
            ([10, 10, 10], [0, 1, 0], true),
            ([10, 10, 10], [1, 1, 0], true),
            ([10, 10, 10], [0, 0, 1], true),
            ([10, 10, 10], [1, 0, 1], true),
            ([10, 10, 10], [0, 1, 1], true),
            ([10, 10, 10], [1, 1, 1], true),
            ([10, 10, 10], [0, 1, 1], true),
            ([10, 10, 10], [12, 0, 0], false),
            ([10, 10, 10], [0, 12, 0], false),
            ([10, 10, 10], [12, 12, 0], false),
            ([10, 10, 10], [0, 0, 12], false),
            ([10, 10, 10], [12, 0, 12], false),
            ([10, 10, 10], [0, 12, 12], false),
            ([10, 10, 10], [12, 12, 12], false),
        ];

        test_is_position_valid::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_is_contained_within() {
        const CASES: &[Dims1Dims2BoolTestCase<3>] = &[
            ([10, 10, 10], [5, 5, 5], false),
            ([10, 10, 10], [10, 10, 10], true),
            ([10, 10, 10], [11, 11, 11], true),
        ];

        test_is_contained_within::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_get_all_possible_positions() {
        const CASES: &[DimsCountTestCase<3>] = &[
            ([10, 10, 10], 1000),
            ([100, 100, 100], 1000000),
            ([33, 33, 33], 35937),
        ];

        test_get_all_possible_positions::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_max_tile_count() {
        const CASES: &[DimsCountTestCase<3>] = &[
            ([10, 10, 10], 1000),
            ([100, 100, 100], 1000000),
            ([33, 33, 33], 35937),
        ];

        test_max_tile_count::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_center() {
        const CASES: &[Dims1Dims2TestCase<3>] = &[
            ([10, 10, 10], [5, 5, 5]),
            ([100, 100, 100], [50, 50, 50]),
            ([33, 33, 33], [16, 16, 16]),
            ([32, 32, 32], [16, 16, 16]),
        ];

        test_center::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_distance_from_center() {
        const CASES: &[Dims1Dims2CountTestCase<3>] = &[
            ([10, 10, 10], [0, 0, 0], 5),
            ([10, 10, 10], [9, 9, 9], 4),
            ([10, 10, 10], [0, 0, 9], 4),
            ([10, 10, 10], [9, 9, 9], 4),
            ([10, 10, 10], [7, 6, 5], 0),
        ];

        test_distance_from_center::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_distance_from_border() {
        const CASES: &[Dims1Dims2CountTestCase<3>] = &[
            ([10, 10, 10], [0, 0, 0], 0),
            ([10, 10, 10], [1, 0, 0], 0),
            ([10, 10, 10], [0, 1, 0], 0),
            ([10, 10, 10], [1, 1, 0], 0),
            ([10, 10, 10], [0, 0, 1], 0),
            ([10, 10, 10], [1, 0, 1], 0),
            ([10, 10, 10], [0, 1, 1], 0),
            ([10, 10, 10], [1, 1, 1], 1),
            ([10, 10, 10], [9, 0, 0], 0),
            ([10, 10, 10], [0, 9, 0], 0),
            ([10, 10, 10], [9, 9, 0], 0),
            ([10, 10, 10], [0, 0, 9], 0),
            ([10, 10, 10], [9, 0, 9], 0),
            ([10, 10, 10], [0, 9, 9], 0),
            ([10, 10, 10], [9, 9, 9], 0),
            ([10, 10, 10], [7, 6, 5], 2),
        ];

        test_distance_from_border::<3, ThreeDim>(CASES);
    }
}
