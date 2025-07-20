#[macro_export]
macro_rules! __impl_propagate_item {
    (
        struct_name: $name:ident,
        position: $position:ident,
    ) => {

        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            pub position: $position,
            pub to_remove: usize,
        }

        impl $name {
            pub fn new(position: $position, to_remove: usize) -> Self {
                Self { position, to_remove }
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_propagator {
    (
        struct_name: $struct_name:ident,
        propagate_item: $propagate_item:ident,
        collapsible_tile_data: $collapsible_tile_data:ident,
        per_option_data: $per_option_data:ident,
        entrophy_queue: $entrophy_queue:ident,
        direction: $direction:ident,
        position: $position:ident,
        grid: $grid:ident,
    ) => {
        pub struct $struct_name {
            inner: Vec<$propagate_item>,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self { inner: Vec::new() }
            }
        }

        impl $struct_name {
            pub fn push_propagate(&mut self, item: $propagate_item) {
                self.inner.push(item);
            }
        
            pub(crate) fn propagate(
                &mut self,
                grid: &mut impl $grid<$collapsible_tile_data>,
                option_data: &$per_option_data,
                queue: &mut $entrophy_queue,
            ) -> Result<(), $position> {
                let mut tiles_to_update = HashSet::new();
                let size = *grid.size();
                while let Some(item) = self.inner.pop() {
                    for direction in $direction::ALL {
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
                            let binding = tile.data();
                            let removed = binding
                                .mut_ways_to_be_option()
                                .decrement(*option_idx, direction);
                            if removed {
                                binding.remove_option(option_data.get_weights(*option_idx));
                            }
                            if !binding.has_compatible_options() {
                                return Err(pos_to_update);
                            }
                            if removed {
                                self.push_propagate($propagate_item::new(tile.grid_position(), *option_idx));
                                tiles_to_update.insert(tile.grid_position());
                            }
                        }
                    }
                }
        
                for pos in tiles_to_update {
                    queue.update_queue((pos, &grid.get_data_at_position(&pos).unwrap()));
                }
        
                Ok(())
            }
        }
    };
}