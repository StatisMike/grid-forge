#[macro_export]
macro_rules! __impl_singular_resolver {
    (
        struct_name: $name:ident,
        subscriber_trait: $subscriber_trait:ident,
        collapsible_grid: $collapsible_grid:ident,
        propagate_item: $propagate_item:ident,
        propagator: $propagator:ident,
        entrophy_queue: $entrophy_queue:ident,
        position_queue: $position_queue:ident,
        collapse_error: $collapse_error:ident,
        position: $position:ty,
    ) => {
        /// Resolver of the singular collapsible procedural algorithm.
        ///
        /// It uses either [`EntrophyQueue`] or [`PositionQueue`] to process the option collapsing process of the [`CollapsibleTileGrid`],
        /// additionally providing an option to subscribe to the collapse process via [`singular::Subscriber`](Subscriber).
        pub struct $name<Data>
        where
            Data: TypedData,
        {
            subscriber: Option<Box<dyn $subscriber_trait>>,
            tile_type: PhantomData<Data>,
        }

        impl<Data> Default for $name<Data>
        where
            Data: TypedData,
        {
            fn default() -> Self {
                Self {
                    subscriber: None,
                    tile_type: PhantomData,
                }
            }
        }

        impl<Data> $name<Data>
        where
            Data: TypedData,
        {
            /// Attach a subscriber to the resolver. The subscriber will be notified of each tile being collapsed.
            pub fn with_subscriber(mut self, subscriber: Box<dyn $subscriber_trait>) -> Self {
                self.subscriber = Some(subscriber);
                self
            }

            /// Retrieve the subscriber attached to the resolver.
            pub fn retrieve_subscriber(&mut self) -> Option<Box<dyn $subscriber_trait>> {
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
            pub fn generate_entrophy<R: Rng>(
                &mut self,
                grid: &mut $collapsible_grid<Data>,
                rng: &mut R,
                positions: &[$position],
            ) -> Result<(), $collapse_error>
            {

                let mut iter = 0;
                let mut queue = $entrophy_queue::default();
                let mut propagator = $propagator::default();

                if let Some(subscriber) = self.subscriber.as_mut() {
                    subscriber.on_generation_start();
                }

                grid.remove_uncollapsed();

                let option_data = &grid.option_data;

                let tiles = CollapsibleTile2D::new_from_frequency_with_entrophy(
                    rng,
                    positions, 
                    option_data
                );
                
                for tile in tiles {
                    queue.update_queue(tile.0, tile.1.calc_entrophy());
                    grid.grid.insert_data(&tile.0, tile.1);
                }

                for initial_propagate in grid.get_initial_propagate_items(positions) {
                    propagator.push_propagate(initial_propagate);
                }

                $collapse_error::from_result(
                    propagator.propagate(&mut grid.grid, &option_data, &mut queue),
                    CollapseErrorKind::Init,
                    iter,
                )?;

                // Progress with collapse.
                while let Some(collapse_position) = queue.get_next_position() {
                    let to_collapse = grid
                        .grid
                        .get_mut_data_at_position(&collapse_position)
                        .unwrap();
                    // skip collapsed;
                    if to_collapse.is_collapsed() {
                        continue;
                    }
                    if !to_collapse.has_compatible_options() {
                        return Err($collapse_error::new(
                            collapse_position,
                            CollapseErrorKind::Collapse,
                            iter,
                        ));
                    }
                    let removed_options = to_collapse.collapse_gather_removed(rng, &option_data);

                    let collapsed_idx = to_collapse.collapsed_idx().unwrap();
                    if let Some(subscriber) = self.subscriber.as_mut() {
                        let collapsed_id = grid
                            .option_data
                            .get_tile_type_id(collapsed_idx)
                            .unwrap();
                        subscriber
                            .as_mut()
                            .on_collapse(&collapse_position, collapsed_id);
                    }
                    for removed_option in removed_options.into_iter() {
                        propagator.push_propagate($propagate_item::new(collapse_position, removed_option))
                    }
                    $collapse_error::from_result(
                        propagator.propagate(&mut grid.grid, &option_data, &mut queue),
                        CollapseErrorKind::Propagation,
                        iter,
                    )?;
                    iter += 1;
                }

                Ok(())
            }

            pub fn generate_position<R: Rng>(
                &mut self,
                grid: &mut $collapsible_grid<Data>,
                rng: &mut R,
                positions: &[$position],
                mut queue: $position_queue,
            ) -> Result<(), $collapse_error>
            {
                let mut iter = 0;

                if let Some(subscriber) = self.subscriber.as_mut() {
                    subscriber.on_generation_start();
                }

                grid.remove_uncollapsed();

                let option_data = &grid.option_data;

                let tiles = CollapsibleTile2D::new_from_frequency(
                    positions, 
                    option_data
                );
                
                for tile in tiles {
                    queue.update_queue(tile.0);
                    grid.grid.insert_data(&tile.0, tile.1);
                }

                // Progress with collapse.
                while let Some(collapse_position) = queue.get_next_position() {
                    let to_collapse = grid
                        .grid
                        .get_data_at_position(&collapse_position)
                        .unwrap();
                    // skip collapsed;
                    if to_collapse.is_collapsed() {
                        continue;
                    }
                    // Make sure that the tile has at leas option, and purge them based on the direct neighbours.
                    if !to_collapse.has_compatible_options()
                        || !$collapsible_grid::<Data>::purge_incompatible_options(
                            &mut grid.grid,
                            &collapse_position,
                            &option_data,
                        )
                    {
                        return Err($collapse_error::new(
                            collapse_position,
                            CollapseErrorKind::Collapse,
                            iter,
                        ));
                    };

                    let to_collapse = grid
                        .grid
                        .get_mut_data_at_position(&collapse_position)
                        .unwrap();
                    to_collapse.collapse_basic(rng, &option_data);

                    let collapsed_idx = to_collapse.collapsed_idx().unwrap();

                    // Purge options for the neighbours. This step is not required for the generation to be sound at the end,
                    // but it increases the success rate of the process greatly at the relatively small performance cost.
                    $collapsible_grid::<Data>::purge_options_for_neighbours(
                        &mut grid.grid,
                        collapsed_idx,
                        &collapse_position,
                        &option_data,
                    );

                    if let Some(subscriber) = self.subscriber.as_mut() {
                        let collapsed_id = grid
                            .option_data
                            .get_tile_type_id(collapsed_idx)
                            .unwrap();
                        subscriber
                            .as_mut()
                            .on_collapse(&collapse_position, collapsed_id);
                    }
                    iter += 1;
                }
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_singular_subscriber_trait {
    (
        trait_name: $name:ident,
        position: $position:ty,
    ) => {

        /// When applied to the struct allows injecting it into [`singular::Resolver`](Resolver) to react on each tile being collapsed.
        pub trait $name: Any {
            /// Called when the generation process starts. No-op by default, should be overridden to clear the state of the subcscriber
            /// if it retains any state.
            fn on_generation_start(&mut self) {
                // no-op by default
            }

            /// Called when a tile is collapsed.
            fn on_collapse(&mut self, position: &$position, tile_type_id: u64);

            /// To retrieve the concrete subscriber type from [`singular::Resolver`](Resolver).
            fn as_any(&self) -> &dyn Any;
        }
    }
}

#[macro_export]
macro_rules! __impl_singular_debug_subscriber {
    (
        struct_name: $name:ident,
        trait_name: $trait_name:ident,
        position: $position:ty,
    ) => {

        /// Basic Subscriber for debugging purposes.
        ///
        /// Implements both [`overlap::Subscriber`] and [`singular::Subscriber`], making it usable with both resolvers.
        /// Upon collapsing a tile, it will print the collapsed `GridPosition`, `tile_type_id` and (if applicable) `pattern_id`.
        #[derive(Debug, Default)]
        pub struct $name {
            file: Option<File>,
        }

        impl $name {
            pub fn new(file: Option<File>) -> Self {
                Self { file }
            }
        }

        impl $trait_name for $name {
            fn on_collapse(&mut self, position: &$position, tile_type_id: u64) {
                if let Some(file) = &mut self.file {
                    writeln!(
                        file,
                        "collapsed tile_type_id: {tile_type_id} on position: {position:?}"
                    )
                    .unwrap();
                } else {
                    println!("collapsed tile_type_id: {tile_type_id} on position: {position:?}");
                }
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_singular_collapse_history_subscriber {
    (
        struct_name: $name:ident,
        history_item_name: $history_item:ident,
        trait_name: $trait_name:ident,
        position: $position:ty,
    ) => {
        /// Event in the history of tile generation process, containing the [`GridPosition`] of the tile alongside its collapsed
        /// `tile_type_id`.
        #[derive(Debug, Clone)]
        pub struct CollapseHistoryItem<D: Dimensionality> {
            pub position: D::Pos,
            pub tile_type_id: u64,
        }

        /// Simple subscriber to collect history of tile generation process.
        ///
        /// Every new generation began by the resolver will clear the history.
        #[derive(Debug, Clone, Default)]
        pub struct CollapseHistorySubscriber<D: Dimensionality> {
            history: Vec<CollapseHistoryItem<D>>,
        }

        impl<D: Dimensionality> CollapseHistorySubscriber<D> {
            /// Returns history of tile generation process.
            pub fn history(&self) -> &[CollapseHistoryItem<D>] {
                &self.history
            }
        }

        impl<D: Dimensionality> Subscriber<D> for CollapseHistorySubscriber<D> {
            fn on_generation_start(&mut self) {
                self.history.clear();
            }

            fn on_collapse(&mut self, position: &D::Pos, tile_type_id: u64) {
                self.history.push(CollapseHistoryItem {
                    position: *position,
                    tile_type_id,
                });
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    }
}

