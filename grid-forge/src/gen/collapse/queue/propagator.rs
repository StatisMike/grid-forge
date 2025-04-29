use std::collections::HashSet;

use crate::{core::common::*, r#gen::collapse::{option::private::PerOptionData, CollapsibleTileData}};

use super::{entrophy::EntrophyQueue, CollapseQueue};

#[derive(Debug)]
pub struct PropagateItem<D: Dimensionality> {
    pub position: D::Pos,
    pub to_remove: usize,
}

impl <D: Dimensionality> PropagateItem<D> {
    pub fn new(position: D::Pos, to_remove: usize) -> Self {
        Self {
            position,
            to_remove,
        }
    }
}

#[derive(Default)]
pub struct Propagator<D: Dimensionality> {
    inner: Vec<PropagateItem<D>>,
}

impl <D: Dimensionality> Propagator<D> {
    pub fn push_propagate(&mut self, item: PropagateItem<D>) {
        self.inner.push(item);
    }

    pub(crate) fn propagate<Tile: CollapsibleTileData<D>>(
        &mut self,
        grid: &mut GridMap<D, Tile>,
        option_data: &impl PerOptionData<D>,
        queue: &mut EntrophyQueue,
    ) -> Result<(), D::Pos> {
        let mut tiles_to_update = HashSet::new();
        let size = *grid.size();
        while let Some(item) = self.inner.pop() {
            for direction in D::Pos::all() {
                let pos_to_update =
                    if let Some(pos) = direction.opposite().march_step(&item.position, &size) {
                        pos
                    } else {
                        continue;
                    };
                let mut tile = if let Some(tile) = grid.get_mut_tile_at_position(&pos_to_update) {
                    tile
                } else {
                    continue;
                };
                if tile.as_ref().is_collapsed() {
                    continue;
                }
                for option_idx in
                    option_data.get_all_enabled_in_direction(item.to_remove, direction.opposite())
                {
                    let binding = tile.as_mut();
                    let removed = binding
                        .mut_ways_to_be_option()
                        .decrement(*option_idx, *direction);
                    if removed {
                        binding.remove_option(option_data.get_weights(*option_idx));
                    }
                    if !binding.has_compatible_options() {
                        return Err(pos_to_update);
                    }
                    if removed {
                        self.push_propagate(PropagateItem::new(tile.grid_position(), *option_idx));
                        tiles_to_update.insert(tile.grid_position());
                    }
                }
            }
        }

        for pos in tiles_to_update {
            queue.update_queue(&grid.get_mut_tile_at_position(&pos).unwrap());
        }

        Ok(())
    }
}
