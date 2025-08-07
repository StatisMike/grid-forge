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

// grid_forge_core::__impl_ways_to_be_option! {
//     struct_name: WaysToBeOption2D,
//     direction: Direction2D,
//     direction_table: DirectionTable2D,
//     direction_count: 4,
// }

// Recursive expansion of __impl_ways_to_be_option! macro
// =======================================================

#[derive(Debug, Clone, Default)]
pub struct WaysToBeOption2D {
    inner: Vec<DirectionTable2D<usize>>,
}
impl WaysToBeOption2D {
    const EMPTY_DIR_TABLE: DirectionTable2D<usize> = DirectionTable2D::new([0; 4]);
    #[doc = r" Decrements number of ways to become option from given direction. If reaches"]
    #[doc = r" 0, returns `true` and given option should be removed."]
    pub(crate) fn decrement(&mut self, option_idx: usize, direction: Direction2D) -> bool {
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
            if t[Direction2D::from_idx(0).unwrap()] == 0 {
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
        let mut inner = [0; 4];
        inner.copy_from_slice(slice);
        self.inner.push(DirectionTable2D::new(inner));
    }
    pub(crate) fn insert_empty(&mut self) {
        self.inner.push(Self::EMPTY_DIR_TABLE);
    }
}

grid_forge_core::__impl_per_option_data!{
    struct_name: PerOptionData2D,
    direction: Direction2D,
    direction_table: DirectionTable2D,
    adjacency_table: AdjacencyTable2D,
    ways_to_be_option: WaysToBeOption2D,
}
