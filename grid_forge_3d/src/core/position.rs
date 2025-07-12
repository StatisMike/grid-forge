use std::ops::{Add, AddAssign, Sub};

/// Position of the tile in 3D rectangular grid.
#[derive(Debug, Copy, Clone)]
pub struct GridPosition3D {
    x: u32,
    y: u32,
    z: u32,
}

impl GridPosition3D {
    #[inline]
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
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

    #[inline]
    pub fn coords(&self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn from_coords(coords: [u32; 3]) -> Self {
        let [x, y, z] = coords;
        Self { x, y, z }
    }

    #[inline]
    fn coords_sum(&self) -> u32 {
        self.x + self.y + self.z
    }

    pub fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self> {
        let mut out = Vec::new();

        for x in a.x.min(b.x)..a.x.max(b.x) + 1 {
            for y in a.y.min(b.y)..a.y.max(b.y) + 1 {
                for z in a.z.min(b.z)..a.z.max(b.z) + 1 {
                    out.push(Self { x, y, z });
                }
            }
        }
        out
    }
}

impl Add for GridPosition3D {
    type Output = Self;

    fn add(self, rhs: Self) -> GridPosition3D {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for GridPosition3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> GridPosition3D {
        Self {
            x: self.x.max(rhs.x) - self.x.min(rhs.x),
            y: self.y.max(rhs.y) - self.y.min(rhs.y),
            z: self.z.max(rhs.z) - self.z.min(rhs.z),
        }
    }
}

impl AddAssign for GridPosition3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

grid_forge_core::__impl_position!(
    position_type: GridPosition3D,
    coord_count: 3,
);

#[cfg(test)]
mod tests {
    use super::*;
    grid_forge_core::__impl_position_tests! {
        position_type: GridPosition3D,
        coord_count: 3,
    }

    #[test]
    fn test_3d_compare() {
        const CASES: &[ComparisonTestCase] = &[
            ComparisonTestCase::new(
                GridPosition3D::new(0, 0, 0),
                GridPosition3D::new(0, 0, 0),
                Ordering::Equal,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(0, 0, 0),
                GridPosition3D::new(1, 0, 0),
                Ordering::Less,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(0, 0, 0),
                GridPosition3D::new(0, 1, 0),
                Ordering::Less,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(0, 0, 0),
                GridPosition3D::new(0, 0, 1),
                Ordering::Less,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(1, 0, 0),
                GridPosition3D::new(0, 0, 0),
                Ordering::Greater,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(0, 1, 0),
                GridPosition3D::new(0, 0, 0),
                Ordering::Greater,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(0, 0, 1),
                GridPosition3D::new(0, 0, 0),
                Ordering::Greater,
            ),
            ComparisonTestCase::new(
                GridPosition3D::new(1, 1, 1),
                GridPosition3D::new(1, 1, 1),
                Ordering::Equal,
            ),
        ];
        compare_test(CASES);
    }

    #[test]
    fn test_3d_order() {
        const CASES: &[OrderingTestCase] = &[OrderingTestCase::new(&[
            GridPosition3D::new(0, 0, 0),
            GridPosition3D::new(1, 1, 0),
            GridPosition3D::new(2, 0, 0),
            GridPosition3D::new(0, 2, 0),
            GridPosition3D::new(1, 2, 0),
            GridPosition3D::new(2, 2, 2),
            GridPosition3D::new(2, 12, 2),
            GridPosition3D::new(12, 2, 2),
            GridPosition3D::new(12, 12, 12),
            GridPosition3D::new(33, 33, 33),
        ])];

        order_test(CASES);
    }

    #[test]
    fn test_3d_add() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 1, 1), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(2, 2, 2),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(2, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 1, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 2, 1),
            ),
        ];

        add_test(CASES);
    }

    #[test]
    fn test_3d_add_assign() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 1, 1), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(2, 2, 2),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(2, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 1, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 2, 1),
            ),
        ];

        add_assign_test(CASES);
    }

    #[test]
    fn test_3d_sub() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition3D::new(0, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(1, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 1, 1), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(0, 0, 0),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(1, 0, 0), GridPosition3D::new(1, 1, 1)],
                GridPosition3D::new(0, 1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition3D::new(2, 2, 2), GridPosition3D::new(1, 0, 0)],
                GridPosition3D::new(1, 2, 2),
            ),
        ];

        sub_test(CASES);
    }

    #[test]
    fn test_3d_generate_rect_area() {
        const CASES: &[GenerateRectTestCase] = &[
            GenerateRectTestCase::new(GridPosition3D::new(0, 0, 0), GridPosition3D::new(1, 1, 1)),
            GenerateRectTestCase::new(
                GridPosition3D::new(1, 1, 1),
                GridPosition3D::new(10, 15, 20),
            ),
            GenerateRectTestCase::new(
                GridPosition3D::new(10, 15, 20),
                GridPosition3D::new(1, 1, 1),
            ),
            GenerateRectTestCase::new(GridPosition3D::new(0, 0, 0), GridPosition3D::new(0, 0, 0)),
        ];

        generate_rect_area_test(CASES);
    }
}
