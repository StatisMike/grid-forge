#[macro_export]
macro_rules! __impl_collapsed_grid {
    (
        struct_name: $struct_name:ident,
        grid: $grid_name:ident,
        size: $size_type:ident,
    ) => {

        macro_rules! collapsed_grid {
            () => { $grid_name<CollapsedTileData> };
        }

        #[derive(Debug, Clone)]
        pub struct $struct_name {
            pub (crate) grid: collapsed_grid!(),
            pub (crate) tile_type_ids: TypeIdSet,
        }

        impl $struct_name {
            
            pub fn new(size: $size_type) -> Self {
                Self {
                    grid: <collapsed_grid!()>::new(size),
                    tile_type_ids: TypeIdSet::default(),
                }
            }

            #[inline]
            pub fn grid(&self) -> &collapsed_grid!() {
                &self.grid
            }

            #[inline]
            pub fn tile_type_ids(&self) -> &TypeIdSet {
                &self.tile_type_ids
            }
        }
    };
}


#[macro_export]
macro_rules! __impl_collapsible_grid {
    (
        struct_name: $name:ident,
        collapsible_data: $collapsible_data:ident,
        collapsed_grid: $collapsed_grid:ident,
        grid: $grid:ident,
        position: $position:ident,
        direction: $direction:ident,
        per_option_data: $per_option_data:ident,
        error: $error:ident,
    ) => {

        macro_rules! collapsible_grid {
            () => { $grid<$collapsible_data> };
        }

        macro_rules! propagate_item {
            () => { PropagateItem<$position> };
        }

        pub struct $name<Tile: TypedData> {
            grid: collapsible_grid!(),
            option_data: $per_option_data,
            tile_type: PhantomData<Tile>,
        } 

        impl <Tile: TypedData> $name<Tile> {

            pub fn retrieve_collapsed(&self) -> $collapsed_grid {
                let mut out = $collapsed_grid::new(self.grid.size().clone());
        
                for tile in self.grid.iter_tiles() {
                    if !tile.data().is_collapsed() {
                        continue;
                    }
        
                    out.grid.insert_data(
                        &tile.grid_position(),
                        CollapsedTileData::new(
                            self.option_data
                                .get_tile_type_id(
                                    &tile
                                        .data()
                                        .collapsed_idx()
                                        .expect("cannot get `collapse_idx` for uncollapsed tile"),
                                )
                                .expect("cannot get `tile_type_id` for uncollapsed tile"),
                        ),
                    );
                }
        
                out
            }
        
            pub fn retrieve_ident<Builder: IdentTileBuilder<InputTile>, InputTile>(
                &self,
                builder: &Builder,
            ) -> Result<$grid<InputTile>, $error> 
            where
                Builder: IdentTileBuilder<InputTile>,
                InputTile: TypedData,
            { 
                let mut out = $grid::<InputTile>::new(*self.grid.size());
        
                for tile in self.grid.iter_tiles() {
                    if !tile.data().is_collapsed() {
                        continue;
                    }
                    out.insert_data(
                        &tile.grid_position(),
                        builder.build_tile_unchecked(
                            self.option_data
                                .get_tile_type_id(
                                    &tile
                                        .data()
                                        .collapsed_idx()
                                        .expect("cannot get `collapse_idx` for uncollapsed tile"),
                                )
                                .expect("cannot get `tile_type_id` for uncollapsed tile"),
                        ),
                    );
                }
        
                Ok(out)
            }
        
            pub fn retrieve_ident_default<InputTile>(&self) -> $grid<InputTile>
            where
                InputTile: TypedData + IdDefault,
            {
                let mut out = $grid::<InputTile>::new(*self.grid.size());
        
                for tile in self.grid.iter_tiles() {
                    if !tile.data().is_collapsed() {
                        continue;
                    }
                    out.insert_data(
                        &tile.grid_position(),
                        InputTile::tile_type_default(
                            self.option_data
                                .get_tile_type_id(
                                    &tile
                                        .data()
                                        .collapsed_idx()
                                        .expect("cannot get `collapse_idx` for uncollapsed tile"),
                                )
                                .expect("cannot get `tile_type_id` for uncollapsed tile"),
                        ),
                    );
                }
        
                out
            }
        
            /// Returns all empty positions in the internal grid.
            pub fn empty_positions(&self) -> Vec<$position> {
                self.grid.get_all_empty_positions()
            }
        
            /// Returns all possitions in the internal grid holds collapsed or uncollapsed tiles are either collapsed.
            pub fn retrieve_positions(&self, collapsed: bool) -> Vec<$position> {
                let func: fn(&$collapsible_data) -> bool = if collapsed {
                    |d| d.is_collapsed()
                } else {
                    |d| !d.is_collapsed()
                };
                self.grid
                    .indexed_iter()
                    .filter_map(|t| {
                        if let Some(d) = t.1 {
                            if func(d) {
                                return Some(t.0);
                            }
                        }
                        None
                    })
                    .collect()
            }
        
            pub fn remove_uncollapsed(&mut self) {
                for t in self.grid.iter_mut() {
                    if let Some(d) = t {
                        if d.is_collapsed() {
                            continue;
                        }
                        t.take();
                    }
                }
            }

            pub (crate) fn get_initial_propagate_items(&self, to_collapse: &[$position]) -> Vec<propagate_item!()> {
                let mut out = Vec::new();
                let mut cache = HashMap::new();
                let mut check_generated = HashSet::new();
                let check_provided: HashSet<_> = HashSet::from_iter(to_collapse.iter());
    
                for pos_to_collapse in to_collapse {
                    for neighbour_tile in self.grid.get_neighbours(pos_to_collapse).inner().iter().flatten() {
                        if !neighbour_tile.as_ref().is_collapsed()
                            || check_provided.contains(&neighbour_tile.grid_position())
                            || check_generated.contains(&neighbour_tile.grid_position())
                        {
                            continue;
                        }
                        check_generated.insert(neighbour_tile.grid_position());
                        let collapsed_idx = neighbour_tile.as_ref().collapsed_idx().unwrap();
                        for opt_to_remove in cache.entry(collapsed_idx).or_insert_with(|| {
                            (0..self.option_data.option_count)
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