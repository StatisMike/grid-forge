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

                for position in map.positions() {
                    if let Some(pattern) = instance.create_pattern(map, &position) {
                        let tile = PatternTileData::WithPattern {
                            tile_type_id: pattern.tile_type_id(),
                            pattern_id: pattern.pattern_id(),
                        };
                        collection.add_pattern(pattern);
                        instance.inner.insert_data(&position, tile);
                    } else if let Some(ident_tile) = map.tile_at(&position) {
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
                    let tiles = map.tiles_at(&positions);
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
macro_rules! __impl_pattern_collapsible_grid {
    (
        struct_name: $name:ident,
        pattern: $pattern:ident,
        collapsible_data: $collapsible_data:ident,
        collection: $collection:ident,
        propagate_item: $propagate_item:ident,
        per_option_data: $per_option_data:ident,
        adjacency_rules: $adjacency_rules:ident,
        frequency_hints: $frequency_hints:ident,
        collapsible_grid_error: $collapsible_grid_error:ident,
        collapsed_grid: $collapsed_grid:ident,
        grid: $grid:ident,
        grid_size: $grid_size:ident,
        tile: $tile:ident,
        position: $position:ident,
        direction: $direction:ident,
        size_consts: [$($size_const:ident),*],
    ) => {

        macro_rules! collapsible_grid {
            () => { $grid<$collapsible_data> };
        }

        pub struct $name<$(const $size_const: usize),*, Tile: TypedData> {
            pub(crate) pattern_grid: collapsible_grid!(),
            pub(crate) patterns: $collection<$($size_const),*>,
            pub(crate) option_data: $per_option_data,
            types: PhantomData<Tile>,
        }

        impl<$(const $size_const: usize),*, Tile: TypedData> Clone for $name<$($size_const),*, Tile> {
            fn clone(&self) -> Self {
                Self {
                    pattern_grid: self.pattern_grid.clone(),
                    patterns: self.patterns.clone(),
                    option_data: self.option_data.clone(),
                    types: self.types,
                }
            }
        }

        impl<$(const $size_const: usize),*, Tile: TypedData> $name<$($size_const),*, Tile>
        {
            pub fn new_empty(
                size: $grid_size,
                patterns: $collection<$($size_const),*>,
                frequencies: &$frequency_hints<$($size_const),*, Tile>,
                adjacencies: &$adjacency_rules<$($size_const),*, Tile>,
            ) -> Result<Self, $collapsible_grid_error> {
                let mut option_data = $per_option_data::default();
                option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner().clone());

                let pattern_ids: TypeIdSet =
                    TypeIdSet::from_iter(patterns.inner().values().map(|p| p.pattern_id()));
                let option_ids: TypeIdSet = TypeIdSet::from_iter(option_data.option_map.keys().copied());
                let mut missing_ids = pattern_ids
                    .symmetric_difference(&option_ids)
                    .copied()
                    .collect::<Vec<_>>();
                missing_ids.sort();

                if !missing_ids.is_empty() {
                    return Err($collapsible_grid_error::new_missing(missing_ids));
                }

                Ok(Self {
                    pattern_grid: $grid::new(size),
                    patterns,
                    option_data,
                    types: PhantomData,
                })
            }

            pub fn new_from_collapsed<R: Rng>(
                rng: &mut R,
                collapsed: &$collapsed_grid,
                patterns: $collection<$($size_const),*>,
                frequencies: &$frequency_hints<$($size_const),*, Tile>,
                adjacencies: &$adjacency_rules<$($size_const),*, Tile>,
            ) -> Result<Self, $collapsible_grid_error> {

                let mut option_data = $per_option_data::default();
                option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner().clone());

                let pattern_ids: TypeIdSet = TypeIdSet::from_iter(patterns.iter_tile_types().copied());
                let option_ids: TypeIdSet = TypeIdSet::from_iter(option_data.option_map.keys().copied());
                let mut missing_ids = pattern_ids
                    .symmetric_difference(&option_ids)
                    .copied()
                    .collect::<Vec<_>>();
                missing_ids.sort();
                if !missing_ids.is_empty() {
                    return Err($collapsible_grid_error::new_missing(missing_ids));
                }

                let mut grid = $grid::new(*collapsed.grid.size());

                for tile in
                    Self::collapsed_into_collapsible_pattern(rng, collapsed, &patterns, &option_data)?
                {
                    grid.insert(tile);
                }

                Ok(Self {
                    pattern_grid: grid,
                    patterns,
                    option_data,
                    types: PhantomData,
                })
            }

            pub fn retrieve_collapsed(&self) -> $collapsed_grid {
                let mut out = $collapsed_grid::new(self.pattern_grid.size().clone());

                for tile in self.pattern_grid.tiles() {
                    if !tile.data().is_collapsed() {
                        continue;
                    }

                    out.grid.insert_data(
                        &tile.grid_position(),
                        CollapsedTileData::new(
                            self.retrieve_tile_type_id(&tile)
                                .expect("cannot get `tile_type_id` for uncollapsed tile"),
                        ),
                    );
                }

                out
            }

            pub fn retrieve_ident<Builder: IdentTileBuilder<OutputTile>, OutputTile>(
                &self,
                builder: &Builder,
            ) -> Result<$grid<OutputTile>, $collapsible_grid_error>
            where
                Builder: IdentTileBuilder<OutputTile>,
                OutputTile: TypedData,
            {
                let mut out = $grid::<OutputTile>::new(*self.pattern_grid.size());

                for tile in self.pattern_grid.tiles() {
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

            pub fn retrieve_ident_default<OutputTile>(&self) -> $grid<OutputTile>
            where
                OutputTile: TypedData + IdDefault,
            {
                let mut out = $grid::<OutputTile>::new(*self.pattern_grid.size());

                for tile in self.pattern_grid.tiles() {
                    if !tile.data().is_collapsed() {
                        continue;
                    }
                    out.insert_data(
                        &tile.grid_position(),
                        OutputTile::tile_type_default(
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

            /// Returns all possitions in the internal grid holding either collapsed or uncollapsed tiles.
            pub fn retrieve_positions(&self, collapsed: bool) -> Vec<$position> {
                let func: fn(&$collapsible_data) -> bool = if collapsed {
                    |d| d.is_collapsed()
                } else {
                    |d| !d.is_collapsed()
                };
                self.pattern_grid
                    .enumerate()
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
                for t in self.pattern_grid.iter_mut() {
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
                let mut cache = TypeIdMap::default();
                let mut check_generated = HashSet::<$position>::default();
                let check_provided = HashSet::<$position>::from_iter(to_collapse.iter().copied());

                for pos_to_collapse in to_collapse {
                    for neighbour_tile in self.pattern_grid.neighbors(pos_to_collapse).inner().iter().flatten() {
                        if !neighbour_tile.as_ref().is_collapsed()
                            || check_provided.contains(&neighbour_tile.grid_position())
                            || check_generated.contains(&neighbour_tile.grid_position())
                        {
                            continue;
                        }
                        check_generated.insert(neighbour_tile.grid_position());
                        let collapsed_idx = neighbour_tile.as_ref().collapsed_idx().unwrap();
                        for opt_to_remove in cache.entry(collapsed_idx as u64).or_insert_with(|| {
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
                    if let Some(mut tile) = grid.neighbor_at_mut(collapsed_position, &direction) {
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
                    if let Some(tile) = grid.neighbor_at(position, &direction) {
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

                let tile = grid.data_at_mut(position).unwrap();
                for (possible, (option_idx, weights)) in
                    possible_options.iter().zip(option_data.iter_weights())
                {
                    if !possible && tile.mut_ways_to_be_option().purge_option(option_idx) {
                        tile.remove_option(*weights);
                    }
                }
                true
            }


            fn collapsed_into_collapsible_pattern<R: Rng>(
                rng: &mut R,
                collapsed: &$collapsed_grid,
                patterns: &$collection<$($size_const),*>,
                options: &$per_option_data,
            ) -> Result<Vec<$tile<$collapsible_data>>, $collapsible_grid_error> {
                let entrophy_uniform = EntrophyUniform::new();
                let ways = &options.ways_to_be_option;
                let mut out = Vec::new();

                for position in collapsed.grid.positions() {
                    let mut possible_patterns = Vec::new();
                    let tile_type_id = collapsed
                        .grid
                        .tile_at(&position)
                        .unwrap()
                        .as_ref()
                        .tile_type_id();
                    'pat_loop: for pattern in patterns.get_patterns_for_tile(tile_type_id) {
                        for pos_to_check in $pattern::<$($size_const),*>::secondary_tile_positions(&position) {
                            if let Some(tile_type_id) = collapsed
                                .grid
                                .tile_at(&pos_to_check)
                                .map(|t| t.as_ref().tile_type_id())
                            {
                                if tile_type_id != pattern.get_id_for_pos(&position, &pos_to_check) {
                                    continue 'pat_loop;
                                }
                            }
                        }

                        possible_patterns.push(
                            options
                                .get_tile_offset(pattern.pattern_id())
                                .expect("cannot get pattern idx"),
                        );
                    }
                    if possible_patterns.is_empty() {
                        return Err($collapsible_grid_error::new_collapse(position));
                    }
                    let mut current_ways = ways.clone();
                    current_ways.purge_others(&possible_patterns);

                    let num_options = possible_patterns.len();

                    let mut weights = OptionWeights::default();
                    for pattern in possible_patterns {
                        let w = options.get_weights(pattern);
                        weights.0 += w.0;
                        weights.1 += w.1;
                    }

                    out.push(
                        $tile::new(
                            position,
                            $collapsible_data::new_uncollapsed_tile(
                                num_options,
                                current_ways,
                                weights,
                                entrophy_uniform.sample(rng),
                            )
                        )
                    );
                }

                Ok(out)
            }

            fn retrieve_tile_type_id(&self, tile: &impl AsRef<$collapsible_data>) -> Option<u64> {
                match tile.as_ref().collapsed_idx() {
                    Some(pattern_idx) => {
                        let pattern_id = self.option_data.get_tile_type_id(pattern_idx).unwrap();

                        let pattern = self.patterns.get_pattern(pattern_id).unwrap();
                        Some(pattern.tile_type_id())
                    }
                    None => None,
                }
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
            min_weight: Option<u32>,
            max_weight: Option<u32>,
        }

        impl<$(const $size_const: usize),*, Data: TypedData> Clone for $name<$($size_const),*, Data> {
            fn clone(&self) -> Self {
                Self {
                    weights: self.weights.clone(),
                    data_type: self.data_type,
                    min_weight: self.min_weight,
                    max_weight: self.max_weight,
                }
            }
        }


        impl<$(const $size_const: usize),*, Data: TypedData> Default for $name<$($size_const),*, Data>
        {
            fn default() -> Self {
                Self {
                    weights: BTreeMap::default(),
                    data_type: PhantomData,
                    min_weight: None,
                    max_weight: None,
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

            pub fn with_min_weight(mut self, min_weight: u32) -> Self {
                self.min_weight = Some(min_weight);
                self
            }
            pub fn with_max_weight(mut self, max_weight: u32) -> Self {
                self.max_weight = Some(max_weight);
                self
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
                for tile in grid.inner().tiles() {
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

#[macro_export]
macro_rules! __impl_pattern_subscriber_trait {
    (
        trait_name: $trait_name:ident,
        position: $position:ty,
    ) => {
        /// When applied to the struct allows injecting it into [`singular::Resolver`](Resolver) to react on each tile being collapsed.
        pub trait $trait_name: Any {
            /// Called when the generation process starts. No-op by default, should be overridden to clear the state of the subcscriber
            /// if it retains any state.
            fn on_generation_start(&mut self) {
                // no-op by default
            }

            /// Called when a tile is collapsed.
            fn on_collapse(&mut self, position: &$position, tile_type_id: u64, pattern_id: u64) {
                // no-op by default
            }

            /// To retrieve the concrete subscriber type from [`singular::Resolver`](Resolver).
            fn as_any(&self) -> &dyn Any;
        }
    };
}

#[macro_export]
macro_rules! __impl_pattern_resolver {
    (
        struct_name: $name:ident,
        subscriber_trait: $subscriber_trait:ident,
        collapsible_grid: $collapsible_grid:ident,
        collapsible_tile: $collapsible_tile:ident,
        propagate_item: $propagate_item:ident,
        propagator: $propagator:ident,
        entrophy_queue: $entrophy_queue:ident,
        position_queue: $position_queue:ident,
        collapse_error: $collapse_error:ident,
        position: $position:ty,
        size_consts: [$($size_const:ident),*],
    ) => {
        pub struct $name<$(const $size_const: usize),*, Data>
        {
            subscriber: Option<Box<dyn $subscriber_trait>>,
            tile_type: PhantomData<Data>,
        }

        impl<$(const $size_const: usize),*, Data: TypedData> Default for $name<$($size_const),*, Data>
        {
            fn default() -> Self {
                Self {
                    subscriber: None,
                    tile_type: PhantomData,
                }
            }
        }

        impl<$(const $size_const: usize),*, Data: TypedData> $name<$($size_const),*, Data>
        {
            pub fn with_subscriber(mut self, subscriber: Box<dyn $subscriber_trait>) -> Self {
                self.subscriber = Some(subscriber);
                self
            }

            /// Retrieve the subscriber attached to the resolver.
            pub fn retrieve_subscriber(&mut self) -> Option<Box<dyn $subscriber_trait>> {
                self.subscriber.take()
            }

            pub fn generate_entrophy<R: Rng>(
                &mut self,
                mut grid: $collapsible_grid<$($size_const),*, Data>,
                rng: &mut R,
                positions: &[$position],
            ) -> Result<$collapsible_grid<$($size_const),*, Data>, $collapse_error>
            {

                let mut iter = 0;
                let mut queue = $entrophy_queue::default();
                let mut propagator = $propagator::default();

                if let Some(subscriber) = self.subscriber.as_mut() {
                    subscriber.on_generation_start();
                }

                grid.remove_uncollapsed();

                let option_data = &grid.option_data;
                let tiles =
                    $collapsible_tile::new_from_frequency(positions, option_data);

                for tile in tiles {
                    queue.update_queue(tile.0, tile.1.calc_entrophy());
                    grid.pattern_grid.insert_data(&tile.0, tile.1);
                }

                for initial_propagate in grid.get_initial_propagate_items(positions) {
                    propagator.push_propagate(initial_propagate);
                }

                $collapse_error::from_result(
                    propagator.propagate(&mut grid.pattern_grid, &grid.option_data, &mut queue),
                    CollapseErrorKind::Init,
                    iter,
                )?;

                while let Some(collapse_position) = queue.get_next_position() {
                    let to_collapse = grid
                        .pattern_grid
                        .data_at_mut(&collapse_position)
                        .unwrap();

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

                    let removed_options = to_collapse
                        .collapse_gather_removed(rng, &grid.option_data);
                    let collapsed_idx = to_collapse.collapsed_idx().unwrap();

                    if let Some(subscriber) = self.subscriber.as_mut() {
                        let pattern_id = grid.option_data.get_tile_type_id(collapsed_idx).unwrap();
                        let collapsed_id = grid
                            .patterns
                            .get_pattern(pattern_id)
                            .unwrap()
                            .tile_type_id();

                        subscriber.on_collapse(&collapse_position, collapsed_id, pattern_id);
                    }

                    for removed_option in removed_options.into_iter() {
                        propagator.push_propagate($propagate_item::new(collapse_position, removed_option))
                    }

                    $collapse_error::from_result(
                        propagator.propagate(&mut grid.pattern_grid, &grid.option_data, &mut queue),
                        CollapseErrorKind::Propagation,
                        iter,
                    )?;
                    iter += 1;
                }

                Ok(grid)
            }

            pub fn generate_position<R: Rng>(
                &mut self,
                mut grid: $collapsible_grid<$($size_const),*, Data>,
                rng: &mut R,
                position: &[$position],
                mut queue: $position_queue,
            ) -> Result<$collapsible_grid<$($size_const),*, Data>, $collapse_error>
            {
                let mut iter = 0;

                if let Some(subscriber) = self.subscriber.as_mut() {
                    subscriber.on_generation_start();
                }

                grid.remove_uncollapsed();

                let option_data = &grid.option_data;
                let tiles =
                    $collapsible_tile::new_from_frequency(position, option_data);

                for tile in tiles {
                    queue.update_queue(tile.0);
                    grid.pattern_grid.insert_data(&tile.0, tile.1);
                }

                while let Some(collapse_position) = queue.get_next_position() {
                    let to_collapse = grid
                        .pattern_grid
                        .data_at(&collapse_position)
                        .unwrap();
                    // skip collapsed.
                    if to_collapse.is_collapsed() {
                        continue;
                    }

                    if !to_collapse.has_compatible_options()
                        || !$collapsible_grid::<$($size_const),*,Data>::purge_incompatible_options(
                            &mut grid.pattern_grid,
                            &collapse_position,
                            &grid.option_data,
                        )
                    {
                        return Err($collapse_error::new(
                            collapse_position,
                            CollapseErrorKind::Collapse,
                            iter,
                        ));
                    }

                    let to_collapse = grid
                    .pattern_grid
                    .data_at_mut(&collapse_position)
                    .unwrap();

                    to_collapse.collapse_basic(rng, &grid.option_data);
                    let collapsed_idx = to_collapse.collapsed_idx().unwrap();
                    $collapsible_grid::<$($size_const),*,Data>::purge_options_for_neighbours(
                        &mut grid.pattern_grid,
                        collapsed_idx,
                        &collapse_position,
                        &grid.option_data,
                    );

                    if let Some(subscriber) = self.subscriber.as_mut() {
                        let pattern_id = grid.option_data.get_tile_type_id(collapsed_idx).unwrap();
                        let collapsed_id = grid
                            .patterns
                            .get_pattern(pattern_id)
                            .unwrap()
                            .tile_type_id();

                        subscriber.on_collapse(&collapse_position, collapsed_id, pattern_id);
                    }
                    iter += 1;
                }
                Ok(grid)
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_pattern_debug_subscriber {
    (
        struct_name: $name:ident,
        trait_name: $trait_name:ident,
        position: $position:ty,
    ) => {

        use std::io::Write as _;

        impl $trait_name for $name {
            fn on_collapse(&mut self, position: &$position, tile_type_id: u64, pattern_id: u64) {
                if let Some(file) = &mut self.file {
                    writeln!(
                        file,
                        "collapsed tile_type_id: {tile_type_id} on position: {position:?}; pattern_id: {pattern_id}"
                    )
                    .unwrap();
                } else {
                    println!("collapsed tile_type_id: {tile_type_id} on position: {position:?}; pattern_id: {pattern_id}");
                }
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}

/// Implements a collapse history subscriber logic for pattern-based collapse procedural algorithm.
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_pattern_history_subscriber {
    (
        struct_name: $name:ident,
        history_item_name: $history_item:ident,
        trait_name: $trait_name:ident,
        position: $position:ty,
    ) => {
        impl $trait_name for $name {
            fn on_generation_start(&mut self) {
                self.history.clear();
            }

            fn on_collapse(&mut self, position: &$position, tile_type_id: u64, pattern_id: u64) {
                self.history.push($history_item {
                    position: *position,
                    tile_type_id,
                    pattern_id: Some(pattern_id),
                });
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}
