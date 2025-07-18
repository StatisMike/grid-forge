#[macro_export]
macro_rules! __impl_collapsible_grid {
    (
        trait_name: $name:ident,
        collapsible_data: $collapsible_data:ident,
        grid: $grid:ident,
        position: $position:ident,
        direction: $direction:ident,
        per_option_data: $per_option_data:ident,
    ) => {

        macro_rules! collapsible_grid {
            () => { $grid<$collapsible_data> };
        }
        macro_rules! propagate_item {
            () => { PropagateItem<$position> };
        }

        pub trait $name {       
            #[doc(hidden)]
            fn _grid(&self) -> &collapsible_grid!();
    
            #[doc(hidden)]
            fn _grid_mut(&mut self) -> &mut collapsible_grid!();
    
            #[doc(hidden)]
            fn _option_data(&self) -> &$per_option_data;
    
            #[doc(hidden)]
            fn _get_initial_propagate_items(&self, to_collapse: &[$position]) -> Vec<propagate_item!()> {
                let mut out = Vec::new();
                let mut cache = HashMap::new();
                let mut check_generated = HashSet::new();
                let check_provided: HashSet<_> = HashSet::from_iter(to_collapse.iter());
    
                for pos_to_collapse in to_collapse {
                    for neighbour_tile in self._grid().get_neighbours(pos_to_collapse).inner().iter().flatten() {
                        if !neighbour_tile.as_ref().is_collapsed()
                            || check_provided.contains(&neighbour_tile.grid_position())
                            || check_generated.contains(&neighbour_tile.grid_position())
                        {
                            continue;
                        }
                        check_generated.insert(neighbour_tile.grid_position());
                        let collapsed_idx = neighbour_tile.as_ref().collapsed_idx().unwrap();
                        for opt_to_remove in cache.entry(collapsed_idx).or_insert_with(|| {
                            (0..self._option_data().option_count)
                                .filter(|option_idx| option_idx != &collapsed_idx)
                                .collect::<Vec<usize>>()
                        }) {
                            out.push(<propagate_item!()>::new(
                                neighbour_tile.grid_position(),
                                *opt_to_remove,
                            ))
                        }
                    }
                }
                out
            }
    
            /// Removes options from tile neighbours after its collapse.
            fn purge_options_for_neighbours(
                grid: &mut collapsible_grid!(),
                collapsed_option: usize,
                collapsed_position: &$position,
                option_data: &$per_option_data,
            ) {
                for direction in $direction::ALL {
                    if let Some(mut tile) = grid.get_mut_neighbour_at(collapsed_position, &direction) {
                        if tile.as_ref().is_collapsed() {
                            continue;
                        }
    
                        let enabled =
                            option_data.get_all_enabled_in_direction(collapsed_option, direction);
                        for possible_option in tile
                            .as_ref()
                            .ways_to_be_option()
                            .iter_possible()
                            .collect::<Vec<_>>()
                        {
                            if !enabled.contains(&possible_option)
                                && tile
                                    .data()
                                    .mut_ways_to_be_option()
                                    .purge_option(possible_option)
                            {
                                let weights = option_data.get_weights(possible_option);
                                tile.data().remove_option(weights);
                            }
                        }
                    }
                }
            }
    
            /// Removes options from tile based of possible options for its neighbours.
            fn purge_incompatible_options(
                grid: &mut collapsible_grid!(),
                position: &$position,
                option_data: &$per_option_data,
            ) -> bool {
                let num_options = option_data.option_count;
                let mut possible_options = Vec::with_capacity(num_options);
                possible_options.resize(num_options, true);
    
                for direction in $direction::ALL {
                    if let Some(tile) = grid.get_neighbour_at(position, &direction) {
                        if let Some(collapsed_idx) = tile.as_ref().collapsed_idx() {
                            let enabled = option_data
                                .get_all_enabled_in_direction(collapsed_idx, direction.opposite());
                            for (option_idx, state) in possible_options.iter_mut().enumerate() {
                                if *state && !enabled.contains(&option_idx) {
                                    *state = false;
                                }
                            }
                        } else if tile.as_ref().num_possible_options() < option_data.possible_options_count
                        {
                            let mut possible_in_any: HashSet<usize> = HashSet::new();
                            for neigbour_idx in tile.as_ref().ways_to_be_option().iter_possible() {
                                possible_in_any.extend(
                                    option_data
                                        .get_all_enabled_in_direction(
                                            neigbour_idx,
                                            direction.opposite(),
                                        )
                                        .iter(),
                                );
                            }
                            for (option_idx, state) in possible_options.iter_mut().enumerate() {
                                if *state && !possible_in_any.contains(&option_idx) {
                                    *state = false;
                                }
                            }
                        }
                    }
                }
    
                if !possible_options.iter().any(|state| *state) {
                    return false;
                }
    
                let tile = grid.get_mut_data_at_position(position).unwrap();
                for (possible, (option_idx, weights)) in
                    possible_options.iter().zip(option_data.iter_weights())
                {
                    if !possible && tile.mut_ways_to_be_option().purge_option(option_idx) {
                        tile.remove_option(*weights);
                    }
                }
                true
            }
        }
    };
}