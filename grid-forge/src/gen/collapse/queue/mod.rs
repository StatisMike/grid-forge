mod entrophy;
mod position;
mod propagator;

pub(crate) mod three_d;
pub(crate) mod two_d;

pub use entrophy::*;
pub use position::*;
pub use propagator::*;

pub(crate) use position::private::*;

use super::{private::CollapseBounds, tile::CollapsibleTileData};
use crate::core::common::*;

/// Trait shared by objects that handle the selecting algorithm for next tile to collapse within collapse resolvers.
pub trait CollapseQueue<D, Data: CollapsibleTileData<D>>
where
    Self: Default + Sized + private::Sealed<D, Data>,
    D: Dimensionality + CollapseBounds + ?Sized,
{
    /// Pop next position for collapsing.
    fn get_next_position(&mut self) -> Option<D::Pos>;

    /// Initialize the queue based on provided tiles.
    fn initialize_queue(&mut self, tiles: &[(D::Pos, Data)]);

    /// Update internal based on provided tile.
    fn update_queue(&mut self, tile: (D::Pos, &Data));

    /// Checks the current size of the inner queue.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

pub(crate) mod private {
    use rand::Rng;

    use crate::{
        core::common::*, r#gen::collapse::common::*, r#gen::collapse::private::CollapseBounds,
    };

    /// Sealed trait for queues usable in collapse resolvers.
    pub trait Sealed<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>> {
        fn populate_inner_grid<R>(
            &mut self,
            rng: &mut R,
            grid: &mut impl GridMap<D, Data>,
            positions: &[D::Pos],
            options_data: &D::PerOptionData,
        ) where
            R: Rng;

        fn needs_update_after_options_change(&self) -> bool {
            false
        }

        fn propagating(&self) -> bool {
            false
        }
    }
}
