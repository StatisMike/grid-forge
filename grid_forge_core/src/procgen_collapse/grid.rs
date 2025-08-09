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
        propagate_item: $propagate_item:ident,
        frequency_hints: $frequency_hints:ident,
        adjacency_rules: $adjacency_rules:ident,
        grid: $grid:ident,
        grid_size: $grid_size:ident,
        position: $position:ident,
        direction: $direction:ident,
        per_option_data: $per_option_data:ident,
        collapse_error: $error:ident,
        collapsible_error: $collapsible_error:ident,
    ) => {

        macro_rules! collapsible_grid {
            () => { $grid<$collapsible_data> };
        }

        pub struct $name<Tile: TypedData> {
            pub (crate) grid: collapsible_grid!(),
            pub (crate) option_data: $per_option_data,
            tile_type: PhantomData<Tile>,
        } 

        impl <Tile: TypedData> Clone for $name<Tile> {
            fn clone(&self) -> Self {
                Self {
                    grid: self.grid.clone(),
                    option_data: self.option_data.clone(),
                    tile_type: PhantomData,
                }
            }
        }

        impl <Tile: TypedData> std::fmt::Debug for $name<Tile> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("CollapsibleTileGrid2D")
                    .field("grid", &self.grid)
                    .field("option_data", &self.option_data)
                    .finish()
            }
        }

        impl <Tile: TypedData> $name<Tile> {

                /// Creates a new empty grid with given [`GridSize`], preparing the rules for the generation of the tiles and the weights of the options.
                pub fn new_empty(
                    size: $grid_size,
                    frequencies: &$frequency_hints<Tile>,
                    adjacencies: &$adjacency_rules<Tile>,
                ) -> Self {
                    let mut option_data = $per_option_data::default();
                    option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner().clone());

                    Self {
                        grid: GridMap2D::new(size),
                        option_data,
                        tile_type: PhantomData,
                    }
                }

                /// Creates a new grid using the [`CollapsedGrid`] as a source grid. Created grid will have the same size as the
                /// source grid, and will be populated with existing collapsed tiles.
                ///
                /// Method can return an error if the collapsed grid contains tiles with `tile_type_id`s that are not present in the
                /// provided frequency hints and adjacency rules.
                pub fn new_from_collapsed(
                    collapsed: &$collapsed_grid,
                    frequencies: &$frequency_hints<Tile>,
                    adjacencies: &$adjacency_rules<Tile>,
                ) -> Result<Self, $collapsible_error> {
                    let mut option_data = $per_option_data::default();
                    option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner().clone());

                    let missing_ids = collapsed
                        .tile_type_ids()
                        .into_iter()
                        .filter(|id| !option_data.option_map.keys().any(|k| k == *id))
                        .copied()
                        .collect::<Vec<_>>();

                    if !missing_ids.is_empty() {
                        return Err($collapsible_error::new_missing(missing_ids));
                    }

                    let mut grid = GridMap2D::new(*collapsed.grid.size());

                    for tile in collapsed.grid.iter_tiles() {
                        grid.insert_data(
                            &tile.grid_position(),
                            $collapsible_data::new_collapsed_data(
                                option_data
                                    .get_tile_offset(tile.as_ref().tile_type_id())
                                    .expect("cannot get `option_idx`") as usize,
                            ),
                        );
                    }

                    Ok(Self {
                        grid,
                        option_data,
                        tile_type: PhantomData,
                    })
                }

                /// Changes the rules for the generation of the tiles and the weights of the options.
                ///
                /// Method can return an error if the inner collapsible grid contains tiles with `tile_type_id`s that are not present in the
                /// provided frequency hints and adjacency rules.
                pub fn change(
                    self,
                    frequencies: &$frequency_hints<Tile>,
                    adjacencies: &$adjacency_rules<Tile>,
                ) -> Result<Self, $collapsible_error> {
                    let collapsed = self.retrieve_collapsed();

                    Self::new_from_collapsed(&collapsed, frequencies, adjacencies)
                }

                /// Populates the grid with all collapsed tiles from the provided [`CollapsedGrid`].
                ///
                /// Method can return an error if the provided grid contains tiles with `tile_type_id`s that are not present in the
                /// provided frequency hints and adjacency rules or the provided grid size is greater than the size of inner
                /// collapsible grid.
                pub fn populate_from_collapsed(
                    &mut self,
                    collapsed: &$collapsed_grid,
                ) -> Result<(), $collapsible_error> {
                    if !self
                        .grid
                        .size()
                        .is_contained_within(collapsed.grid.size())
                    {
                        return Err($collapsible_error::new_wrong_size(
                            *collapsed.grid.size(),
                            *self.grid.size(),
                        ));
                    }

                    let missing_ids = collapsed
                        .tile_type_ids()
                        .into_iter()
                        .filter(|id| !self.option_data.option_map.keys().any(|k| k == *id))
                        .copied()
                        .collect::<Vec<_>>();

                    if !missing_ids.is_empty() {
                        return Err($collapsible_error::new_missing(missing_ids));
                    }

                    for tile in collapsed.grid.iter_tiles() {
                        self.grid.insert_data(
                            &tile.grid_position(),
                            $collapsible_data::new_collapsed_data(
                                self
                                    .option_data
                                    .get_tile_offset(tile.as_ref().tile_type_id())
                                    .expect("cannot get `option_idx`") as usize,
                            ),
                        );
                    }

                    Ok(())
                }

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
                                    tile
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
                                    tile
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
                                    tile
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
        
            /// Returns all possitions in the internal grid holding either collapsed or uncollapsed tiles.
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

            pub (crate) fn get_initial_propagate_items(&self, to_collapse: &[$position]) -> Vec<$propagate_item> {
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
                            out.push($propagate_item::new(
                                neighbour_tile.grid_position(),
                                *opt_to_remove,
                            ))
                        }
                    }
                }
                out
            }
    
            /// Removes options from tile neighbours after its collapse.
            pub (crate) fn purge_options_for_neighbours(
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
            pub (crate) fn purge_incompatible_options(
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