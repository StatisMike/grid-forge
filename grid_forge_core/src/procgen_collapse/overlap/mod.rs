use crate::TileData;

/// Tile data of inner grid within [`OverlappingPatternGrid`].
#[derive(Debug, Clone)]
pub enum PatternTileData {
    /// Tile which besides containing information about `tile_type_id` of the original [`TypedData`],
    /// is also a first tile of pattern with given `pattern_id` (so it is in first position within the pattern).
    WithPattern { tile_type_id: u64, pattern_id: u64 },
    /// Tile which contains only information about `tile_type_id` of original [`TypedData`]. No pattern
    /// information is held, as its position makes it impossible to be first tile of any pattern.
    OnlyId { tile_type_id: u64 },
}

impl TileData for PatternTileData {}

#[macro_export]
macro_rules! __impl_pattern_collection {
    (
        struct_name: $name:ident,
        pattern_struct: $pattern_struct:ident,
        size_consts: [$($size_const:ident),*],
    ) => {
        /// Collection holding all found patterns found in sample maps.
        #[derive(Debug, Clone)]
        pub struct $name<$(const $size_const: usize),*> {
            inner: TypeIdMap<$pattern_struct<$($size_const),*>>,
            rev: TypeIdMap<u64>,
            by_tile_id: TypeIdMap<TypeIdSet>,
        }

        impl<$(const $size_const: usize),*> Default for $name<$($size_const),*> { 
            fn default() -> Self {
                Self {
                    inner: TypeIdMap::default(),
                    rev: TypeIdMap::default(),
                    by_tile_id: TypeIdMap::default(),
                }
            }
        }

        impl<$(const $size_const: usize),*> $name<$($size_const),*> {
            pub fn get_patterns_for_tile(&self, tile_type_id: u64) -> Vec<&$pattern_struct<$($size_const),*>> {
                if let Some(patterns) = self.by_tile_id.get(&tile_type_id) {
                    patterns
                        .iter()
                        .filter_map(|pattern_id| self.inner.get(pattern_id))
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            }

            pub fn iter_tile_types(&self) -> impl Iterator<Item = &u64> {
                self.by_tile_id.keys()
            }

            pub fn add_pattern(&mut self, data: $pattern_struct<$($size_const),*>) -> bool {;
                if self.inner.contains_key(&data.pattern_id()) {
                    return false;
                }
                self.inner.insert(data.pattern_id(), data);
                
                match self.by_tile_id.entry(data.tile_type_id()) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        e.get_mut().insert(data.pattern_id());
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(TypeIdSet::from_iter([data.pattern_id()]));
                    }
                }

                return true;
            }

            pub fn get_pattern(&self, pattern_id: u64) -> Option<&$pattern_struct<$($size_const),*>> {
                self.inner.get(&pattern_id)
            }

            pub fn remove_pattern(&mut self, pattern_id: u64) -> Option<$pattern_struct<$($size_const),*>> {
                if let Some(pattern) = self.inner.remove(&pattern_id) {
                    for by_tile_id in self.by_tile_id.values_mut() {
                        by_tile_id.remove(&pattern_id);
                    }
                    return Some(pattern);
                }
                None
            }

            pub (crate) fn inner(&self) -> &TypeIdMap<$pattern_struct<$($size_const),*>> {
                &self.inner
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_pattern_grid {
    (
        struct_name: $name:ident,
        size_consts: [$($size_const:ident),* $(,)?],
        pattern: $pattern_struct:ident,
        grid: $grid_struct:ident,
        collection: $collection_struct:ident,
        position: $position:ident,
        grid_size: $grid_size:ident,
    ) => {
        /// Grid containing pattern data derived from original [`GridMap2D`].
        #[derive(Debug, Clone)]
        pub struct $name<$(const $size_const: usize),*> {
            inner: $grid_struct<PatternTileData>,
        }

        impl<$(const $size_const: usize),*> $name<$($size_const),*> {
            /// Prepare new instance out of [`GridMap2D`], populating provided [`PatternCollection`] in the process.
            pub fn from_map<Data: TypedData>(
                map: &$grid_struct<Data>,
                collection: &mut $collection_struct<$($size_const),*>,
            ) -> Self {
                let mut instance = Self {
                    inner: $grid_struct::new(*map.size()),
                };

                for position in map.get_all_positions() {
                    if let Some(pattern) = instance.create_pattern(map, &position) {
                        let tile = PatternTileData::WithPattern {
                            tile_type_id: pattern.tile_type_id(),
                            pattern_id: pattern.pattern_id(),
                        };
                        collection.add_pattern(pattern);
                        instance.inner.insert_data(&position, tile);
                    } else if let Some(ident_tile) = map.get_tile_at_position(&position) {
                        let tile = PatternTileData::OnlyId {
                            tile_type_id: ident_tile.as_ref().tile_type_id(),
                        };
                        instance.inner.insert_data(&position, tile);
                    }
                }

                instance
            }

            /// Gets a reference to inner [`GridMap2D`] containing [`PatternTileData`].
            ///
            /// Useful for getting insight into pattern extracted from specific portion of original map.
            pub fn inner(&self) -> &$grid_struct<PatternTileData> {
                &self.inner
            }

            fn create_pattern<Data: TypedData>(
                &self,
                map: &$grid_struct<Data>,
                anchor_pos: &$position,
            ) -> Option<$pattern_struct<$($size_const),*>> {
                if let Some(positions) = self.generate_pattern_positions(anchor_pos, map.size()) {
                    let mut pattern = $pattern_struct::empty();
                    let tiles = map.get_tiles_at_positions(&positions);
                    for tile in tiles {
                        pattern.set_id_for_pos(
                            anchor_pos,
                            &tile.grid_position(),
                            tile.data().tile_type_id(),
                        );
                    }
                    pattern.finalize();
                    return Some(pattern);
                }
                None
            }

            fn generate_pattern_positions(
                &self,
                from: &$position,
                size: &$grid_size,
            ) -> Option<Vec<$position>> {
                let mut to = *from;
                to += $position::new($($size_const as u32 - 1),*);
                // to += $position::new((SIZE_X - 1) as u32, (SIZE_Y - 1) as u32);
                if !size.is_position_valid(&to) {
                    return None;
                }
                Some($position::generate_rect_area(from, &to))
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_pattern_frequency_hints {
    (
        struct_name: $name:ident,
        pattern: $pattern:ident,
        pattern_grid: $pattern_grid:ident,
        size_consts: [$($size_const:ident),*],
    ) => {
        /// Frequency hints for the *overlapping pattern*-based generative algorithm.
        ///
        /// Describes the frequency of occurence of each discovered pattern. Generated automatically while analyzing sample
        /// maps with [`Analyzer`], though afterwards frequencies could be tweaked manually.
        #[derive(Debug)]
        pub struct $name<$(const $size_const: usize),*, Data: TypedData> 
        {
            weights: BTreeMap<u64, u32>,
            data_type: PhantomData<Data>,
        }

        impl<$(const $size_const: usize),*, Data: TypedData> Clone for $name<$($size_const),*, Data> {
            fn clone(&self) -> Self {
                Self {
                    weights: self.weights.clone(),
                    data_type: self.data_type,
                }
            }
        }


        impl<$(const $size_const: usize),*, Data: TypedData> Default for $name<$($size_const),*, Data>
        {
            fn default() -> Self {
                Self {
                    weights: BTreeMap::default(),
                    data_type: PhantomData,
                }
            }
        }

        impl<$(const $size_const: usize),*, Data: TypedData> $name<$($size_const),*, Data>
        {
            pub fn set_weight_for_pattern(&mut self, pattern: &$pattern<$($size_const),*>, weight: u32) {
                let entry = self.weights.entry(pattern.pattern_id()).or_default();
                *entry = weight;
            }

            pub fn set_weight_for_pattern_id(&mut self, pattern_id: u64, weight: u32) {
                let entry = self.weights.entry(pattern_id).or_default();
                *entry = weight;
            }

            pub(crate) fn count_pattern(&mut self, pattern_id: u64) {
                if let Some(count) = self.weights.get_mut(&pattern_id) {
                    *count += 1;
                } else {
                    self.weights.insert(pattern_id, 1);
                }
            }

            pub(crate) fn get_all_weights_cloned(&self) -> BTreeMap<u64, u32> {
                self.weights.clone()
            }

            pub fn get_weight_for_pattern(&self, pattern_id: u64) -> u32 {
                *self.weights.get(&pattern_id).unwrap_or(&0)
            }

            pub fn analyze_pattern_grid(&mut self, grid: &$pattern_grid<$($size_const),*>) {
                for tile in grid.inner().iter_tiles() {
                    if let PatternTileData::WithPattern {
                        tile_type_id: _,
                        pattern_id,
                    } = tile.as_ref() {
                        self.count_pattern(*pattern_id);
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_pattern_adjacency_rules {
    (
        struct_name: $name:ident,
        pattern: $pattern:ident,
        pattern_grid: $pattern_grid:ident,
        collection: $collection:ident,
        adjacency_table: $adjacency_table:ident,
        direction: $direction:ident,
        size_consts: [$($size_const:ident),*],
    ) => {
        /// Adjacency rules for the *overlapping pattern*-based generative algorithm.
        ///
        /// Contrary to [`singular::AdjacencyRules`](crate::gen::collapse::singular::AdjacencyRules), these rules are not based directly on the
        /// neighbouring tiles found within the sample maps, but on the `tile_type_id`s hold within the patterns.
        ///
        /// Two patterns are considered compatible in given direction if the overlapping tiles contained within them are identical.
        #[derive(Clone, Debug)]
        pub struct $name<$(const $size_const: usize),*, Data: TypedData> 
        {
            inner: $adjacency_table,
            data_type: PhantomData<Data>,
        }

        impl<$(const $size_const: usize),*, Data: TypedData> Default for $name<$($size_const),*, Data>
        {
            fn default() -> Self {
                Self {
                    inner: $adjacency_table::default(),
                    data_type: PhantomData,
                }
            }
        }

        impl<$(const $size_const: usize),*, Data: TypedData> $name<$($size_const),*, Data>
        {
            /// Analyzes the [`PatternCollection`] to find out which patterns are compatible with each other.
            pub fn analyze_collection(&mut self, collection: &$collection<$($size_const),*>) {
                for (id_outer, pat_outer) in collection.inner().iter() {
                    for (id_inner, pat_inner) in collection.inner().iter() {
                        for direction in $direction::ALL {
                            if pat_outer.is_compatible_with(pat_inner, direction) {
                                self.inner
                                    .insert_adjacency(*id_outer, direction, *id_inner);
                            }
                        }
                    }
                }
            }

            pub(crate) fn inner(&self) -> &$adjacency_table {
                &self.inner
            }

            pub fn is_valid_at_dir(
                &self,
                pattern_id: u64,
                direction: $direction,
                other_pattern_id: u64,
            ) -> bool {
                let Some(adj) = self.inner().as_ref().get(&pattern_id) else {
                    return false;
                };
                adj.as_ref()[direction].contains(&other_pattern_id)
            }
        }

        impl<$(const $size_const: usize),*, Data: TypedData> AsRef<$adjacency_table> for $name<$($size_const),*, Data>
        {
            fn as_ref(&self) -> &$adjacency_table {
                &self.inner
            }
        }

    }
}

#[macro_export]
macro_rules! __impl_pattern_analyzer {
    ( 
        struct_name: $name:ident,
        pattern: $pattern:ident,
        pattern_grid: $pattern_grid:ident,
        collection: $collection:ident,
        frequency_hints: $frequency_hints:ident,
        adjacency_rules: $adjacency_rules:ident,
        grid: $grid:ident,
        size_consts: [$($size_const:ident),*],
    ) => {

        #[derive(Debug, Clone)]
        /// GridMap analyzer for overlapping pattern data.
        ///
        /// It allows analyzing the [`GridMap2D`] of [`TypedData`], producing all elements necessary for
        /// creation of [`CollapsiblePatternGrid`](crate::gen::collapse::overlap::CollapsiblePatternGrid) for
        /// [`overlap::Resolver`](crate::gen::collapse::overlap::Resolver) to collapse into new map.
        pub struct $name<$(const $size_const: usize),*, Data: TypedData> 
        {
            collection: $collection<$($size_const),*>,
            frequency: $frequency_hints<$($size_const),*, Data>,
            adjacency: $adjacency_rules<$($size_const),*, Data>,
        }

        impl<$(const $size_const: usize),*, Data: TypedData> Default for $name<$($size_const),*, Data>
        {
            fn default() -> Self {
                Self {
                    collection: $collection::default(),
                    frequency: $frequency_hints::default(),
                    adjacency: $adjacency_rules::default(),
                }
            }
        }

        impl<$(const $size_const: usize),*, Data: TypedData> $name<$($size_const),*, Data> {
            /// Analyzes the [`GridMap2D`] of [`TypedData`], gathering elements necessary for creation of new
            /// [`CollapsiblePatternGrid`](crate::gen::collapse::overlap::CollapsiblePatternGrid) to collapse.
            ///
            /// Returns [`OverlappingPatternGrid`], which is a transformed source map if more insights about which patterns
            /// were discoveren in specific positions on the map.
            pub fn analyze(&mut self, map: &$grid<Data>) -> $pattern_grid<$($size_const),*> {
                let grid = $pattern_grid::from_map(map, &mut self.collection);
                self.frequency.analyze_pattern_grid(&grid);
                self.adjacency.analyze_collection(&self.collection);

                grid
            }

            pub fn get_collection(&self) -> &$collection<$($size_const),*> {
                &self.collection
            }

            pub fn get_frequency(&self) -> &$frequency_hints<$($size_const),*, Data> {
                &self.frequency
            }

            pub fn get_adjacency(&self) -> &$adjacency_rules<$($size_const),*, Data> {
                &self.adjacency
            }
        }
    }
}