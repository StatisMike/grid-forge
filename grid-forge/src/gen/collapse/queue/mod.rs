pub(crate) mod entrophy;
pub(crate) mod position;
mod propagator;

pub use entrophy::EntrophyQueue;
pub use position::*;
pub(crate) use propagator::*;

use super::tile::CollapsibleTileData;
use crate::core::common::*;

/// Trait shared by objects that handle the selecting algorithm for next tile to collapse within collapse resolvers.
pub trait CollapseQueue<D>
where
    Self: Default + Sized + private::Sealed<D>,
    D: Dimensionality,
{
    /// Pop next position for collapsing.
    fn get_next_position(&mut self) -> Option<D::Pos>;

    /// Initialize the queue based on provided tiles.
    fn initialize_queue<Data: CollapsibleTileData<D>>(&mut self, tiles: &[(D::Pos, Data)]);

    /// Update internal based on provided tile.
    fn update_queue<Data: CollapsibleTileData<D>>(&mut self, tile: (D::Pos, Data))
    where
        Data: CollapsibleTileData<D>;

    /// Checks the current size of the inner queue.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

pub(crate) mod private {
    use rand::Rng;

    use crate::{core::common::*, r#gen::collapse::{option::private::PerOptionData, CollapsibleTileData}};

    /// Sealed trait for queues usable in collapse resolvers.
    pub trait Sealed<D: Dimensionality> {
        fn populate_inner_grid<R, Data>(
            &mut self,
            rng: &mut R,
            grid: &mut impl GridMap<D, Data>,
            positions: &[D::Pos],
            options_data: &impl PerOptionData<D>,
        ) where
            R: Rng,
            Data: CollapsibleTileData<D>;

        fn needs_update_after_options_change(&self) -> bool {
            false
        }

        fn propagating(&self) -> bool {
            false
        }
    }
}
