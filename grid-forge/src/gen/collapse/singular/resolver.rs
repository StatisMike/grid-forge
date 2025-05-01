use std::any::Any;
use std::marker::PhantomData;

use crate::core::common::*;
use crate::r#gen::collapse::private::CollapseBounds;
use crate::r#gen::collapse::CollapsibleGrid;
use crate::id::*;

use crate::gen::collapse::grid::private::CommonCollapsibleGrid;
use crate::gen::collapse::{
    CollapsibleTileData, EntrophyQueue, PropagateItem, Propagator,
};

use crate::gen::collapse::error::{CollapseError, CollapseErrorKind};
use crate::gen::collapse::queue::CollapseQueue;

use rand::Rng;

use super::subscriber::Subscriber;

/// Resolver of the singular collapsible procedural algorithm.
///
/// It uses either [`EntrophyQueue`] or [`PositionQueue`] to process the option collapsing process of the [`CollapsibleTileGrid`],
/// additionally providing an option to subscribe to the collapse process via [`singular::Subscriber`](Subscriber).
pub struct Resolver<D: Dimensionality, CB: CollapseBounds<D>, Data>
where
    Data: IdentifiableTileData,
{
    subscriber: Option<Box<dyn Subscriber<D>>>,
    tile_type: PhantomData<(CB, Data)>,
}

impl<D, CB: CollapseBounds<D>, Data> Default for Resolver<D, CB, Data>
where
    D: Dimensionality,
    Data: IdentifiableTileData,
{
    fn default() -> Self {
        Self {
            subscriber: None,
            tile_type: PhantomData,
        }
    }
}

impl<D, CB: CollapseBounds<D>, Data> Resolver<D, CB, Data>
where
    D: Dimensionality,
    Data: IdentifiableTileData,
{
    /// Attach a subscriber to the resolver. The subscriber will be notified of each tile being collapsed.
    pub fn with_subscriber(mut self, subscriber: Box<dyn Subscriber<D>>) -> Self {
        self.subscriber = Some(subscriber);
        self
    }

    /// Retrieve the subscriber attached to the resolver.
    pub fn retrieve_subscriber(&mut self) -> Option<Box<dyn Subscriber<D>>> {
        self.subscriber.take()
    }

    /// Collapse the [`CollapsibleTileGrid`] using [`EntrophyQueue`].
    ///
    /// Contrary to [`generate_position`](Self::generate_position), this method don't require providing the precreated
    /// queue, as it don't allow for any configuration - it will always collapse the tile with the lowest entrophy next.
    ///
    /// # Arguments
    /// * `grid` - [`CollapsibleTileGrid`] to be processed. All non-collapsed tiles provided within will be
    ///   removed on the beginning of the process.
    /// * `rng` - [`Rng`] to be used for randomness.
    /// * `positions` - [`GridPosition`]s to be collapsed. If any collapsed tile is present inside the provided `grid`
    ///   at one of the positions provided, the tile will be overwritten with uncollapsed one.
    ///
    /// Provided `grid` can be translated into either a [`CollapsedGrid`](crate::gen::collapse::grid::CollapsedGrid)
    /// or [`GridMap2D`](crate::map::GridMap2D) of some [`IdentifiableTileData`] after the process.
    pub fn generate_entrophy<CG: CollapsibleGrid<D, CB, Data>, R>(
        &mut self,
        grid: &mut CG,
        rng: &mut R,
        positions: &[D::Pos],
    ) -> Result<(), CollapseError<D>>
    where
        R: Rng,
    {
        use crate::gen::collapse::queue::private::Sealed as _;
        use crate::gen::collapse::tile::private::CommonCollapsibleTileData as _;

        let mut iter = 0;
        let mut queue = EntrophyQueue::default();
        let mut propagator = Propagator::default();

        if let Some(subscriber) = self.subscriber.as_mut() {
            subscriber.on_generation_start();
        }

        grid.remove_uncollapsed();

        let option_data = grid._option_data().clone();

        queue.populate_inner_grid(rng, grid._grid_mut(), positions, &option_data);

        for initial_propagate in grid._get_initial_propagate_items(positions) {
            propagator.push_propagate(initial_propagate);
        }

        CollapseError::from_result(
            propagator.propagate(grid._grid_mut(), &option_data, &mut queue),
            CollapseErrorKind::Init,
            iter,
        )?;

        // Progress with collapse.
        while let Some(collapse_position) = queue.get_next_position() {
            let (_, to_collapse) = grid
                ._grid_mut()
                .get_mut_tile_at_position(&collapse_position)
                .unwrap();
            // skip collapsed;
            if to_collapse.is_collapsed() {
                continue;
            }
            if !to_collapse.has_compatible_options() {
                return Err(CollapseError::new(
                    collapse_position,
                    CollapseErrorKind::Collapse,
                    iter,
                ));
            }
            let removed_options = to_collapse.collapse_gather_removed(rng, &option_data);

            let collapsed_idx = to_collapse.collapse_idx().unwrap();
            if let Some(subscriber) = self.subscriber.as_mut() {
                let collapsed_id = grid
                    ._option_data()
                    .get_tile_type_id(&collapsed_idx)
                    .unwrap();
                subscriber
                    .as_mut()
                    .on_collapse(&collapse_position, collapsed_id);
            }
            for removed_option in removed_options.into_iter() {
                propagator.push_propagate(PropagateItem::new(collapse_position, removed_option))
            }
            CollapseError::from_result(
                propagator.propagate(grid._grid_mut(), &option_data, &mut queue),
                CollapseErrorKind::Propagation,
                iter,
            )?;
            iter += 1;
        }

        Ok(())
    }

    // pub fn generate_position<CG: CollapsibleGrid<D, CB, Data>, R, P: PositionQueueProcession<D>>(
    //     &mut self,
    //     grid: &mut CG,
    //     rng: &mut R,
    //     positions: &[D::Pos],
    //     mut queue: PositionQueue<D, P>,
    // ) -> Result<(), CollapseError<D>>
    // where
    //     R: Rng,
    // {
    //     use crate::gen::collapse::queue::private::Sealed as _;
    //     use crate::gen::collapse::tile::private::CommonCollapsibleTileData as _;
    //     let mut iter = 0;

    //     if let Some(subscriber) = self.subscriber.as_mut() {
    //         subscriber.on_generation_start();
    //     }

    //     grid.remove_uncollapsed();

    //     queue.populate_inner_grid(rng, grid._grid_mut(), positions, &grid._option_data());

    //     // Progress with collapse.
    //     while let Some(collapse_position) = queue.get_next_position() {
    //         let (_,to_collapse) = grid._grid().get_tile_at_position(&collapse_position).unwrap();
    //         // skip collapsed;
    //         if to_collapse.is_collapsed() {
    //             continue;
    //         }
    //         // Make sure that the tile has at leas option, and purge them based on the direct neighbours.
    //         if !to_collapse.has_compatible_options()
    //             || !CG::purge_incompatible_options(
    //                 grid._grid_mut(),
    //                 &collapse_position,
    //                 grid._option_data(),
    //             )
    //         {
    //             return Err(CollapseError::new(
    //                 collapse_position,
    //                 CollapseErrorKind::Collapse,
    //                 iter,
    //             ));
    //         };

    //         let mut to_collapse = grid
    //             ._grid_mut()
    //             .get_mut_data_at_position(&collapse_position)
    //             .unwrap();
    //         to_collapse.collapse_basic(rng, grid._option_data());

    //         let collapsed_idx = to_collapse.collapse_idx().unwrap();
    //         // Purge options for the neighbours. This step is not required for the generation to be sound at the end,
    //         // but it increases the success rate of the process greatly at the relatively small performance cost.
    //         CG::purge_options_for_neighbours(
    //             grid._grid_mut(),
    //             collapsed_idx,
    //             &collapse_position,
    //             grid._option_data(),
    //         );

    //         if let Some(subscriber) = self.subscriber.as_mut() {
    //             let collapsed_id = grid
    //                 ._option_data()
    //                 .get_tile_type_id(&collapsed_idx)
    //                 .unwrap();
    //             subscriber
    //                 .as_mut()
    //                 .on_collapse(&collapse_position, collapsed_id);
    //         }
    //         iter += 1;
    //     }
    //     Ok(())
    // }
}




