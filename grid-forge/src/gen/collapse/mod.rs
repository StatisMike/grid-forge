//! # Collapsible procedural generation
//!
//! Algorithms contained within this module works by having a collection of tiles, which final identity is unknown in the beginning -
//! *every position can be any tile*. In the process of generating a new grid, algorithm will *collapse* possible options for position,
//! choosing randomly one of them. Collapsed tile will then put contraints over the remaining tiles, reducing the number of possible options.
//! Process will then continue until all tiles are collapsed into one possible option.
//!
//! This process is called elsewhere most often *Model Synthesis* or *Wave Function Collapse*, though the distinction between the two is
//! not always clear. Implementation contained there can be tailored to specific needs for the generation process, which makes the line
//! even more blurry, so new name is used here.
//!
//! ## Main distinction
//!
//! Two main types of algorithms are provided in distinct submodules:
//! - [`singular`] - implementation that can be used to generate maps. Each tile type needs to be self-descriptive in
//!   regards to its possible neighbours. Constraints for this type of generation is based strictly on possible neighbours for each given tile,
//!   set of rules which can be described as: tile `X` can be placed in direction `D` of tile `Y`.
//! - [`overlap`] - implementation that can be used to generate maps based on observed patterns. Direct possible adjacents of each tile are
//!   less important there, the pattern context is more important. First step for this type of generation is to create a collection of patterns
//!   from sample gridmaps, which then will be tested for possible neigbours. During the process the possible patterns will then be collapsed
//!   instead of individual tiles.
//!
//! ## Struct types
//!
//! Structs contained in submodules mentioned above are categorized in analogous way, with some additional distinctions described further
//! in documentation for concrete struct.
//!
//! - *adjacency rules* are the rules that are used to determine possible neighbours for each tile.
//! - *frequency hints* can be described as a weights per option - they can be derived from sample gridmaps and are a way to influence the
//!   frequency of options occuring in the generated grid.
//! - *analyzers* provide methods for analyzing sample gridmaps and creating rulesets out of them.
//! - *collapsible grids* are the source of information for the *resolvers*, from the collection of collapsible tiles to the prepared
//!   adjacency rules and frequency hints. They can also contain some pre-collapsed tiles, providing initial constraints for the generation.
//! - *resolvers* are the main executors of the algorithm and are responsible for collapsing the tiles in the *collapsible grids*.
//! - *queues* are used to determine the order in which tiles are collapsed: [`PositionQueue`] takes next position to collapse in a fixed
//!   order, while [`EntrophyQueue`] fetch the next position to collapse with the lowest entrophy.

pub mod error;
pub mod grid;
pub mod option;
// pub mod overlap;
pub mod queue;
pub mod singular;
pub mod tile;

#[cfg(test)]
mod dimension_tests;

pub mod three_d;
pub mod two_d;

use std::{collections::HashSet, marker::PhantomData, ops::Index};

pub mod common {
    pub use super::error::*;
    pub use super::grid::{
        private::CommonCollapsedGrid, private::CommonCollapsibleGrid, CollapsedGrid,
        CollapsibleGrid,
    };
    pub use super::queue::{entrophy, position, propagator};
    pub use super::singular;
    pub use super::tile::*;

    pub use super::private::*;
}

use crate::core::common::*;

#[derive(Clone, Debug, Default)]
pub(crate) struct Adjacencies<D: Dimensionality> {
    inner: Vec<HashSet<u64>>,
    phantom: PhantomData<D>,
}

impl<D: Dimensionality> Adjacencies<D> {
    pub fn new() -> Self {
        let mut inner = Vec::new();

        for _ in 0..D::Dir::N {
            inner.push(HashSet::default());
        }

        Self {
            inner,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn add_at_dir(&mut self, direction: D::Dir, id: u64) {
        let v = self.inner.get_mut(direction.as_idx()).unwrap();
        v.insert(id);
    }
}

impl<D: Dimensionality> Index<D::Dir> for Adjacencies<D> {
    type Output = HashSet<u64>;

    fn index(&self, index: D::Dir) -> &Self::Output {
        &self.inner[index.as_idx()]
    }
}

pub(crate) mod private {
    use std::collections::HashMap;

    use super::common::*;
    use super::option::private::{PerOptionData, WaysToBeOption};
    use super::queue::PositionQueueProcession;
    use super::Adjacencies;
    use crate::core::common::*;

    /// Extension trait for [`Dimensionality`] trait, providing the necessary types for the collapse generative algorithm.
    pub trait CollapseBounds: Dimensionality {
        type WaysToBeOption: WaysToBeOption<Self>;
        type PerOptionData: PerOptionData<Self>;
        type OptionAdjacency: DirectionTable<Self, Vec<usize>> + Default;
        type CollapsedGrid: CollapsedGrid<Self>;
        type PositionQueueProcession: PositionQueueProcession<Self>;
    }

    #[derive(Clone, Debug, Default)]
    pub struct AdjacencyTable<D: Dimensionality> {
        inner: HashMap<u64, Adjacencies<D>>,
    }

    impl<D: Dimensionality> AsRef<HashMap<u64, Adjacencies<D>>> for AdjacencyTable<D> {
        fn as_ref(&self) -> &HashMap<u64, Adjacencies<D>> {
            &self.inner
        }
    }

    impl<D: Dimensionality> AdjacencyTable<D> {
        pub(crate) fn insert_adjacency(&mut self, el_id: u64, direction: D::Dir, adj_id: u64) {
            match self.inner.entry(el_id) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    e.get_mut().add_at_dir(direction, adj_id)
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    let mut adjacencies = Adjacencies::new();
                    adjacencies.add_at_dir(direction, adj_id);
                    e.insert(adjacencies);
                }
            }
        }

        pub(crate) fn get_all_adjacencies_in_direction(
            &self,
            el_id: &u64,
            direction: &D::Dir,
        ) -> impl Iterator<Item = &u64> {
            self.inner
                .get(el_id)
                .expect("cannot get adjacencies for provided `el_id`")[*direction]
                .iter()
        }
    }
}
