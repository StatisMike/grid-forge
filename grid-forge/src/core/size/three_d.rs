use super::private::*;
use crate::core::three_d::*;

#[derive(Debug, Clone, Copy)]
pub struct GridSize3D {
    x: u32,
    y: u32,
    z: u32,
    yz: usize,
    z_usize: usize,

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
            .wrapping_mul(self.yz)
            .wrapping_add((pos.y() as usize).wrapping_mul(self.z_usize))
            .wrapping_add(pos.z() as usize)
    }

    #[inline(always)]
    fn pos_from_offset(&self, offset: usize) -> GridPosition3D {
        // Use precomputed values and avoid division
        let x = offset / self.yz;
        let remainder = offset % self.yz;
        let y = remainder / self.z_usize;
        let z = remainder % self.z_usize;

        GridPosition3D::new(x as u32, y as u32, z as u32)
    }
}

impl GridSize3D {
    #[inline]
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        let yz = (y as usize)
            .checked_mul(z as usize)
            .expect("Dimension overflow");

        Self {
            x,
            y,
            z,
            yz,
            center: GridPosition3D::new(x / 2, y / 2, z / 2),
            z_usize: z as usize,
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
