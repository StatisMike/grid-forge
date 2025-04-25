use std::{cmp::Ordering, fmt::Debug, ops::{Add, AddAssign, Index, IndexMut, Sub}};

use grid::Order;


pub mod two_dim {
    use std::{cmp::Ordering, ops::{Add, AddAssign, Index, IndexMut, Sub}};

    use super::*;


    /// Two-dimensional space.
    #[derive(Debug, Copy, Clone)]
    pub struct TwoDim {}
    impl super::private::Sealed for TwoDim {}

    impl super::Dimensionality for TwoDim {
        const N: usize = 2;

        type Dir = Directions2D;
        type Size = GridSize2D;
        type Pos = GridPosition2D;
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone)]
    pub enum Directions2D {
        Up = 0,
        Down = 1,
        Left = 2,
        Right = 3,
    }

    impl super::private::Sealed for Directions2D {}

    impl super::Directions<TwoDim> for Directions2D {

        const N: usize = 4;

        fn all() -> &'static [Self] {
            &[Self::Up, Self::Down, Self::Left, Self::Right]
        }

        fn march_step(&self, from: &GridPosition2D, size: &GridSize2D) -> Option<GridPosition2D> {
            let (x_dif, y_dif) = match self {
                Self::Up => {
                    if from.y() == 0 {
                        return None;
                    }
                    (0i32, -1i32)
                }
                Self::Down => {
                    if from.y() + 1 == size.y() {
                        return None;
                    }
                    (0i32, 1i32)
                }
                Self::Left => {
                    if from.x() == 0 {
                        return None;
                    }
                    (-1i32, 0i32)
                }
                Self::Right => {
                    if from.x() + 1 == size.x() {
                        return None;
                    }
                    (1i32, 0i32)
                }
            };
            let (x, y) = (
                (x_dif.wrapping_add_unsigned(from.x())) as u32,
                (y_dif.wrapping_add_unsigned(from.y())) as u32,
            );
            Some(GridPosition2D::new(x, y)) 
        }

        #[inline]
        fn opposite(&self) -> Self {
            match self {
                Self::Up => Self::Down,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
                Self::Right => Self::Left,
            }
        }

        #[inline]
        fn as_idx(&self) -> usize {
            *self as usize
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct GridPosition2D {
        x: u32,
        y: u32,
    }
    impl super::private::Sealed for GridPosition2D {}

    impl super::GridPositionTrait<TwoDim> for GridPosition2D {
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
        pub fn new(x: u32, y: u32) -> Self {
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

    #[derive(Debug, Clone, Copy)]
    pub struct GridSize2D {
        x: u32,
        y: u32,
        x_usize: usize,
        center: (u32, u32),
    }
    impl super::private::Sealed for GridSize2D {}

    impl super::GridSize<TwoDim> for GridSize2D {    
        type Center = (u32, u32);

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
    
        fn distance_from_border(&self, position: &GridPosition2D) -> u32 {
            *[
                position.x(),
                self.x - position.x() - 1,
                position.y(),
                self.y - position.y() - 1,
            ]
            .iter()
            .min()
            .unwrap()
        }
    
        fn distance_from_center(&self, position: &GridPosition2D) -> u32 {
            if self.center.0 < position.x() {
                position.x() - self.center.0
            } else {
                self.center.0 - position.x()
            }
            .min(if self.center.1 < position.y() {
                position.y() - self.center.1
            } else {
                self.center.1 - position.y()
            })
        }
    
        #[inline]
        fn center(&self) -> Self::Center {
            self.center
        }

        #[inline]
        fn tile_count(&self) -> usize {
            (self.x * self.y) as usize
        }

        #[inline(always)]
        fn offset(&self, pos: &GridPosition2D) -> usize {
            pos.x() as usize * self.x_usize + pos.y() as usize
        }

        #[inline(always)]
        fn pos_from_offset(&self, offset: usize) -> GridPosition2D {

            let y = offset / self.x_usize;

            let x = offset % self.x_usize;

            GridPosition2D { x: x as u32, y: y as u32 }

        }
    }

    impl GridSize2D {
        pub fn new(x: u32, y: u32) -> Self {
            Self { x, y , x_usize: x as usize, center: (x / 2, y / 2) }
        }

        pub fn x(&self) -> u32 {
            self.x
        }
        pub fn y(&self) -> u32 {
            self.y
        }
    }

    pub struct DirectionTable2D<T> {
        table: [T; 4],
    }
    impl<T> super::private::Sealed for DirectionTable2D<T> {}
    impl<T> super::DirectionTable<TwoDim, T> for DirectionTable2D<T> {
        type Inner = [T; 4];

        fn new_array(values: [T; 4]) -> Self {
            Self { table: values }
        }

        fn inner(&self) -> &[T; 4] {
            &self.table
        }
    }

    impl <T: Default>Default for DirectionTable2D<T> {
        fn default() -> Self {
            Self {
                table: [T::default(), T::default(), T::default(), T::default()],
            }
        }
    }

    impl <T>Index<Directions2D> for DirectionTable2D<T> {
        type Output = T;

        fn index(&self, index: Directions2D) -> &Self::Output {
            &self.table[index.as_idx()]
        }
    }

    impl <T> IndexMut<Directions2D> for DirectionTable2D<T> {
        fn index_mut(&mut self, index: Directions2D) -> &mut Self::Output {
            &mut self.table[index.as_idx()]
        }
    }

}

pub mod three_dims {
    use std::{cmp::Ordering, ops::{Add, AddAssign, Index, IndexMut, Sub}};

    use super::*;

    /// Three-dimensional space.
    #[derive(Debug, Copy, Clone)]
    pub struct ThreeDim {}
    impl super::private::Sealed for ThreeDim {}

    impl super::Dimensionality for ThreeDim {
        const N: usize = 3;

        type Dir = Directions3D;
        type Size = GridSize3D;
        type Pos = GridPosition3D;
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone)]
    pub enum Directions3D {
        Up,
        Down,
        Left,
        Right,
        Forward,
        Backward,
    }
    impl super::private::Sealed for Directions3D {}

    impl super::Directions<ThreeDim> for Directions3D {

        const N: usize = 6;

        fn all() -> &'static [Self] {
            &[
                Self::Up,
                Self::Down,
                Self::Left,
                Self::Right,
                Self::Forward,
                Self::Backward,
            ]
        }

        fn march_step(&self, from: &GridPosition3D, size: &GridSize3D) -> Option<GridPosition3D> {
            let (x_dif, y_dif, z_dif) = match self {
                Self::Up => {
                    if from.y() == 0 {
                        return None;
                    }
                    (0i32, -1i32, 0i32)
                }
                Self::Down => {
                    if from.y() + 1 == size.y() {
                        return None;
                    }
                    (0i32, 1i32, 0i32)
                }
                Self::Left => {
                    if from.x() == 0 {
                        return None;
                    }
                    (-1i32, 0i32, 0i32)
                }
                Self::Right => {
                    if from.x() + 1 == size.x() {
                        return None;
                    }
                    (1i32, 0i32, 0i32)
                }
                Self::Forward => {
                    if from.z() == 0 {
                        return None;
                    }
                    (0i32, 0i32, -1i32)
                }
                Self::Backward => {
                    if from.z() + 1 == size.z() {
                        return None;
                    }
                    (0i32, 0i32, 1i32)
                }
            };
            let (x, y, z) = (
                (x_dif.wrapping_add_unsigned(from.x())) as u32,
                (y_dif.wrapping_add_unsigned(from.y())) as u32,
                (z_dif.wrapping_add_unsigned(from.z()) as u32),
            );

            Some(GridPosition3D::new(x, y, z))

        }

        fn opposite(&self) -> Self {
            match self {
                Self::Up => Self::Down,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Forward => Self::Backward,
                Self::Backward => Self::Forward,
            }
        }

        #[inline]
        fn as_idx(&self) -> usize {
            *self as usize
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct GridPosition3D {
        x: u32,
        y: u32,
        z: u32,
    }
    impl super::private::Sealed for GridPosition3D {}

    impl super::GridPositionTrait<ThreeDim> for GridPosition3D {
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
        pub fn new(x: u32, y: u32, z: u32) -> Self {
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

    #[derive(Debug, Clone, Copy)]
    pub struct GridSize3D {
        x: u32,
        y: u32,
        z: u32,
        yz: usize,
        z_usize: usize,

        center: (u32, u32, u32),
    }
    impl super::private::Sealed for GridSize3D {}

    impl super::GridSize<ThreeDim> for GridSize3D {
    
        type Center = (u32, u32, u32);

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
    
        fn distance_from_border(&self, position: &GridPosition3D) -> u32 {
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
            .unwrap()
        }
    
        fn distance_from_center(&self, position: &GridPosition3D) -> u32 {
            if self.center.0 < position.x() {
                position.x() - self.center.0
            } else {
                self.center.0 - position.x()
            }
            .min(if self.center.1 < position.y() {
                position.y() - self.center.1
            } else {
                self.center.1 - position.y()
            })
            .min(if self.center.2 < position.z() {
                position.z() - self.center.2
            } else {
                self.center.2 - position.z()
            })
        }
    
        #[inline]
        fn center(&self) -> Self::Center {
            self.center
        }

        #[inline]
        fn tile_count(&self) -> usize {
            (self.x * self.y * self.z) as usize
        }

        #[inline(always)]
        fn offset(&self, pos: &GridPosition3D) -> usize {
            // Use precomputed values and leverage wrapping casts
            (pos.x as usize).wrapping_mul(self.yz)
                .wrapping_add((pos.y as usize).wrapping_mul(self.z_usize))
                .wrapping_add(pos.z as usize)
        }

        #[inline(always)]
        fn pos_from_offset(&self, offset: usize) -> GridPosition3D {
            // Use precomputed values and avoid division
            let x = offset / self.yz;
            let remainder = offset % self.yz;
            let y = remainder / self.z_usize;
            let z = remainder % self.z_usize;
            
            // Use unsafe transmute for zero-cost type conversion
            unsafe {
                GridPosition3D {
                    x: std::mem::transmute(x as u32),
                    y: std::mem::transmute(y as u32),
                    z: std::mem::transmute(z as u32),
                }
            }
        }
    }

    impl GridSize3D {
        #[inline]
        pub fn new(x: u32, y: u32, z: u32) -> Self {
            let yz = (y as usize).checked_mul(z as usize).expect("Dimension overflow");

            Self { x, y, z ,yz, center: (x / 2, y / 2, z / 2), z_usize: z as usize }
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

    pub struct DirectionTable3D<T> {
        table: [T; 6],
    }
    impl<T> super::private::Sealed for DirectionTable3D<T> {}
    impl<T> super::DirectionTable<ThreeDim, T> for DirectionTable3D<T> {
        type Inner = [T; 6];

        fn new_array(values: [T; 6]) -> Self {
            Self { table: values }
        }

        fn inner(&self) -> &[T; 6] {
            &self.table
        }
    }

    impl <T: Default>Default for DirectionTable3D<T> {
        fn default() -> Self {
            Self {
                table: [T::default(), T::default(), T::default(), T::default(), T::default(), T::default()],
            }
        }
    }

    impl <T>Index<Directions3D> for DirectionTable3D<T> {
        type Output = T;

        fn index(&self, index: Directions3D) -> &Self::Output {
            &self.table[index.as_idx()]
        }
    }

    impl <T> IndexMut<Directions3D> for DirectionTable3D<T> {
        fn index_mut(&mut self, index: Directions3D) -> &mut Self::Output {
            &mut self.table[index.as_idx()]
        }
    }
}


/// Trait declaring the number of dimensions in the space
pub trait Dimensionality: private::Sealed + 'static {

    /// Number of dimensions
    const N: usize;

    /// Directions for neighboring cells
    type Dir: Directions<Self>;

    /// Size of the grid
    type Size: GridSize<Self>;

    /// Position of the tile in the grid
    type Pos: GridPositionTrait<Self>;
}

pub trait Directions<D: Dimensionality + ?Sized>: private::Sealed + Sized + Copy + Clone + Debug{

    const N: usize;
    
    fn all() -> &'static [Self];

    fn march_step(&self, from: &D::Pos, size: &D::Size) -> Option<D::Pos>;

    fn opposite(&self) -> Self;

    fn as_idx(&self) -> usize;
}

pub trait GridPositionTrait<D>
where 
D: Dimensionality + ?Sized,
Self: private::Sealed + Ord + PartialOrd + Add<Output = Self> + Sub + AddAssign + Copy + Clone + Debug + Sized + Send + Sync 
{
    type Coords: AsRef<[u32]> + Index<usize, Output = u32>;

    /// Returns the coordinates of the position.
    /// 
    /// It is guranteed that the length of the returned slice is equal to the [Dimensionality::N] of the space.
    fn coords(&self) -> Self::Coords;

    fn from_coords(coords: Self::Coords) -> Self;

    fn in_range(&self, other: &Self, range: u32) -> bool {
        let mut distance = 0;

        for i in 0..D::N {
            distance += self.coords()[i].max(other.coords()[i]) - self.coords()[i].min(other.coords()[i]);
            if distance > range {
                return false;
            }
        }

        true
    }

    fn generate_rect_area(a: &Self, b: &Self) -> Vec<Self>;

    fn filter_positions(pos: &mut Vec<Self>, to_filter: &[Self]) {
        pos.retain(|p| !to_filter.contains(p));
    }
}

pub trait GridSize<D: Dimensionality + ?Sized>: private::Sealed + Clone + Copy
{
    type Center;

    fn is_position_valid(&self, position: &D::Pos) -> bool;

    fn is_contained_within(&self, other: &Self) -> bool;

    fn get_all_possible_positions(&self) -> Vec<D::Pos>;

    fn distance_from_border(&self, position: &D::Pos) -> u32;

    fn distance_from_center(&self, position: &D::Pos) -> u32;

    fn center(&self) -> Self::Center;

    fn tile_count(&self) -> usize;

    fn offset(&self, pos: &D::Pos) -> usize;

    fn pos_from_offset(&self, offset: usize) -> D::Pos;
}

pub trait DirectionTable<D: Dimensionality, T>: private::Sealed + Index<D::Dir> + IndexMut<D::Dir> {
    type Inner: AsRef<[T]> + AsMut<[T]>;

    fn new_array(values: Self::Inner) -> Self;
    fn inner(&self) -> &Self::Inner;
}


mod private {
    pub trait Sealed {}
}
