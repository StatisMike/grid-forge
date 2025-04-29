use std::cmp::Ordering;

use crate::{core::common::*, r#gen::collapse::{option::private::PerOptionData, CollapsibleTileData}};

use private::{PositionQueueDirection, PositionQueueProcession, PositionQueueStartingPoint};
use rand::Rng;

use super::CollapseQueue;

/// A queue that collapses tiles consecutively in a fixed direction, based solely on their position.
#[derive(Default)]
pub struct PositionQueue<D: Dimensionality> {
    cmp_fun: fn(&D::Pos, &D::Pos) -> Ordering,
    positions: Vec<D::Pos>,
    changed: bool,
}

impl <D: Dimensionality> PositionQueue<D> {
    pub fn new(starting: impl PositionQueueStartingPoint<D>, direction: PositionQueueDirection<D>) -> Self { 
        Self {
            cmp_fun: PositionQueueProcession::<D>::cmp_fun(starting, direction),
            ..Default::default()
        }
    }

    pub fn sort_elements(&mut self) {
        self.positions.sort_by(self.cmp_fun);
        self.positions.reverse();
    }
}

impl <D: Dimensionality> CollapseQueue<D> for PositionQueue<D> {
    fn get_next_position(&mut self) -> Option<D::Pos> {
        if self.changed {
            self.sort_elements()
        }
        self.positions.pop()
    }

    fn initialize_queue<Data: CollapsibleTileData<D>>(&mut self, tiles: &[(D::Pos, Data)]) {
        for tile in tiles {
            self.update_queue(tile)
        }
    }

    fn update_queue<Tile, Data>(&mut self, tile: &Tile)
    where
        Tile: TileContainer + AsRef<Data>,
        Data: CollapsibleTileData,
    {
        if !self.positions.contains(&tile.grid_position()) {
            self.positions.push(tile.grid_position());
        }
        self.changed = true;
    }

    fn len(&self) -> usize {
        self.positions.len()
    }

    fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}

pub (crate) mod two_d {

    use std::cmp::Ordering;

    use crate::{core::two_d::TwoDim, two_d::GridPosition2D};

    /// Enum defining the starting point of the collapse wave.
    #[derive(Default, Eq, PartialEq)]
    pub enum PositionQueueStartingPoint2D {
        #[default]
        /// Starts at the `(0, 0)` position.
        UpLeft,
        /// Starts at the `(0, max)` position.
        UpRight,
        /// Starts at the `(max, 0)` position.
        DownLeft,
        /// Starts at the `(max, max)` position.
        DownRight,
    }

    impl super::private::PositionQueueStartingPoint<TwoDim> for PositionQueueStartingPoint2D {}

    /// Enum defining the direction in which the tiles will be collapsed.
    #[derive(Default, Eq, PartialEq)]
    pub enum PositionQueueDirection2D {
        #[default]
        /// Collapses tiles in a rowwise fashion.
        Rowwise,
        /// Collapses tiles in a columnwise fashion.
        Columnwise,
    }

    impl super::private::PositionQueueDirection<TwoDim> for PositionQueueDirection2D {}

    pub struct PositionQueueProcession2D;

    impl super::private::PositionQueueProcession<TwoDim> for PositionQueueProcession2D {
        type StartingPoint = PositionQueueStartingPoint2D;
        type Direction = PositionQueueDirection2D;

        fn cmp_fun(point: PositionQueueStartingPoint2D, direction: PositionQueueDirection2D) -> fn(&GridPosition2D, &GridPosition2D) -> Ordering {
            match (point, direction) {
                (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Rowwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.y().cmp(*b.y()).then_with(|| a.x().cmp(*b.x()))
                    }
                }
                (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Columnwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.x().cmp(*b.x()).then_with(|| a.y().cmp(*b.y()))
                    }
                }
                (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Columnwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.x().cmp(&b.x()).reverse().then_with(|| a.y().cmp(&b.y()))
                    }
                },
                (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Rowwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.y().cmp(&b.y()).then_with(|| a.x().cmp(&b.x()))
                    }
                },
                (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Columnwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.x().cmp(&b.x()).then_with(|| b.y().cmp(&a.y()))
                    }
                },
                (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Rowwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.y().cmp(&b.y()).reverse().then_with(|| b.x().cmp(&a.x()))
                    }
                },
                (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Columnwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.x().cmp(&b.x()).reverse().then_with(|| b.y().cmp(&a.y()))
                    }
                },
                (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Rowwise) => {
                    |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                        a.y().cmp(&b.y()).reverse().then_with(|| a.x().cmp(&b.x()).reverse())
                    }
                }
            }
        }
    }
}

pub (crate) mod three_d {
    use std::cmp::Ordering;
    use crate::{core::three_d::ThreeDim, three_d::GridPosition3D};

    /// Enum defining the starting point of the collapse wave in 3D space
    #[derive(Default, Eq, PartialEq)]
    pub enum PositionQueueStartingPoint3D {
        #[default]
        /// Starts at the `(0, 0, 0)` position
        UpLeftFront,
        /// Starts at the `(0, max, 0)` position
        UpRightFront,
        /// Starts at the `(max, 0, 0)` position
        DownLeftFront,
        /// Starts at the `(max, max, 0)` position
        DownRightFront,
        /// Starts at the `(0, 0, max)` position
        UpLeftBack,
        /// Starts at the `(0, max, max)` position
        UpRightBack,
        /// Starts at the `(max, 0, max)` position
        DownLeftBack,
        /// Starts at the `(max, max, max)` position
        DownRightBack,
    }

    impl super::private::PositionQueueStartingPoint<ThreeDim> for PositionQueueStartingPoint3D {}

    /// Enum defining the direction in which the tiles will be collapsed in 3D
    #[derive(Default, Eq, PartialEq)]
    pub enum PositionQueueDirection3D {
        #[default]
        /// Collapses tiles in a rowwise fashion (X-axis primary)
        Rowwise,
        /// Collapses tiles in a columnwise fashion (Y-axis primary)
        Columnwise,
        /// Collapses tiles in a heightwise fashion (Z-axis primary)
        Heightwise,
    }

    impl super::private::PositionQueueDirection<ThreeDim> for PositionQueueDirection3D {}

    pub struct PositionQueueProcession3D;

    impl super::private::PositionQueueProcession<ThreeDim> for PositionQueueProcession3D {
        type StartingPoint = PositionQueueStartingPoint3D;
        type Direction = PositionQueueDirection3D;

        fn cmp_fun(
            point: PositionQueueStartingPoint3D,
            direction: PositionQueueDirection3D
        ) -> fn(&GridPosition3D, &GridPosition3D) -> Ordering {
            match (point, direction) {
                // Front layer comparisons
                (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Rowwise) => {
                    |a, b| a.z().cmp(&b.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
                (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Columnwise) => {
                    |a, b| a.z().cmp(&b.z())
                        .then_with(|| a.x().cmp(&b.x()))
                        .then_with(|| a.y().cmp(&b.y()))
                }
                (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Heightwise) => {
                    |a, b| a.x().cmp(&b.x())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.z().cmp(&b.z()))
                }

                // Back layer comparisons
                (PositionQueueStartingPoint3D::UpLeftBack, PositionQueueDirection3D::Rowwise) => {
                    |a, b| b.z().cmp(&a.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
                
                // Add other combinations following this pattern...
                // This would need 8 starting points × 3 directions = 24 match arms
                
                // Default case
                _ => |a, b| a.z().cmp(&b.z())
                    .then_with(|| a.y().cmp(&b.y()))
                    .then_with(|| a.x().cmp(&b.x()))
            }
        }
    }
}

impl <D: Dimensionality> super::private::Sealed<D> for PositionQueue<D> {
    fn populate_inner_grid<R: Rng, Data: CollapsibleTileData<D>>(
        &mut self,
        _rng: &mut R,
        grid: &mut impl GridMap<D, Data>,
        positions: &[D::Pos],
        options_data: &PerOptionData<D>,
    ) {
        let tiles = Data::new_from_frequency(positions, options_data);
        self.initialize_queue(&tiles);
        for tile in tiles {
            grid.insert_tile(tile);
        }
    }
}

pub (crate) mod private {
    use std::cmp::Ordering;

    use crate::core::common::Dimensionality;

    pub trait PositionQueueProcession<D: Dimensionality> {
        type StartingPoint: PositionQueueStartingPoint<D>;
        type Direction: PositionQueueDirection<D>;

        fn cmp_fun(point: Self::StartingPoint, direction: Self::Direction) -> fn(&D::Pos, &D::Pos) -> Ordering;

        fn cmp_fun_default() -> fn(&D::Pos, &D::Pos) -> Ordering {
            Self::cmp_fun(Self::StartingPoint::default(), Self::Direction::default())
        }
    }

    pub trait PositionQueueStartingPoint<D: Dimensionality>: Eq + PartialEq + Default {}

    pub trait PositionQueueDirection<D: Dimensionality>: Eq + PartialEq + Default {}


}

#[cfg(test)]
mod test {
    use crate::{core::common::*, r#gen::collapse::position::private::PositionQueueProcession};

    use super::private::{PositionQueueDirection, PositionQueueStartingPoint};

    // Test helper function
    fn test_ordering<D: Dimensionality> (start: impl PositionQueueStartingPoint<D>, dir: impl PositionQueueDirection<D>, expected: [&[u32]]) {
        let comparator = PositionQueueProcession::<D>::cmp_fun(start, dir);
        let mut positions = D::Pos::generate_rect_area(
            &D::Pos::from_coords(&[0; D::N]),
            &D::Pos::from_coords(&[2; D::N]),
        );
        
        positions.sort_by(comparator);
        let actual: Vec<(u32, u32)> = positions.iter()
            .map(|p| (p.x(), p.y()))
            .collect();

        assert_eq!(actual, expected);
    }
}
