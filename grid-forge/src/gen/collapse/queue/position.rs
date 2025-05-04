use std::{cmp::Ordering, marker::PhantomData};

use crate::{
    core::common::*, r#gen::collapse::common::*, r#gen::collapse::private::CollapseBounds,
};

use private::PositionQueueProcession;
use rand::Rng;

use super::CollapseQueue;

/// A queue that collapses tiles consecutively in a fixed direction, based solely on their position.
pub struct PositionQueue<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
{
    cmp_fun: fn(&D::Pos, &D::Pos) -> Ordering,
    positions: Vec<D::Pos>,
    changed: bool,
    phantom: PhantomData<Data>,
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>> Default
    for PositionQueue<D, Data>
{
    fn default() -> Self {
        Self {
            cmp_fun: D::PositionQueueProcession::cmp_fun_default(),
            positions: Vec::new(),
            changed: false,
            phantom: PhantomData,
        }
    }
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    PositionQueue<D, Data>
{
    pub fn new(
        starting: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<
            D,
        >>::StartingPoint,
        direction: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<D>>::Direction,
    ) -> Self {
        Self {
            cmp_fun: D::PositionQueueProcession::cmp_fun(starting, direction),
            ..Default::default()
        }
    }

    pub fn sort_elements(&mut self) {
        self.positions.sort_by(self.cmp_fun);
        self.positions.reverse();
    }
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    CollapseQueue<D, Data> for PositionQueue<D, Data>
{
    fn get_next_position(&mut self) -> Option<D::Pos> {
        if self.changed {
            self.sort_elements()
        }
        self.positions.pop()
    }

    fn initialize_queue(&mut self, tiles: &[(D::Pos, Data)]) {
        for element in tiles {
            self.update_queue((element.0, &element.1))
        }
    }

    fn update_queue(&mut self, tile: (D::Pos, &Data)) {
        if !self.positions.contains(&tile.0) {
            self.positions.push(tile.0);
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

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    super::private::Sealed<D, Data> for PositionQueue<D, Data>
{
    fn populate_inner_grid<R: Rng>(
        &mut self,
        _rng: &mut R,
        grid: &mut impl GridMap<D, Data>,
        positions: &[D::Pos],
        options_data: &D::PerOptionData,
    ) {
        let tiles = Data::new_from_frequency(positions, options_data);
        self.initialize_queue(&tiles);
        for tile in tiles {
            grid.insert_data(&tile.0, tile.1);
        }
    }
}

pub(crate) mod private {
    use std::cmp::Ordering;

    use crate::core::common::Dimensionality;

    pub trait PositionQueueProcession<D: Dimensionality> {
        type StartingPoint: PositionQueueStartingPoint<D>;
        type Direction: PositionQueueDirection<D>;

        fn cmp_fun(
            point: Self::StartingPoint,
            direction: Self::Direction,
        ) -> fn(&D::Pos, &D::Pos) -> Ordering;

        fn cmp_fun_default() -> fn(&D::Pos, &D::Pos) -> Ordering {
            Self::cmp_fun(Self::StartingPoint::default(), Self::Direction::default())
        }
    }

    pub trait PositionQueueStartingPoint<D: Dimensionality>: Eq + PartialEq + Default {}

    pub trait PositionQueueDirection<D: Dimensionality>: Eq + PartialEq + Default {}
}