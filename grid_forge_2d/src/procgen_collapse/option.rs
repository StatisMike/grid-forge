use crate::core::{Direction2D, DirectionTable2D};
use grid_forge_core::id::{TypeIdMap, TypeIdSet};
use grid_forge_core::procgen_collapse::option::OptionWeights;
use std::collections::hash_map::Entry;
use std::collections::BTreeMap;

grid_forge_core::__impl_collapse_adjacencies! {
    struct_name: Adjacencies2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    direction_count: 4,
}

grid_forge_core::__impl_adjacency_table! {
    struct_name: AdjacencyTable2D,
    adjacencies: Adjacencies2D,
    direction: Direction2D,
}

grid_forge_core::__impl_ways_to_be_option! {
    struct_name: WaysToBeOption2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    direction_count: 4,
}

grid_forge_core::__impl_per_option_data!{
    struct_name: PerOptionData2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    adjacency_table: AdjacencyTable2D,
    ways_to_be_option: WaysToBeOption2D,
}

// // Recursive expansion of __impl_per_option_data! macro
// // =====================================================

// #[derive(Debug, Clone, Default)]
// pub struct PerOptionData2D {
//     pub(crate) option_map: TypeIdMap<usize>,
//     option_map_rev: TypeIdMap<u64>,
//     adjacencies: Vec<DirectionTable2D<Vec<usize>>>,
//     pub(crate) ways_to_be_option: WaysToBeOption2D,
//     opt_with_weight: Vec<OptionWeights>,
//     pub(crate) option_count: usize,
//     pub(crate) possible_options_count: usize,
// }
// impl PerOptionData2D {
//     pub(crate) fn populate(
//         &mut self,
//         options_with_weights: &BTreeMap<u64, u32>,
//         adjacencies: AdjacencyTable2D,
//     ) {
//         for (n, (option_id, option_weight)) in options_with_weights.iter().enumerate() {
//             self.add_tile_offset(*option_id, n);
//             self.opt_with_weight
//                 .push(OptionWeights::new(*option_weight));
//         }
//         self.option_count = self.option_map.len();
//         self.possible_options_count = self.option_count;
//         for trans_id in 0..self.option_count {
//             let original_id = self.get_tile_type_id(trans_id).unwrap();
//             let translated_table = self.translate_adjacency_table(original_id, &adjacencies);
//             self.adjacencies.push(translated_table);
//         }
//         self.generate_ways_to_be_option();
//     }
//     pub(crate) fn generate_ways_to_be_option(&mut self) {
//         for adj in self.adjacencies.iter() {
//             let table = Direction2D::ALL
//                 .iter()
//                 .map(|dir| adj[*dir].len())
//                 .collect::<Vec<usize>>();
//             if table.contains(&0) {
//                 self.possible_options_count -= 1;
//                 self.ways_to_be_option.insert_empty();
//             } else {
//                 self.ways_to_be_option.insert_from_slice(&table);
//             }
//         }
//     }
//     pub(crate) fn get_all_enabled_in_direction(
//         &self,
//         option_id: usize,
//         direction: Direction2D,
//     ) -> &[usize] {
//         &self.adjacencies[option_id][direction]
//     }
//     pub(crate) fn translate_adjacency_table(
//         &self,
//         original_id: u64,
//         adjacencies: &AdjacencyTable2D,
//     ) -> DirectionTable2D<Vec<usize>> {
//         let mut translated_table = DirectionTable2D::default();
//         if let Some(adj) = adjacencies.inner.get(&original_id) {
//             for dir in Direction2D::ALL.iter() {
//                 translated_table[*dir] = adj[*dir]
//                     .iter()
//                     .map(|&id| self.get_tile_offset(id).expect("cannot get mapped id"))
//                     .collect();
//             }
//         }
//         translated_table
//     }
//     pub(crate) fn get_weights(&self, option_idx: usize) -> OptionWeights {
//         self.opt_with_weight[option_idx]
//     }
//     pub(crate) fn iter_weights(&self) -> impl Iterator<Item = (usize, &OptionWeights)> {
//         self.opt_with_weight.iter().enumerate()
//     }
//     pub(crate) fn add_tile_offset(&mut self, option_id: u64, data: usize) {
//         self.option_map.insert(option_id, data);
//         self.option_map_rev.insert(data as u64, option_id);
//     }
//     pub(crate) fn get_tile_offset(&self, option_id: u64) -> Option<usize> {
//         self.option_map.get(&option_id).copied()
//     }
//     pub(crate) fn get_tile_type_id(&self, offset: usize) -> Option<u64> {
//         self.option_map_rev.get(&(offset as u64)).copied()
//     }
// }
