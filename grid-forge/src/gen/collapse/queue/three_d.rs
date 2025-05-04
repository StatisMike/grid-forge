use crate::{core::three_d::ThreeDim, three_d::GridPosition3D};
use std::cmp::Ordering;

use super::position::private::{
    PositionQueueDirection, PositionQueueProcession, PositionQueueStartingPoint,
};

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

impl PositionQueueStartingPoint<ThreeDim> for PositionQueueStartingPoint3D {}

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

impl PositionQueueDirection<ThreeDim> for PositionQueueDirection3D {}

pub struct PositionQueueProcession3D;

impl PositionQueueProcession<ThreeDim> for PositionQueueProcession3D {
    type StartingPoint = PositionQueueStartingPoint3D;
    type Direction = PositionQueueDirection3D;

    fn cmp_fun(
        point: PositionQueueStartingPoint3D,
        direction: PositionQueueDirection3D,
    ) -> fn(&GridPosition3D, &GridPosition3D) -> Ordering {
        match (point, direction) {
            // Front layer comparisons
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Rowwise) => {
                |a, b| {
                    a.z()
                        .cmp(&b.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
            }
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Columnwise) => {
                |a, b| {
                    a.z()
                        .cmp(&b.z())
                        .then_with(|| a.x().cmp(&b.x()))
                        .then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Heightwise) => {
                |a, b| {
                    a.x()
                        .cmp(&b.x())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.z().cmp(&b.z()))
                }
            }

            // Back layer comparisons
            (PositionQueueStartingPoint3D::UpLeftBack, PositionQueueDirection3D::Rowwise) => {
                |a, b| {
                    b.z()
                        .cmp(&a.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
            }

            // Add other combinations following this pattern...
            // This would need 8 starting points × 3 directions = 24 match arms

            // Default case
            _ => |a, b| {
                a.z()
                    .cmp(&b.z())
                    .then_with(|| a.y().cmp(&b.y()))
                    .then_with(|| a.x().cmp(&b.x()))
            },
        }
    }
}
