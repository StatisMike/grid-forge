use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct GridPosition2D {
    x: u32,
    y: u32,
}

impl GridPosition2D {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
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
    pub fn coords(&self) -> [u32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn from_coords(coords: [u32; 2]) -> Self {
        Self {
            x: coords[0],
            y: coords[1],
        }
    }

    #[inline]
    fn coords_sum(&self) -> u32 {
        self.x + self.y
    }

    pub fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self> {
        let mut out = Vec::new();

        for x in a.x.min(b.x)..a.x.max(b.x) + 1 {
            for y in a.y.min(b.y)..a.y.max(b.y) + 1 {
                out.push(Self { x, y });
            }
        }
        out
    }
}

impl Add for GridPosition2D {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for GridPosition2D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.max(rhs.x) - self.x.min(rhs.x),
            y: self.y.max(rhs.y) - self.y.min(rhs.y),
        }
    }
}

impl AddAssign for GridPosition2D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

crate::core::position::macros::__impl_position! {
    position_type: GridPosition2D,
    coord_count: 2,
}

#[cfg(test)]
mod tests {
    use super::GridPosition2D;
    crate::core::position::macros::__impl_position_tests! {
        position_type: GridPosition2D,
        coord_count: 2,
    }

    #[test]
    fn test_2d_compare() {
        const CASES: &[ComparisonTestCase] = &[
            ComparisonTestCase::new(
                GridPosition2D::new(0, 0),
                GridPosition2D::new(0, 0),
                Ordering::Equal,
            ),
            ComparisonTestCase::new(
                GridPosition2D::new(0, 0),
                GridPosition2D::new(1, 0),
                Ordering::Less,
            ),
            ComparisonTestCase::new(
                GridPosition2D::new(0, 0),
                GridPosition2D::new(0, 1),
                Ordering::Less,
            ),
            ComparisonTestCase::new(
                GridPosition2D::new(1, 0),
                GridPosition2D::new(0, 0),
                Ordering::Greater,
            ),
            ComparisonTestCase::new(
                GridPosition2D::new(0, 1),
                GridPosition2D::new(0, 0),
                Ordering::Greater,
            ),
            ComparisonTestCase::new(
                GridPosition2D::new(1, 1),
                GridPosition2D::new(1, 1),
                Ordering::Equal,
            ),
        ];

        compare_test(CASES);
    }

    #[test]
    fn test_2d_order() {
        const CASES: &[OrderingTestCase] = &[OrderingTestCase::new(&[
            GridPosition2D::new(0, 0),
            GridPosition2D::new(0, 2),
            GridPosition2D::new(1, 1),
            GridPosition2D::new(2, 0),
            GridPosition2D::new(1, 2),
            GridPosition2D::new(2, 2),
            GridPosition2D::new(2, 12),
            GridPosition2D::new(12, 2),
            GridPosition2D::new(12, 12),
            GridPosition2D::new(33, 33),
        ])];

        order_test(CASES);
    }

    #[test]
    fn test_2d_add() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition2D::new(0, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 1), GridPosition2D::new(1, 1)],
                GridPosition2D::new(2, 2),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(2, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(0, 1), GridPosition2D::new(1, 1)],
                GridPosition2D::new(1, 2),
            ),
        ];

        add_test(CASES);
    }

    #[test]
    fn test_2d_add_assign() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition2D::new(0, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 1), GridPosition2D::new(1, 1)],
                GridPosition2D::new(2, 2),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(2, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(0, 1), GridPosition2D::new(1, 1)],
                GridPosition2D::new(1, 2),
            ),
        ];

        add_assign_test(CASES);
    }

    #[test]
    fn test_2d_sub() {
        const CASES: &[MathOpTestCase] = &[
            MathOpTestCase::new(
                &[GridPosition2D::new(0, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(1, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 1), GridPosition2D::new(1, 1)],
                GridPosition2D::new(0, 0),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(1, 0), GridPosition2D::new(1, 1)],
                GridPosition2D::new(0, 1),
            ),
            MathOpTestCase::new(
                &[GridPosition2D::new(2, 2), GridPosition2D::new(1, 0)],
                GridPosition2D::new(1, 2),
            ),
        ];

        sub_test(CASES);
    }

    #[test]
    fn test_2d_generate_rect_area() {
        const CASES: &[GenerateRectTestCase] = &[
            GenerateRectTestCase::new(GridPosition2D::new(0, 0), GridPosition2D::new(1, 1)),
            GenerateRectTestCase::new(GridPosition2D::new(1, 1), GridPosition2D::new(10, 15)),
            GenerateRectTestCase::new(GridPosition2D::new(10, 15), GridPosition2D::new(1, 1)),
            GenerateRectTestCase::new(GridPosition2D::new(0, 0), GridPosition2D::new(0, 0)),
        ];

        generate_rect_area_test(CASES);
    }
}
