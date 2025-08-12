use std::ops::{Add, Sub, SubAssign};

use crate::utils::OrderedFloat;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionWeights(pub u32, pub f32);

impl OptionWeights {
    pub fn new(option_weight: u32) -> Self {
        Self(option_weight, Self::calc_weigth(option_weight as f32))
    }

    pub fn calc_weigth(weight: f32) -> f32 {
        let weight = weight * weight.log2();
        (weight % OrderedFloat::EPSILON) * OrderedFloat::EPSILON
    }

    pub fn round(&mut self) {
        self.1 = (self.1 % OrderedFloat::EPSILON) * OrderedFloat::EPSILON;
    }
}

impl Add for OptionWeights {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for OptionWeights {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for OptionWeights {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.round();
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_collapse_adjacencies {
    (
        struct_name: $struct_name:ident,
        direction: $direction:ident,
        direction_table: $direction_table:ident,
        direction_count: $direction_count:literal,
    ) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Debug)]
        pub struct $struct_name {
            inner: $direction_table<TypeIdSet>,
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    inner: $direction_table::new(core::array::from_fn(|_| TypeIdSet::default())),
                }
            }

            #[inline]
            pub fn add_at_dir(&mut self, direction: $direction, id: u64) {
                self.inner[direction].insert(id);
            }
        }

        impl std::ops::Index<$direction> for $struct_name {
            type Output = TypeIdSet;

            #[inline]
            fn index(&self, index: $direction) -> &Self::Output {
                &self.inner[index]
            }
        }

        impl AsRef<$direction_table<TypeIdSet>> for $struct_name {
            fn as_ref(&self) -> &$direction_table<TypeIdSet> {
                &self.inner
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_adjacency_table {
    (
        struct_name: $struct_name:ident,
        adjacencies: $adjacencies:ident,
        direction: $direction:ident,
    ) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Debug)]
        pub struct $struct_name {
            inner: TypeIdMap<$adjacencies>,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    inner: TypeIdMap::<$adjacencies>::default(),
                }
            }
        }

        impl AsRef<TypeIdMap<$adjacencies>> for $struct_name {
            fn as_ref(&self) -> &TypeIdMap<$adjacencies> {
                &self.inner
            }
        }

        impl $struct_name {
            pub(crate) fn insert_adjacency(
                &mut self,
                el_id: u64,
                direction: $direction,
                adj_id: u64,
            ) {
                match self.inner.entry(el_id) {
                    Entry::Occupied(mut e) => e.get_mut().add_at_dir(direction, adj_id),
                    Entry::Vacant(e) => {
                        let mut adjacencies = $adjacencies::new();
                        adjacencies.add_at_dir(direction, adj_id);
                        e.insert(adjacencies);
                    }
                }
            }

            pub(crate) fn get_all_adjacencies_in_direction(
                &self,
                el_id: &u64,
                direction: &$direction,
            ) -> impl Iterator<Item = &u64> {
                self.inner
                    .get(el_id)
                    .expect("cannot get adjacencies for provided `el_id`")[*direction]
                    .iter()
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_ways_to_be_option {
    (
        struct_name: $struct_name:ident,
        direction: $direction:ident,
        direction_table: $direction_table:ident,
        direction_count: $direction_count:literal,
    ) => {
        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            inner: Vec<$direction_table<usize>>,
        }

        impl $struct_name {
            const EMPTY_DIR_TABLE: $direction_table<usize> =
                $direction_table::new([0; $direction_count]);

            /// Decrements number of ways to become option from given direction. If reaches
            /// 0, returns `true` and given option should be removed.
            pub(crate) fn decrement(&mut self, option_idx: usize, direction: $direction) -> bool {
                if self.inner[option_idx][direction] == 0 {
                    return false;
                }
                self.inner[option_idx][direction] -= 1;
                if self.inner[option_idx][direction] > 0 {
                    return false;
                }
                self.inner[option_idx] = Self::EMPTY_DIR_TABLE;
                true
            }

            pub(crate) fn iter_possible(&self) -> impl Iterator<Item = usize> + '_ {
                self.inner.iter().enumerate().filter_map(|(idx, t)| {
                    if t[$direction::from_idx(0).unwrap()] == 0 {
                        None
                    } else {
                        Some(idx)
                    }
                })
            }

            pub(crate) fn purge_others(&mut self, options: &[usize]) {
                for (option_id, ways) in self.inner.iter_mut().enumerate() {
                    if options.contains(&option_id) {
                        continue;
                    }
                    *ways = Self::EMPTY_DIR_TABLE;
                }
            }

            pub(crate) fn purge_option(&mut self, option_idx: usize) -> bool {
                if self.inner[option_idx]
                    .inner()
                    .as_ref()
                    .iter()
                    .all(|i| i == &0)
                {
                    return false;
                }
                self.inner[option_idx] = Self::EMPTY_DIR_TABLE;
                true
            }

            pub(crate) fn insert_from_slice(&mut self, slice: &[usize]) {
                let mut inner = [0; $direction_count];
                inner.copy_from_slice(slice);
                self.inner.push($direction_table::new(inner));
            }

            pub(crate) fn insert_empty(&mut self) {
                self.inner.push(Self::EMPTY_DIR_TABLE);
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_per_option_data {
    (
        struct_name: $struct_name:ident,
        direction: $direction:ident,
        direction_table: $direction_table:ident,
        adjacency_table: $adjacency_table:ident,
        ways_to_be_option: $ways:ident,
    ) => {
        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            pub(crate) option_map: TypeIdMap<usize>,
            option_map_rev: TypeIdMap<u64>,
            adjacencies: Vec<$direction_table<Vec<usize>>>,
            pub(crate) ways_to_be_option: $ways,
            opt_with_weight: Vec<OptionWeights>,
            pub(crate) option_count: usize,
            pub(crate) possible_options_count: usize,
        }

        impl $struct_name {
            pub(crate) fn populate(
                &mut self,
                options_with_weights: &BTreeMap<u64, u32>,
                adjacencies: $adjacency_table,
            ) {
                for (n, (option_id, option_weight)) in options_with_weights.iter().enumerate() {
                    self.add_tile_offset(*option_id, n);
                    self.opt_with_weight
                        .push(OptionWeights::new(*option_weight));
                }

                self.option_count = self.option_map.len();
                self.possible_options_count = self.option_count;

                for trans_id in 0..self.option_count {
                    let original_id = self.get_tile_type_id(trans_id).unwrap();
                    let translated_table =
                        self.translate_adjacency_table(original_id, &adjacencies);
                    self.adjacencies.push(translated_table);
                }

                self.generate_ways_to_be_option();
            }

            pub(crate) fn generate_ways_to_be_option(&mut self) {
                for adj in self.adjacencies.iter() {
                    let table = $direction::ALL
                        .iter()
                        .map(|dir| adj[*dir].len())
                        .collect::<Vec<usize>>();
                    if table.contains(&0) {
                        self.possible_options_count -= 1;
                        self.ways_to_be_option.insert_empty();
                    } else {
                        self.ways_to_be_option.insert_from_slice(&table);
                    }
                }
            }

            pub(crate) fn get_all_enabled_in_direction(
                &self,
                option_id: usize,
                direction: $direction,
            ) -> &[usize] {
                &self.adjacencies[option_id][direction]
            }

            pub(crate) fn translate_adjacency_table(
                &self,
                original_id: u64,
                adjacencies: &$adjacency_table,
            ) -> $direction_table<Vec<usize>> {
                let mut translated_table = $direction_table::default();
                if let Some(adj) = adjacencies.inner.get(&original_id) {
                    for dir in $direction::ALL.iter() {
                        translated_table[*dir] = adj[*dir]
                            .iter()
                            .map(|&id| self.get_tile_offset(id).expect("cannot get mapped id"))
                            .collect();
                    }
                }
                translated_table
            }

            pub(crate) fn get_weights(&self, option_idx: usize) -> OptionWeights {
                self.opt_with_weight[option_idx]
            }

            pub(crate) fn iter_weights(&self) -> impl Iterator<Item = (usize, &OptionWeights)> {
                self.opt_with_weight.iter().enumerate()
            }

            pub(crate) fn add_tile_offset(&mut self, option_id: u64, offset: usize) {
                self.option_map.insert(option_id, offset);
                self.option_map_rev.insert(offset as u64, option_id);
            }

            pub(crate) fn get_tile_offset(&self, option_id: u64) -> Option<usize> {
                self.option_map.get(&option_id).copied()
            }

            pub(crate) fn get_tile_type_id(&self, offset: usize) -> Option<u64> {
                self.option_map_rev.get(&(offset as u64)).copied()
            }
        }
    };
}
