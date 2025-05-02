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

mod error;
mod grid;
mod option;
// pub mod overlap;
mod queue;
pub mod singular;
mod tile;

use std::{collections::HashSet, fs::File, io::Write, marker::PhantomData, ops::Index};

// Flattened reexports
pub use error::CollapseError;
pub use grid::{CollapsedGrid, CollapsibleGrid};
pub use queue::*;
pub use tile::*;

use crate::core::common::*;

pub mod two_d {
    use crate::core::two_d::*;

    use super::grid::two_d::*;
    pub use crate::gen::collapse::singular::tile::two_d::*;

    pub struct TwoDimCollapseBounds;
    impl crate::gen::collapse::private::CollapseBounds<TwoDim> for TwoDimCollapseBounds {
        type Ways = super::option::two_d::WaysToBeOption2D;
        type PerOption = super::option::two_d::PerOptionData2D;
        type OptionAdjacency = DirectionTable2D<Vec<usize>>;
        type CollapsedGrid = CollapsedGrid2D;
    }
}

pub mod three_d {
    use crate::core::three_d::*;

    use super::grid::three_d::*;

    pub struct ThreeDimCollapseBounds;
    impl crate::gen::collapse::private::CollapseBounds<ThreeDim> for ThreeDimCollapseBounds {
        type Ways = super::option::three_d::WaysToBeOption3D;
        type PerOption = super::option::three_d::PerOptionData3D;
        type OptionAdjacency = DirectionTable3D<Vec<usize>>;
        type CollapsedGrid = CollapsedGrid3D;
    }
}

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

// impl<D: Dimensionality> overlap::Subscriber<D> for DebugSubscriber {
//     fn on_collapse(&mut self, position: &D::Pos, tile_type_id: u64, pattern_id: u64) {
//         if let Some(file) = &mut self.file {
//             writeln!(
//                 file,
//                 "collapsed tile_type_id: {tile_type_id}, pattern_id: {pattern_id} on position: {position:?}"
//             )
//             .unwrap();
//         } else {
//             println!(
//                 "collapsed tile_type_id: {tile_type_id}, pattern_id: {pattern_id} on position: {position:?}"
//             );
//         }
//     }
// }

pub(crate) mod private {
    use std::collections::HashMap;

    use crate::core::common::*;

    use super::{option::private::{PerOptionData, WaysToBeOption}, Adjacencies, CollapsedGrid};

    pub trait CollapseBounds<D: Dimensionality> {
        type Ways: WaysToBeOption<D>;
        type PerOption: PerOptionData<D, Self>;
        type OptionAdjacency: DirectionTable<D, Vec<usize>> + Default;
        type CollapsedGrid: CollapsedGrid<D, Self>;
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
