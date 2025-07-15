#[doc(hidden)]
#[macro_export]
macro_rules! __impl_grid_trait {
    (
        grid_map_trait: $grid_map_trait:ident,

        direction: $direction_type:ty,
        direction_table: $direction_table:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        tile: $tile_type:ident,
        tile_ref: $tile_ref_type:ident,
        tile_mut: $tile_mut_type:ident,
        tile_container_trait: $tile_container_trait:ident,
        neighbours_count: $neighbour_size:literal,

        $(generic_params: [$($generic_param:tt)*],)?
        $(where_clause: [$($where_clause:tt)*],)?
        $(other_mut_access: [$($other_mut_access:tt => $other_mut_access_translate:tt)*],)?
    )
    => {

        #[allow(unused_imports)]
        use crate::core::$tile_container_trait as _;
        #[allow(unused_imports)]
        use std::default::Default as _;

        macro_rules! __tile_type { () => { $tile_type<Data> }; }
        macro_rules! __tile_ref_type { () => { $tile_ref_type<'a, Data> }; }
        macro_rules! __tile_mut_type { () => { $tile_mut_type<'a, Data> }; }

        /// Trait for basic grid map operations.
        /// 
        /// Encapsulates all operations on the grid map and data stored in its tiles.
        /// 
        /// For documentation of each method, refer to the documentation on the concrete type implementing this trait.
        pub trait $grid_map_trait<$($($generic_param)*)?>
        $(where $($where_clause)*)?
        {
            fn size(&self) -> &$size_type;

            fn get_data_at_position(&self, position: &$position_type) -> Option<&Data>;

            fn get_tile_at_position<'a>(&'a self, position: &$position_type) -> Option<__tile_ref_type!()>;

            fn get_mut_data_at_position(&mut self, position: &$position_type) -> Option<&mut Data>;

            fn get_mut_tile_at_position<'a>(&'a mut self, position: &$position_type) -> Option<__tile_mut_type!()>;

            fn get_tiles_at_positions<'a>(&'a self, positions: &[$position_type]) -> Vec<__tile_ref_type!()>;

            fn insert_tile(&mut self, tile: __tile_type!()) -> bool;

            fn insert_data(&mut self, position: &$position_type, data: Data) -> bool;

            fn remove_tile_at_position(&mut self, position: &$position_type) -> Option<__tile_type!()>;

            fn get_neighbours<'a>(&'a self, position: &$position_type) -> $direction_table<Option<__tile_ref_type!()>>;

             fn get_neighbour_at<'a>(
                &'a self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_ref_type!()>;

           fn get_mut_neighbour_at<'a>(
                &'a mut self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_mut_type!()>;

            fn get_all_positions(&self) -> Vec<$position_type>;

            fn iter_all_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + 'a
            where
                Data: 'a;

            fn get_all_positions_with_type(&self, tile_type_id: &u64) -> Vec<$position_type>
            where Data: grid_forge_core::id::TypedData;

            fn get_all_empty_positions(&self) -> Vec<$position_type>;

            fn iter_all_empty_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + 'a
            where
                Data: 'a;

           fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>>
            where
                Data: 'a;

            fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = __tile_ref_type!()>
            where
                Data: 'a;

            fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = __tile_mut_type!()>
            where
                Data: 'a;

            fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = ($position_type, &'a Option<Data>)>
            where
                Data: 'a;

            fn indexed_iter_mut<'a>(
                &'a mut self,
            ) -> impl Iterator<Item = ($position_type, &'a mut Option<Data>)>
            where
                Data: 'a;

            fn drain_remapped(self, anchor_pos: $position_type) -> Vec<__tile_type!()>;

            fn cloned_remapped(&self, anchor_pos: $position_type) -> Vec<__tile_type!()>
            where
                Data: Clone;

            fn drain(self) -> Vec<__tile_type!()>;

            fn fill_empty_using(&mut self, func: fn($position_type) -> Data);

            fn fill_empty_with_default(&mut self)
            where
                Data: Default;

            fn fill_empty_with(&mut self, data: Data)
            where
                Data: Clone;

            fn mut_access_tracking(&mut self, enabled: bool);

            fn get_mut_accessed(&self) -> Vec<$position_type>;

            fn drain_mut_accessed(&mut self) -> Vec<$position_type>;

            fn len_taken(&self) -> usize;

            fn first_data<'a>(&'a self) -> Option<__tile_ref_type!()>;
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __impl_grid {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $struct_name:ident {
            $($additional_field:ident: $additional_type:ty),* $(,)?
        }

        direction: $direction_type:ty,
        direction_table: $direction_table:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        tile: $tile_type:ident,
        tile_ref: $tile_ref_type:ident,
        tile_mut: $tile_mut_type:ident,
        tile_container_trait: $tile_container_trait:ident,
        neighbours_count: $neighbour_size:literal,

        grid_map_trait: $grid_map_trait:ident,

        $(generic_params: [$($generic_param:tt)*],)?
        $(where_clause: [$($where_clause:tt)*],)?
        $(other_mut_access: [$($other_mut_access:tt => $other_mut_access_translate:tt)*],)?
    ) => {

        #[allow(unused_imports)]
        use crate::core::$tile_container_trait as _;
        #[allow(unused_imports)]
        use std::default::Default as _;

        macro_rules! __tile_type { () => { $tile_type<Data> }; }
        macro_rules! __tile_ref_type { () => { $tile_ref_type<'a, Data> }; }
        macro_rules! __tile_mut_type { () => { $tile_mut_type<'a, Data> }; }

        $(#[$struct_meta])*
        $vis struct $struct_name<$($($generic_param)*)?>
        $(where $($where_clause)*)?
        {
            size: $size_type,
            tiles: Vec<Option<Data>>,
            mut_accessed: Option<std::collections::HashSet<$position_type>>,
            $($additional_field: $additional_type),*
        }

        impl<$($($generic_param)*)?> $struct_name<$($($generic_param)*)?>
        $(where $($where_clause)*)?
        {

            /// Creates a new empty grid with the specified dimensions.
            ///
            /// All tile slots are initialized as empty. Use insertion methods to populate the grid.
            pub fn new(size: $size_type) -> Self {
                let count = size.max_tile_count();
                let mut tiles = Vec::new();
                for _ in 0..count {
                    tiles.push(None);
                }
                tiles.shrink_to_fit();
                Self { size, tiles, mut_accessed: None, $($additional_field: <$additional_type>::default()),* }
            }
        }

        impl<$($($generic_param)*)?> $grid_map_trait<Data> for $struct_name<$($($generic_param)*)?>
        $(where $($where_clause)*)?
        {
            #[inline]
            fn size(&self) -> &$size_type {
                &self.size
            }

            /// Gets a direct reference to data at a position if present and valid.
            ///
            /// # Returns
            /// - `Some(&Data)` if position is valid and contains data
            /// - `None` otherwise
            ///
            /// For a version that returns position and data together, see [`get_mut_tile_at_position`](Self::get_tile_at_position()).
            fn get_data_at_position(&self, position: &$position_type) -> Option<&Data> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                unsafe { self.tiles.get_unchecked(self.size.offset(&position)).as_ref() }
            }

            /// Gets an immutable tile reference containing both position and data.
            ///
            /// # Returns
            #[doc = "- [`Some(TileRef)`]"]
            #[doc = concat!("(", stringify!($tile_ref_type), ")")]
            #[doc = " if position is valid and contains data"]
            /// - `None` otherwise
            ///
            /// For direct data access without position, see [`get_data_at_position`](Self::get_data_at_position()).
            fn get_tile_at_position<'a>(&'a self, position: &$position_type) -> Option<__tile_ref_type!()> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                unsafe {
                    self.tiles.get_unchecked(self.size.offset(&position))
                        .as_ref()
                        .map(|data| (*position, data).into())
                }
            }

            /// Gets a direct mutable reference to data at a position if present and valid.
            ///
            /// # Returns
            /// - `Some(&mut Data)` if position is valid and contains data
            /// - `None` otherwise
            ///
            /// For a version that returns position and data together, see [`get_tile_mut_at_position`](Self::get_tile_mut_at_position()).
            fn get_mut_data_at_position(&mut self, position: &$position_type) -> Option<&mut Data> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe { self.tiles.get_unchecked_mut(self.size.offset(position)).as_mut() }
            }

            /// Gets a mutable tile reference containing both position and data.
            ///
            /// # Returns
            #[doc = "- [`Some(TileMut)`]"]
            #[doc = concat!("(", stringify!($tile_mut_type), ")")]
            #[doc = " if position is valid and contains data"]
            /// - `None` otherwise
            ///
            /// For direct data access without position, see [`get_mut_data_at_position`](Self::get_mut_data_at_position()).
            fn get_mut_tile_at_position<'a>(&'a mut self, position: &$position_type) -> Option<__tile_mut_type!()> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe {
                    self
                        .tiles
                        .get_unchecked_mut(self.size.offset(&position))
                        .as_mut()
                        .map(|data| (*position, data).into())
                }
            }

            /// Gets multiple tile references containing both position and data.
            ///
            /// # Returns
            #[doc = "Vec of [`TileRef`]]"]
            #[doc = concat!("(", stringify!($tile_ref_type), ")")]
            #[doc = " for each provided position containing data."]
            fn get_tiles_at_positions<'a>(&'a self, positions: &[$position_type]) -> Vec<__tile_ref_type!()> {
                positions
                    .iter()
                    .filter_map(|position| self.get_tile_at_position(position))
                    .collect::<Vec<_>>()
            }

            /// Inserts a tile into the grid using its stored position.
            ///
            /// # Returns
            /// - `true` if insertion succeeded (valid position)
            /// - `false` if position is out of bounds
            fn insert_tile(&mut self, tile: __tile_type!()) -> bool
            {
                if !self.size.is_position_valid(&tile.grid_position()) {
                    return false;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(tile.grid_position());
                }
                unsafe {
                    self.tiles.get_unchecked_mut(self.size.offset(&tile.grid_position())).replace(tile.into_data());
                }
                true
            }

            /// Inserts raw data at a specific position.
            ///
            /// See [`insert_tile`](Self::insert_tile()) for version using tile type.
            ///
            /// # Returns
            /// - `true` if insertion succeeded (valid position)
            /// - `false` if position is out of bounds
            fn insert_data(&mut self, position: &$position_type, data: Data) -> bool {
                if !self.size.is_position_valid(position) {
                    return false;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                unsafe {
                    self.tiles.get_unchecked_mut(self.size.offset(&position)).replace(data);
                }
                true
            }

            /// Removes and returns the tile at a position if present.
            ///
            /// # Returns
            #[doc = "- [`Some(Tile)`]"]
            #[doc = concat!("(", stringify!($tile_type), ")")]
            #[doc = " if position was valid and contained data"]
            /// - `None` otherwise
            fn remove_tile_at_position(&mut self, position: &$position_type) -> Option<__tile_type!()>
            {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                let offset = self.size.offset(position);

                unsafe {
                    self.tiles.get_unchecked_mut(offset)
                        .take()
                        .and_then(|t| Some((*position, t).into()))
                }
            }

            /// Retrieve all neigbours of the provided position.
            ///
            #[doc = "Returns the [`DirectionTable`]"]
            #[doc = concat!("(", stringify!($direction_table_type), ")")]
            #[doc = " containing the [`Some(TileRef)`]"]
            #[doc = concat!("(", stringify!($tile_ref_type), ")")]
            #[doc = " for each direction if it contains data, or `None` otherwise."]
            fn get_neighbours<'a>(&'a self, position: &$position_type) -> $direction_table<Option<__tile_ref_type!()>> {
                let mut result = $direction_table::new([const { None }; $neighbour_size]);
                for direction in <$direction_type>::ALL {
                    if let Some(pos) = direction.march_step(position, &self.size) {
                        if let Some(tile) = self.get_tile_at_position(&pos) {
                            result[direction] = Some(tile);
                        }
                    }
                }
                result
            }

            /// Retrieve neighbour in given direction of the provided position.
            ///
            /// # Returns
            #[doc = "- [`Some(TileRef)`]"]
            #[doc = concat!("(", stringify!($tile_ref_type), ")")]
            #[doc = " if position is valid and contains data"]
            /// - `None` otherwise
            fn get_neighbour_at<'a>(
                &'a self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_ref_type!()> {
                if let Some(position) = direction.march_step(position, &self.size) {
                    return self.get_tile_at_position(&position);
                }
                None
            }

            /// Retrieve mutable neighbour in given direction of the provided position.
            ///
            /// # Returns
            #[doc = "- [`Some(TileMut)`]"]
            #[doc = concat!("(", stringify!($tile_mut_type), ")")]
            #[doc = " if position is valid and contains data"]
            /// - `None` otherwise
            fn get_mut_neighbour_at<'a>(
                &'a mut self,
                position: &$position_type,
                direction: &$direction_type,
            ) -> Option<__tile_mut_type!()> {
                if let Some(position) = direction.march_step(position, &self.size) {
                    return self.get_mut_tile_at_position(&position);
                }
                None
            }

            /// Returns all positions contactining data in the grid map..
            fn get_all_positions(&self) -> Vec<$position_type> {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
                    .collect()
            }

            /// Returns iterator over all positions contactining data in the grid map.
            fn iter_all_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + 'a
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_some() { Some(pos) } else { None })
            }

            /// Returns all positions containing data of the specified [`tile_type_id`] in the grid map.
            fn get_all_positions_with_type(&self, tile_type_id: &u64) -> Vec<$position_type>
            where Data: grid_forge_core::id::TypedData
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| {
                        if let Some(t) = t {
                            if t.tile_type_id() == *tile_type_id {
                                Some(pos)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            }

            /// Returns all empty positions in the grid map.
            fn get_all_empty_positions(&self) -> Vec<$position_type> {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
                    .collect()
            }

            /// Returns iterator over all empty positions in the grid map.
            fn iter_all_empty_positions<'a>(&'a self) -> impl Iterator<Item = $position_type> + 'a
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| if t.is_none() { Some(pos) } else { None })
            }

            /// Returns iterator over all tile slots in the grid map (mutable).
            ///
            /// This method is not tracked even with [`mut_access_tracking`](Self::mut_access_tracking) enabled.
            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Option<Data>>
            where
                Data: 'a,
            {
                self.tiles.iter_mut()
            }

            /// Returns iterator over all tiles in the grid map.
            fn iter_tiles<'a>(&'a self) -> impl Iterator<Item = __tile_ref_type!()>
            where
                Data: 'a,
            {
                self.indexed_iter()
                    .filter_map(|(pos, data)| data.as_ref().map(|d| (pos, d).into()))
            }

            /// Returns iterator over all tiles in the grid map (mutable).
            ///
            /// This method is not tracked even with [`mut_access_tracking`](Self::mut_access_tracking) enabled.
            fn iter_mut_tiles<'a>(&'a mut self) -> impl Iterator<Item = __tile_mut_type!()>
            where
                Data: 'a,
            {
                self.indexed_iter_mut()
                    .filter_map(|(pos, data)| data.as_mut().map(|d| (pos, d).into()))
            }

            /// Returns iterator over tuples of position and tile slot data.
            fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = ($position_type, &'a Option<Data>)>
            where
                Data: 'a,
            {
                self.tiles
                    .iter()
                    .enumerate()
                    .map(move |(idx, t)| (self.size().pos_from_offset(idx), t))
            }

            /// Returns iterator over tuples of position and tile slot data (mutable).
            ///
            /// This method is not tracked even with [`mut_access_tracking`](Self::mut_access_tracking) enabled.
            fn indexed_iter_mut<'a>(
                &'a mut self,
            ) -> impl Iterator<Item = ($position_type, &'a mut Option<Data>)>
            where
                Data: 'a,
            {
                let size = self.size().clone();
                self.tiles
                    .iter_mut()
                    .enumerate()
                    .map(move |(idx, t)| (size.pos_from_offset(idx), t))
            }

            /// Drains all tiles in the grid map, remapping their positions.
            ///
            /// This method consumes the grid map and returns all tiles as a vector. Their positions will be
            /// remapped, so that the tile at first position will be set at the specified `anchor_pos`.
            ///
            /// To retrieve the remapped tiles without consuming the grid map, you can use [`cloned_remapped`](Self::cloned_remapped)
            /// method if the tile data implements [`Clone`](Clone).
            fn drain_remapped(mut self, anchor_pos: $position_type) -> Vec<__tile_type!()> {
                self.indexed_iter_mut()
                    .filter_map(|(pos, t)| {
                        if t.is_none() {
                            None
                        } else {
                            Some((anchor_pos + pos, t.take().unwrap()).into())
                        }
                    })
                    .collect()
            }

            /// Retrieves all tiles in the grid map, remapping their positions.
            ///
            /// This method returns all tiles cloned as a vector. Their positions will be remapped,
            /// so that the tile at first position will be set at the specified `anchor_pos`.
            ///
            /// To drain the grid map while remapping, you can use [`drain_remapped`](Self::drain_remapped).
            fn cloned_remapped(&self, anchor_pos: $position_type) -> Vec<__tile_type!()>
            where
                Data: Clone,
            {
                self.indexed_iter()
                    .filter_map(|(pos, t)| {
                        if t.is_some() {
                            Some((anchor_pos + pos, t.clone().unwrap()).into())
                        } else {
                            None
                        }
                    })
                    .collect()
            }

            /// Retrieves all tiles consuming the map.
            fn drain(mut self) -> Vec<__tile_type!()> {
                self.indexed_iter_mut()
                    .filter_map(|(pos, t)| {
                        if t.is_none() {
                            None
                        } else {
                            Some((pos, t.take().unwrap()).into())
                        }
                    })
                    .collect()
            }

            /// Fills empty positions in the grid map with the result of the provided function.
            fn fill_empty_using(&mut self, func: fn($position_type) -> Data) {
                for (pos, t) in self.indexed_iter_mut() {
                    if t.is_none() {
                        t.replace(func(pos));
                    }
                }
            }

            /// Fills empty positions in the grid map with the tile data default value.
            fn fill_empty_with_default(&mut self)
            where
                Data: Default,
            {
                let empty_positions = self.get_all_empty_positions();
                for pos in empty_positions {
                    self.insert_data(&pos, Data::default());
                }
            }

            /// Fills empty positions in the grid map with the provided data.
            fn fill_empty_with(&mut self, data: Data)
            where
                Data: Clone,
            {
                for pos in self.get_all_empty_positions() {
                    self.insert_data(&pos, data.clone());
                }
            }

            /// Enables or disables tracking of mutable accesses.
            ///
            /// If enabled, the [`GridMap`] will track which positions are accessed and will
            /// store their positions for the later retrieval with [`get_mut_accessed`](Self::get_mut_accessed())
            /// or [`drain_mut_accessed`](Self::drain_mut_accessed()).
            ///
            /// **WARNINGS**:
            /// - Enabling this feature will make the mutable access slower.
            /// - Mutable access is not tracked with iterator methods - it is implied that all tiles will be modified.
            /// - Other methods which won't track mutable access even with this enabled are specified as such in their
            ///   documentation.
            fn mut_access_tracking(&mut self, enabled: bool) {
                self.mut_accessed = if enabled {
                    Some(std::collections::HashSet::new())
                } else {
                    None
                };

                $($(
                    self.$other_mut_access.mut_access_tracking(enabled);
                )*)?
            }

            /// Gets all positions that were accessed mutably.
            ///
            /// When [`mut_access_tracking`](Self::mut_access_tracking) is enabled, many mutable accesses to the grid
            /// will be tracked (see method documentation for details). This methods returns all positions that were accessed mutably.
            ///
            /// This method does not drain the mutable accessed positions. If you want to clear the internal tracker,
            /// use [`drain_mut_accessed`](Self::drain_mut_accessed).
            fn get_mut_accessed(&self) -> Vec<$position_type> {
                if let Some(mut_accessed) = &self.mut_accessed {
                    $(
                        let mut combined = mut_accessed.clone();
                        $(
                            if let Some(other) = &self.$other_mut_access.get_mut_accessed() {
                                self.$other_mut_access_translate(&mut combined, other);
                            }
                        )*
                        let mut_accessed = combined;
                    )?

                    let mut result = mut_accessed.iter().copied().collect::<Vec<_>>();
                    result.sort();
                    result
                } else {
                    Vec::new()
                }
            }

            /// Drains all positions that were accessed mutably.
            ///
            /// When [`mut_access_tracking`](Self::mut_access_tracking) is enabled, many mutable accesses to the grid
            /// will be tracked (see method documentation for details). This methods drains all positions that were accessed mutably.
            ///
            /// It will clear the internal tracker. If you want to keep the positions mutably accessed, use [`get_mut_accessed`](Self::get_mut_accessed)
            /// instead.
            fn drain_mut_accessed(&mut self) -> Vec<$position_type> {
                if let Some(mut_accessed) = &self.mut_accessed.take() {
                    $(
                        let mut combined = mut_accessed.clone();
                        $(
                            if let Some(other) = &self.$other_mut_access.take_mut_accessed().take() {
                                self.$other_mut_access_translate(&mut combined, other);
                            }
                        )*
                        let mut_accessed = combined;
                    )?

                    let mut result = mut_accessed.iter().copied().collect::<Vec<_>>();
                    result.sort();
                    result
                } else {
                    Vec::new()
                }
            }

            fn len_taken(&self) -> usize {
                self.iter_all_positions().count()
            }

            fn first_data<'a>(&'a self) -> Option<__tile_ref_type!()> {
                self.iter_tiles().next()
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_grid_with_shared {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $struct_name:ident {
            $($additional_field:ident: $additional_type:ty),* $(,)?
        }

        direction: $direction:ty,
        direction_table: $direction_table:ident,
        size: $size:ty,
        position: $position:ty,
        tile: $tile:ident,
        tile_ref: $tile_ref:ident,
        tile_mut: $tile_mut:ident,
        tile_container_trait: $tile_container_trait:ident,
        neighbours_count: $neighbours_count:literal $(,)?

        grid_map_trait: $grid_map_trait:ident,

        tile_ref_shared: $tile_ref_shared:ident,
        tile_mut_shared: $tile_mut_shared:ident,

        $(generic_params: [$($generic_param:tt)*],)?
        $(where_clause: [$($where_clause:tt)*],)?
        $(other_mut_access: [$($other_mut_access:tt => $other_mut_access_translate:tt)*],)?
    ) => {
        grid_forge_core::__impl_grid! {
            $(#[$struct_meta])*
            $vis struct $struct_name
            {
                shared_data: grid_forge_core::id::SharedDataContainer<Shared>,
                $($additional_field: $additional_type),*
            }

            direction: $direction,
            direction_table: $direction_table,
            size: $size,
            position: $position,
            tile: $tile,
            tile_ref: $tile_ref,
            tile_mut: $tile_mut,
            tile_container_trait: $tile_container_trait,
            neighbours_count: $neighbours_count,

            grid_map_trait: $grid_map_trait,

            generic_params: [
                $($($generic_param)*, )?
                Shared
            ],
            where_clause: [
                $($($where_clause)*, )?
                Shared: grid_forge_core::id::SharedData
            ],
            other_mut_access: [
                shared_data => shared_data_mut_access_translate
                $(, $($other_mut_access => $other_mut_access_translate)*)?
            ],
        }

        macro_rules! __tile_ref_shared_type { () => { $tile_ref_shared<'a, Data, Shared> }; }
        macro_rules! __tile_mut_shared_type { () => { $tile_mut_shared<'a, Data, Shared> }; }

        impl <$($($generic_param)*)?, Shared> $struct_name<$($($generic_param)*)?, Shared>
        where
            $($($where_clause)*, )?
            Shared: grid_forge_core::id::SharedData,
        {
            /// Imports shared data to the grid.
            ///
            /// If there were some shared data already present with the same `type_id`, it will be overwritten.
            /// Non-matched `type_id`s will be ignored. To make sure that they are wiped out, you can use
            /// [`export_shared_data`](Self::export_shared_data) to drain existing shared data..
            ///
            /// While importing shared data, even if `mut_access_tracking` is enabled, the shared data inclusion
            /// will not be tracked. In these scenarios it is implied that all tiles were modified and all
            /// objects dependent on the shared data should be updated.
            pub fn import_shared_data(&mut self, shared_data: grid_forge_core::id::TypeIdMap<Shared>)
            {
                for (type_id, data) in shared_data {
                    self.shared_data.replace_shared_data(type_id, Some(data), false);
                }
            }

            /// Exports shared data from the grid.
            ///
            /// This method will drain all shared data from the grid and return it as a [`TypeIdMap`](crate::id::TypeIdMap).
            /// If you would rather keep the shared data, you can use [`clone_shared_data`](Self::clone_shared_data) instead
            /// if the shared data struct implements [`Clone`](Clone).
            ///
            /// While exporting shared data, even if `mut_access_tracking` is enabled, the shared data mut access
            /// will not be tracked. In these scenarios it is implied that all tiles were modified and all
            /// objects dependent on the shared data should be updated.
            pub fn export_shared_data(&mut self) -> grid_forge_core::id::TypeIdMap<Shared> {
                self.shared_data.drain_shared_data()
            }

            /// Check if shared data for tiles in the grid is present.
            ///
            /// Returns vector of all tile_type_ids present in the grid which don't have
            /// corresponding shared data registered.
            pub fn check_shared_data(&self) -> Vec<u64> {
                let mut checked = std::collections::HashSet::new();
                let mut missing = Vec::new();
                for tile in self.iter_tiles() {
                    if checked.contains(&tile.data().tile_type_id()) {
                        continue;
                    }
                    let shared = self.shared_data.get_shared_data(&tile.data().tile_type_id());
                    if shared.is_none() {
                        missing.push(tile.data().tile_type_id());
                    }
                    checked.insert(tile.data().tile_type_id());
                }
                missing

            }

            /// Clones shared data from the grid.
            ///
            /// This method will export all shared data by cloning them, keeping existing shared data intact.
            pub fn clone_shared_data(&self) -> grid_forge_core::id::TypeIdMap<Shared>
            where Shared: Clone {
                self.shared_data.iter_shared_data().map(|(type_id, data)| (*type_id, data.clone())).collect()
            }

            /// Inserts shared data into the grid for specified `type_id`.
            ///
            /// If there were some shared data already present with the same `type_id`, it will be overwritten.
            /// When `mut_access_tracking` is enabled, the shared data inclusion will be tracked.
            pub fn insert_shared_data(&mut self, type_id: u64, data: Shared) {
                self.shared_data.replace_shared_data(type_id, Some(data), true);
            }

            /// Removes shared data from the grid for the specified `type_id`.
            ///
            /// When `mut_access_tracking` is enabled, the shared data removal will be tracked.
            pub fn remove_shared_data(&mut self, type_id: u64) {
                self.shared_data.replace_shared_data(type_id, None, true);
            }

            /// Gets mutable reference to the shared data for the specified `type_id`.
            ///
            /// When `mut_access_tracking` is enabled, the shared data mutable access will be tracked.
            ///
            /// Returns `None` if there is no shared data with the specified `type_id`.
            pub fn get_mut_shared_data(&mut self, type_id: u64) -> Option<&mut Shared> {
                self.shared_data.get_shared_data_mut(&type_id)
            }

            /// Gets shared data for the specified position.
            ///
            /// Returns `None` if there is no shared data for the tile type at the specified position.
            pub fn get_shared_data_at_position(&self, position: &$position) -> Option<&Shared> {
                let Some(tile) = self.get_tile_at_position(position) else { return None };

                self.shared_data.get_shared_data(&tile.data().tile_type_id())
            }

            /// Gets [`TileContainer`] including tile shared data.
            ///
            /// Returns a composite struct containing the tile position and immutable references to the tile and shared data.
            ///
            /// Returns `None` if there is no tile at the specified position or no shared data for its tile type.
            pub fn get_tile_with_shared_at_position<'a>(&'a self, position: &$position) -> Option<__tile_ref_shared_type!()> {
                let Some(tile) = self.get_tile_at_position(position) else { return None };
                let Some(shared) = self.shared_data.get_shared_data(&tile.data().tile_type_id()) else { return None };
                Some((tile, shared).into())
            }

            /// Gets mutable [`TileContainer`] including tile shared data.
            ///
            /// Returns a composite struct containing the tile position, mutable reference to the tile data and immutable
            /// reference to the shared data.
            ///
            /// Its exclusive way to retrieve reference to the shared data alongside mutable reference to the tile data
            /// at the same time.
            ///
            /// If `mut_access_tracking` is enabled, the position will be tracked as mutably accessed.
            ///
            /// Returns `None` if there is no tile at the specified position or no shared data for its tile type.
            pub fn get_mut_tile_with_shared_at_position<'a>(&'a mut self, position: &$position) -> Option<__tile_mut_shared_type!()> {
                if !self.size.is_position_valid(position) {
                    return None;
                }
                if let Some(mut_accessed) = &mut self.mut_accessed {
                    mut_accessed.insert(*position);
                }
                let Some(data) = (unsafe {
                    self.tiles
                        .get_unchecked_mut(self.size.offset(&position))
                        .as_mut()
                }) else { return None };

                let Some(shared) = self
                    .shared_data
                    .get_shared_data(&data.tile_type_id())
                else {
                    return None;
                };
                Some((*position, data, shared).into())
            }

            fn shared_data_mut_access_translate(
                &self,
                mut_accessed_tiles: &mut std::collections::HashSet<$position>,
                mut_accessed_types: &grid_forge_core::id::TypeIdSet,
            ) {
                use crate::core::$tile_container_trait as _;
                for tile in self.iter_tiles() {
                    if mut_accessed_types.contains(&tile.data().tile_type_id()) {
                        mut_accessed_tiles.insert(tile.grid_position());
                    }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_shared_from_regular {
    ($grid_shared_type:ident, $grid_type:ident) => {
        impl<Data, Shared> $grid_shared_type<Data, Shared>
        where
            Data: grid_forge_core::id::TypedData,
            Shared: grid_forge_core::id::SharedData,
        {
            /// Creates shared data grid from regular grid.
            ///
            /// The source grid will be consumed, and new grid will be created. Its internal shared data container
            /// will be created as empty - you can use [`import_shared_data`](Self::import_shared_data) to populate it.
            pub fn from_regular(regular: $grid_type<Data>) -> Self {
                let mut shared = Self::new(regular.size().clone());
                for tile in regular.drain() {
                    shared.insert_tile(tile);
                }
                shared
            }
        }
    };
}

/// Provides a starter for test suite for a given grid type.
///
/// Generated symbols consists of:
/// - `TestData` - testing [`TileData`](crate::TileData) for the test suite.
/// - `set_up_grid(&mut grid)` - initializes the grid with `TestData` tiles.
/// - Two predefined tests:
///     - `test_grid_read_access` - tests reading access to the grid.
///     - `test_grid_write_access` - tests writing access to the grid.
/// - Structs and functions to write further tests:
///     - `NeighbourTestCase` and `AllNeighboursTestCase` - structs to test neighbours of the tiles.
///     - `test_neigbhours` and `test_all_neigbhours` - functions to test neighbours of the tiles.
///
///
/// # Arguments
/// * `grid` - Identifier of the grid for test suite.
/// * `size` - Type of the size of the grid.
/// * `dimension_count` - Number of dimensions in the grid.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_grid_tests {
    (
        grid: $grid_type:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        direction: $direction_type:ty,
        direction_table: $direction_table_type:ident,
        tile_container_trait: $tile_container_trait:ident,
        dimension_count: $dimension_count:literal,
    ) => {

        use grid_forge_core::TileData;        

        /// Test data.
        ///
        /// It has `offset` `usize` field, containing the offset of the tile in the grid,
        /// and `varia` `usize` field, for testing mutability.
        struct TestData {
            offset: usize,
            varia: usize,
        }

        impl TestData {

            /// Creates a new test data with the specified offset. The `varia` field is initialized to 0.
            pub fn new(offset: usize) -> Self {
                Self {
                    offset: offset,
                    varia: 0,
                }
            }
        }

        impl TileData for TestData {}

        fn set_up_grid(grid: &mut $grid_type<TestData>) {
            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                grid.insert_data(&pos, TestData::new(size.offset(&pos)));
            }
        }

        #[test]
        fn test_grid_read_access() {
            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                let tile = grid.get_tile_at_position(&pos).unwrap();
                assert_eq!(
                    tile.grid_position(), pos,
                    "wrong position on position: {pos:?}; simple access"
                );
                assert_eq!(
                    tile.data().offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}; simple access"
                );
            }

            for pos in size.get_all_possible_positions() {
                let tile_data = grid.get_data_at_position(&pos).unwrap();
                assert_eq!(
                    tile_data.offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}; get_tile_at_position access"
                );
            }

            for tile in grid.iter_tiles() {
                assert_eq!(
                    tile.data().offset,
                    size.offset(&tile.grid_position()),
                    "wrong varia on position: {:?}; iter access", tile.grid_position()
                );
            }

            for tile in grid.drain() {
                assert_eq!(
                    tile.data().offset,
                    size.offset(&tile.grid_position()),
                    "wrong offset on position: {:?}; drain access", tile.grid_position()
                );
            }
        }

        #[test]
        fn test_grid_write_access() {
            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                let mut tile = grid.get_mut_tile_at_position(&pos).unwrap();
                tile.data().varia = size.offset(&pos);
            }

            for pos in size.get_all_possible_positions() {
                let tile = grid.get_data_at_position(&pos).unwrap();
                assert_eq!(
                    tile.offset,
                    size.offset(&pos),
                    "wrong offset on position: {pos:?}"
                );
                assert_eq!(tile.varia, tile.offset, "wrong varia on position: {pos:?}");
            }

            for pos in size.get_all_possible_positions() {
                let data = grid.get_mut_data_at_position(&pos).unwrap();
                data.varia *= 2;
            }

            for tile in grid.drain() {
                assert_eq!(
                    tile.data().varia,
                    size.offset(&tile.grid_position()) * 2,
                    "wrong offset on position: {:?}", tile.grid_position()
                );
            }
        }

        #[test]
        fn test_grid_write_access_with_tracking() {
            use std::collections::BTreeSet;

            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            // turn on tracking
            grid.mut_access_tracking(true);

            let size = *grid.size();
            let count_max = size.max_tile_count();

            // Get positions that should be tracked
            let positions_to_track = BTreeSet::from_iter(grid
                .get_all_positions()
                .into_iter()
                .filter(|pos| {
                    let offset = size.offset(pos);
                    offset < count_max / 2
                })
            );

            for pos in positions_to_track.iter() {
                grid.get_mut_tile_at_position(pos).unwrap();
            }

            let mut_accessed = BTreeSet::from_iter(grid.get_mut_accessed().iter().copied());

            for pos in positions_to_track.iter() {
                assert!(mut_accessed.contains(&pos), "pos: {pos:?} should be in mut_accessed");
            }
            for pos in mut_accessed.iter() {
                assert!(positions_to_track.contains(&pos), "pos: {pos:?} should be in positions_to_track");
            }

            let mut_accessed = BTreeSet::from_iter(grid.drain_mut_accessed().iter().copied());

            for pos in positions_to_track.iter() {
                assert!(mut_accessed.contains(&pos), "pos: {pos:?} should be in mut_accessed");
            }
            for pos in mut_accessed.iter() {
                assert!(positions_to_track.contains(&pos), "pos: {pos:?} should be in positions_to_track");
            }

            assert!(grid.get_mut_accessed().is_empty(), "mut_accessed should be empty");
        }

        struct NeighbourTestCase {
            pos: $position_type,
            direction: $direction_type,
            expected: Option<$position_type>,
        }

        impl NeighbourTestCase {
            const fn new(pos: $position_type, direction: $direction_type, expected: Option<$position_type>) -> Self {
                Self { pos, direction, expected }
            }
        }

        fn test_neigbhours(size: $size_type, cases: &[NeighbourTestCase]) {
            let mut grid = $grid_type::new(size);
            set_up_grid(&mut grid);

            for (
                i,
                NeighbourTestCase {
                    pos,
                    direction,
                    expected,
                },
            ) in cases.iter().enumerate() {
                let actual = grid.get_neighbour_at(pos, direction);
                match (actual, expected) {
                    (Some(neighbour), Some(expected_pos)) => {
                        assert_eq!(neighbour.grid_position(), *expected_pos, "wrong neighbour at position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                        assert_eq!(neighbour.data().offset, size.offset(&neighbour.grid_position()), "wrong offset on position: {pos:?}; direction: {direction:?}. Case: {i}, Size: {size:?}");
                    },
                    (Some(neighbour), None) =>
                        panic!("neigbour at position: {pos:?}; direction: {direction:?} should be None, but is: {neighbour_pos:?}. Case: {i}, Size: {size:?}", neighbour_pos = neighbour.grid_position()),
                    (None, Some(expected_pos)) =>
                        panic!("neigbour at position: {pos:?}; direction: {direction:?} should be {expected_pos:?}, but is None. Case: {i}, Size: {size:?}"),
                    (None, None) => {}

                }
            }
        }

        struct AllNeighboursTestCase {
             pos: $position_type,
            expected: $direction_table_type<Option<$position_type>>,
        }

        impl AllNeighboursTestCase {
            pub const fn new(pos: $position_type, expected: $direction_table_type<Option<$position_type>>) -> Self {
                Self { pos, expected }
            }
        }

        fn test_all_neigbhours(size: $size_type, cases: &[AllNeighboursTestCase]) {
            let mut grid = $grid_type::new(size);
            set_up_grid(&mut grid);

            for (
                i,
                AllNeighboursTestCase {
                    pos,
                    expected,
                },
            ) in cases.iter().enumerate() {
                let neighbours = grid.get_neighbours(pos);

                for direction in <$direction_type>::ALL {
                    let expected = &expected[direction];
                    let actual = &neighbours[direction].as_ref().and_then(|tile| Some(tile.grid_position()));

                    if expected.is_none() {
                        assert!(actual.is_none(), "neigbour at position: {pos:?}; direction: {direction:?} should be None, but is: {actual:?}. Case: {i}, Size: {size:?}");
                        continue;
                    } else {
                        assert!(actual.is_some(), "neigbour at position: {pos:?}; direction: {direction:?} should be {expected:?}, but is None. Case: {i}, Size: {size:?}");
                        assert_eq!(actual.unwrap(), expected.unwrap(), "neigbour at position: {pos:?}; direction: {direction:?} should be {expected:?}, but is {actual:?}. Case: {i}, Size: {size:?}");
                    }
                }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_shared_grid_tests {
    (
        grid: $grid_type:ident,
        size: $size_type:ty,
        position: $position_type:ty,
        direction: $direction_type:ty,
        direction_table: $direction_table_type:ident,
        tile_container_trait: $tile_container_trait:ident,
        dimension_count: $dimension_count:literal,
    ) => {
        use grid_forge_core::id::BasicTypedData;
        use grid_forge_core::id::IdDefault;
        use grid_forge_core::id::SharedData;
        use grid_forge_core::id::TypeIdMap;
        use grid_forge_core::id::TypedData;
        use grid_forge_core::id::WithSharedData;
        use crate::core::$tile_container_trait as _;
        use grid_forge_core::TileData;

        /// Test data.
        ///
        /// It has `offset` `usize` field, containing the offset of the tile in the grid,
        /// and `varia` `usize` field, for testing mutability.
        #[derive(Debug)]
        struct TestData {
            type_id: u64,
            varia: u64,
        }

        impl TestData {
            /// Creates a new test data with the specified offset. The `varia` field is initialized to 0.
            pub fn new(type_id: u64) -> Self {
                Self { type_id, varia: 0 }
            }
        }

        impl SharedData for TestData {}

        fn calc_type_id(grid_position: $position_type) -> u64 {
            grid_position.coords().into_iter().map(|x| x as u64).sum()
        }

        fn set_up_grid(grid: &mut $grid_type<BasicTypedData, TestData>) {
            let size = *grid.size();
            let mut ids = TypeIdMap::<TestData>::default();
            for pos in size.get_all_possible_positions() {
                let type_id = calc_type_id(pos);
                let data = BasicTypedData::tile_type_default(type_id);
                grid.insert_data(&pos, data);
                let shared = TestData::new(type_id);
                ids.insert(type_id, shared);
            }
            grid.import_shared_data(ids);
        }

        #[test]
        fn test_grid_read_access() {
            let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
            set_up_grid(&mut grid);

            assert_eq!(0, grid.check_shared_data().len());

            let size = *grid.size();
            for pos in size.get_all_possible_positions() {
                let tile = grid.get_tile_with_shared_at_position(&pos).unwrap();
                assert_eq!(
                    tile.grid_position(),
                    pos,
                    "wrong position on position: {pos:?}; simple access"
                );
                assert_eq!(
                    tile.data().tile_type_id(),
                    tile.shared_data().type_id,
                    "wrong offset on position: {pos:?}; simple access"
                );
            }

            // for pos in size.get_all_possible_positions() {
            //     let tile_data = grid.get_data_at_position(&pos).unwrap();
            //     assert_eq!(
            //         tile_data.offset,
            //         size.offset(&pos),
            //         "wrong offset on position: {pos:?}; get_tile_at_position access"
            //     );
            // }

            // for tile in grid.iter_tiles() {
            //     assert_eq!(
            //         tile.data().offset,
            //         size.offset(&tile.grid_position()),
            //         "wrong varia on position: {:?}; iter access", tile.grid_position()
            //     );
            // }

            // for tile in grid.drain() {
            //     assert_eq!(
            //         tile.data().offset,
            //         size.offset(&tile.grid_position()),
            //         "wrong offset on position: {:?}; drain access", tile.grid_position()
            //     );
            // }
        }

        // #[test]
        // fn test_grid_write_access() {
        //     let mut grid = $grid_type::new(<$size_type>::from_slice(&[10; $dimension_count]));
        //     set_up_grid(&mut grid);

        //     let size = *grid.size();
        //     for pos in size.get_all_possible_positions() {
        //         let mut tile = grid.get_mut_tile_at_position(&pos).unwrap();
        //         tile.data().varia = size.offset(&pos);
        //     }

        //     for pos in size.get_all_possible_positions() {
        //         let tile = grid.get_data_at_position(&pos).unwrap();
        //         assert_eq!(
        //             tile.offset,
        //             size.offset(&pos),
        //             "wrong offset on position: {pos:?}"
        //         );
        //         assert_eq!(tile.varia, tile.offset, "wrong varia on position: {pos:?}");
        //     }

        //     for pos in size.get_all_possible_positions() {
        //         let data = grid.get_mut_data_at_position(&pos).unwrap();
        //         data.varia *= 2;
        //     }

        //     for tile in grid.drain() {
        //         assert_eq!(
        //             tile.data().varia,
        //             size.offset(&tile.grid_position()) * 2,
        //             "wrong offset on position: {:?}", tile.grid_position()
        //         );
        //     }
        // }
    };
}
