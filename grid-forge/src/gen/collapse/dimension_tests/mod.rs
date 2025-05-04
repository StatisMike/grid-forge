use crate::core::common::Dimensionality;

use super::{common::CollapseBounds, queue::PositionQueueProcession};
use crate::core::common::GridPositionTrait;

mod three_d;
mod two_d;

// Test helper function
pub fn test_ordering<const DIM: usize, D: Dimensionality + CollapseBounds + ?Sized>(
    start: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<D>>::StartingPoint,
    dir: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<D>>::Direction,
    expected: &[[u32; DIM]],
) {
    let comparator = D::PositionQueueProcession::cmp_fun(start, dir);
    let mut ul_coords = Vec::new();
    let mut ld_coords = Vec::new();
    for _ in 0..D::N {
        ul_coords.push(0u32);
        ld_coords.push(2u32);
    }
    let mut positions = D::Pos::generate_rect_area(
        &D::Pos::from_slice(&ul_coords),
        &D::Pos::from_slice(&ld_coords),
    );

    positions.sort_by(comparator);

    let mut expected_pos = Vec::new();
    for pos in expected {
        expected_pos.push(D::Pos::from_slice(pos));
    }

    assert_eq!(positions, expected_pos);
}
