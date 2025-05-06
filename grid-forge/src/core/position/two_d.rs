use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

use crate::core::two_d::*;
use crate::core::common::{GridPosition, Dimensionality};

/// Position of the tile in the [`TwoDim`] dimensionality.
/// 
/// It contains two coordinates, `x` for the horizontal axis and `y` for the vertical.
/// 
#[derive(Debug, Copy, Clone)]
pub struct GridPosition2D {
    x: u32,
    y: u32,
}
impl super::private::Sealed for GridPosition2D {}

impl GridPosition<TwoDim> for GridPosition2D {
    type Coords = [u32; 2];

    #[inline]
    fn coords(&self) -> Self::Coords {
        [self.x, self.y]
    }

    #[inline]
    fn from_coords(coords: Self::Coords) -> Self {
        let [x, y] = coords;
        Self { x, y }
    }

    #[inline]
    fn from_slice(slice: &[u32]) -> Self {
        let [x, y] = slice else {
            panic!("slice should have length 2")
        };
        Self::new(*x, *y)
    }

    fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self> {
        let mut out = Vec::new();

        for x in a.x.min(b.x)..a.x.max(b.x) + 1 {
            for y in a.y.min(b.y)..a.y.max(b.y) + 1 {
                out.push(Self { x, y });
            }
        }
        out
    }
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
}

impl Ord for GridPosition2D {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..TwoDim::N {
            let cmp = self.coords()[i].cmp(&other.coords()[i]);
            if cmp != Ordering::Equal {
                return cmp;
            };
        }
        Ordering::Equal
    }
}

impl PartialOrd for GridPosition2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl PartialEq for GridPosition2D {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..TwoDim::N {
            if self.coords()[i] != other.coords()[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for GridPosition2D {}

impl Hash for GridPosition2D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::core::position::tests::*;
    use crate::core::two_d::*;

    #[test]
    fn test_2d_compare() {
        const CASES: &[ComparisonTestCase<2>] = &[
            ([0, 0], [0, 0], Ordering::Equal),
            ([0, 0], [1, 0], Ordering::Less),
            ([0, 0], [0, 1], Ordering::Less),
            ([1, 0], [0, 0], Ordering::Greater),
            ([0, 1], [0, 0], Ordering::Greater),
            ([1, 1], [1, 1], Ordering::Equal),
        ];

        compare_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_order() {
        const CASES: &[OrderingTestCase<2>] = &[&[
            [0, 0],
            [2, 0],
            [0, 2],
            [1, 1],
            [1, 2],
            [33, 33],
            [2, 2],
            [12, 12],
            [12, 2],
            [2, 12],
        ]];

        order_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_add() {
        const CASES: &[MathOpTestCase<2>] = &[
            (&[[0, 0], [1, 1]], [1, 1]),
            (&[[1, 1], [1, 1]], [2, 2]),
            (&[[1, 0], [1, 1]], [2, 1]),
            (&[[0, 1], [1, 1]], [1, 2]),
        ];

        add_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_add_assign() {
        const CASES: &[MathOpTestCase<2>] = &[
            (&[[0, 0], [1, 1]], [1, 1]),
            (&[[1, 1], [1, 1]], [2, 2]),
            (&[[1, 0], [1, 1]], [2, 1]),
            (&[[0, 1], [1, 1]], [1, 2]),
        ];

        add_assign_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_sub() {
        const CASES: &[MathOpTestCase<2>] = &[
            (&[[0, 0], [1, 1]], [1, 1]),
            (&[[1, 1], [1, 1]], [0, 0]),
            (&[[1, 0], [1, 1]], [0, 1]),
            (&[[2, 2], [1, 0]], [1, 2]),
        ];

        sub_test::<2, TwoDim>(CASES);
    }

    #[test]
    fn test_2d_generate_rect_area() {
        const CASES: &[GenerateRectTestCase<2>] = &[
            ([0, 0], [1, 1]),
            ([1, 1], [10, 15]),
            ([10, 15], [1, 1]),
            ([0, 0], [0, 0]),
        ];

        generate_rect_area_test::<2, TwoDim>(CASES);
    }
}
