use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

use crate::core::three_d::*;

#[derive(Debug, Copy, Clone)]
pub struct GridPosition3D {
    x: u32,
    y: u32,
    z: u32,
}
impl super::private::Sealed for GridPosition3D {}

impl GridPositionTrait<ThreeDim> for GridPosition3D {
    type Coords = [u32; 3];

    #[inline]
    fn coords(&self) -> Self::Coords {
        [self.x, self.y, self.z]
    }

    #[inline]
    fn from_coords(coords: Self::Coords) -> Self {
        let [x, y, z] = coords;
        Self { x, y, z }
    }

    fn from_slice(slice: &[u32]) -> Self {
        let [x, y, z] = slice else {
            panic!("slice should have length 3")
        };
        Self::new(*x, *y, *z)
    }

    fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self> {
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
}

impl Ord for GridPosition3D {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..ThreeDim::N {
            let cmp = self.coords()[i].cmp(&other.coords()[i]);
            if cmp != Ordering::Equal {
                return cmp;
            };
        }
        Ordering::Equal
    }
}

impl PartialOrd for GridPosition3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl PartialEq for GridPosition3D {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..ThreeDim::N {
            if self.coords()[i] != other.coords()[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for GridPosition3D {}

impl Hash for GridPosition3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::core::position::tests::*;
    use crate::core::three_d::*;

    #[test]
    fn test_3d_compare() {
        const CASES: &[ComparisonTestCase<3>] = &[
            ([0, 0, 0], [0, 0, 0], Ordering::Equal),
            ([0, 0, 0], [1, 0, 0], Ordering::Less),
            ([0, 0, 0], [0, 1, 0], Ordering::Less),
            ([0, 0, 0], [0, 0, 1], Ordering::Less),
            ([1, 0, 0], [0, 0, 0], Ordering::Greater),
            ([0, 1, 0], [0, 0, 0], Ordering::Greater),
            ([0, 0, 1], [0, 0, 0], Ordering::Greater),
            ([1, 1, 1], [1, 1, 1], Ordering::Equal),
        ];

        compare_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_order() {
        const CASES: &[OrderingTestCase<3>] = &[&[
            [0, 0, 0],
            [2, 0, 0],
            [0, 2, 0],
            [1, 1, 0],
            [1, 2, 0],
            [33, 33, 33],
            [2, 2, 2],
            [12, 12, 12],
            [12, 2, 2],
            [2, 12, 2],
        ]];

        order_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_add() {
        const CASES: &[MathOpTestCase<3>] = &[
            (&[[0, 0, 0], [1, 1, 1]], [1, 1, 1]),
            (&[[1, 1, 1], [1, 1, 1]], [2, 2, 2]),
            (&[[1, 0, 0], [1, 1, 1]], [2, 1, 1]),
            (&[[0, 1, 0], [1, 1, 1]], [1, 2, 1]),
        ];

        add_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_add_assign() {
        const CASES: &[MathOpTestCase<3>] = &[
            (&[[0, 0, 0], [1, 1, 1]], [1, 1, 1]),
            (&[[1, 1, 1], [1, 1, 1]], [2, 2, 2]),
            (&[[1, 0, 0], [1, 1, 1]], [2, 1, 1]),
            (&[[0, 1, 0], [1, 1, 1]], [1, 2, 1]),
        ];

        add_assign_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_sub() {
        const CASES: &[MathOpTestCase<3>] = &[
            (&[[0, 0, 0], [1, 1, 1]], [1, 1, 1]),
            (&[[1, 1, 1], [1, 1, 1]], [0, 0, 0]),
            (&[[1, 0, 0], [1, 1, 1]], [0, 1, 1]),
            (&[[2, 2, 2], [1, 0, 0]], [1, 2, 2]),
        ];

        sub_test::<3, ThreeDim>(CASES);
    }

    #[test]
    fn test_3d_generate_rect_area() {
        const CASES: &[GenerateRectTestCase<3>] = &[
            ([0, 0, 0], [1, 1, 1]),
            ([1, 1, 1], [10, 15, 20]),
            ([10, 15, 20], [1, 1, 1]),
            ([0, 0, 0], [0, 0, 0]),
        ];

        generate_rect_area_test::<3, ThreeDim>(CASES);
    }
}
