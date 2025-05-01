use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;


use crate::core::common::*;
use crate::r#gen::collapse::option::private::PerOptionData;
use crate::r#gen::collapse::private::CollapseBounds;
use crate::id::*;

use crate::gen::collapse::error::CollapsibleGridError;
use crate::gen::collapse::grid::CollapsibleGrid;
use crate::gen::collapse::option::OptionWeights;
use crate::gen::collapse::{tile::*, CollapsedGrid, PropagateItem};

use super::{AdjacencyRules, FrequencyHints};

// // /// Tile with options that can be collapsed into one of them. Mostly used within the [`CollapsibleTileGrid`].
// // #[derive(Clone, Debug)]
// // pub struct CollapsibleTile<D: Dimensionality> {
// //     collapsed_option: Option<usize>,
// //     num_possible_options: usize,
// //     ways_to_be_option: WaysToBeOption,
// //     weight: OptionWeights,
// //     entrophy_noise: f32,
// // }

// // impl TileData for CollapsibleTile {}

// // impl <D: Dimensionality> crate::gen::collapse::tile::private::Sealed<D> for CollapsibleTile<D> {
// //     type Ways = WaysToBeOption<D>;
// //     type PerOption = PerOptionData<D>;
// //     type Grid = GridMap<D, Self>;


// //     fn remove_option(&mut self, weights: OptionWeights) {
// //         self.num_possible_options -= 1;
// //         self.weight -= weights;
// //     }

// //     fn new_uncollapsed_tile(
// //         position: GridPosition,
// //         num_possible_options: usize,
// //         ways_to_be_option: WaysToBeOption,
// //         weight: OptionWeights,
// //         entrophy_noise: f32,
// //     ) -> GridTile<Self>
// //     where
// //         Self: TileData,
// //     {
// //         GridTile::new(
// //             position,
// //             Self {
// //                 collapsed_option: None,
// //                 num_possible_options,
// //                 ways_to_be_option,
// //                 weight,
// //                 entrophy_noise,
// //             },
// //         )
// //     }

// //     fn ways_to_be_option(&self) -> &WaysToBeOption {
// //         &self.ways_to_be_option
// //     }

// //     fn mut_ways_to_be_option(&mut self) -> &mut WaysToBeOption {
// //         &mut self.ways_to_be_option
// //     }

// //     fn num_possible_options(&self) -> usize {
// //         self.num_possible_options
// //     }

// //     fn collapse<R: Rng>(
// //         &mut self,
// //         rng: &mut R,
// //         options_data: &PerOptionData,
// //     ) -> Option<Vec<usize>> {
// //         assert!(self.weight.0 > 0);
// //         let random = rng.gen_range(0..self.weight.0);
// //         let mut current_sum = 0;
// //         let mut chosen = None;
// //         let mut out = Vec::new();
// //         for option_idx in self.ways_to_be_option().iter_possible() {
// //             current_sum += options_data.get_weights(option_idx).0;
// //             if chosen.is_some() || random > current_sum {
// //                 out.push(option_idx);
// //                 continue;
// //             }
// //             chosen = Some(option_idx);
// //         }
// //         assert!(chosen.is_some(), "option should always be chosen!");
// //         self.collapsed_option = chosen;
// //         self.num_possible_options = 0;
// //         self.weight = OptionWeights::default();
// //         Some(out)
// //     }

// //     fn mark_collapsed(&mut self, collapsed_idx: usize) {
// //         self.collapsed_option = Some(collapsed_idx);
// //         self.num_possible_options = 0;
// //         self.weight = OptionWeights::default();
// //     }

// //     fn weight_sum(&self) -> u32 {
// //         self.weight.0
// //     }
// // }

// // impl CollapsibleTileData for CollapsibleTile {
// //     fn collapse_idx(&self) -> Option<usize> {
// //         self.collapsed_option
// //     }

// //     fn calc_entrophy(&self) -> f32 {
// //         Self::calc_entrophy_ext(self.weight.0, self.weight.1) + self.entrophy_noise
// //     }

// //     fn num_compatible_options(&self) -> usize {
// //         self.num_possible_options
// //     }

// //     fn new_collapsed_data(option_idx: usize) -> Self {
// //         Self {
// //             collapsed_option: Some(option_idx),
// //             num_possible_options: 0,
// //             ways_to_be_option: WaysToBeOption::default(),
// //             weight: OptionWeights::default(),
// //             entrophy_noise: 0.,
// //         }
// //     }
// // }

// /// Collapsible grid compatible with [`singular::Resolver`](super::Resolver).
// ///
// /// It stores the data of the tiles in the internal grid of [`CollapsibleTile`]. Holds information about the rules for
// /// the generation of the tiles and the weights of the options, provide options to populate the grid with some collapsed
// /// tiles before the generation process and retrieve the collapsed tiles after the generation ends.
// ///
// /// ## Example
// /// ```
// /// use grid_forge::gen::collapse::*;
// /// use grid_forge::{GridPosition, GridSize};
// /// use grid_forge::identifiable::*;
// /// use grid_forge::identifiable::builders::*;
// /// #
// /// # // setup to prepopulate the rules.
// /// # use grid_forge::*;
// /// # use grid_forge::identifiable::*;
// /// # use grid_forge::identifiable::builders::*;
// /// # let first_tile = GridTile::new(GridPosition::new_xy(0,0), BasicIdentTileData::tile_new(0));
// /// # let second_tile = GridTile::new(GridPosition::new_xy(1,0), BasicIdentTileData::tile_new(1));
// /// # let mut adjacency_rules = singular::AdjacencyRules::<BasicIdentTileData>::default();
// /// # adjacency_rules.add_adjacency(&first_tile, &second_tile, GridDir::UP);
// /// # adjacency_rules.add_adjacency(&second_tile, &first_tile, GridDir::LEFT);
// /// # let mut frequency_hints = singular::FrequencyHints::<BasicIdentTileData>::default();
// /// # frequency_hints.set_weight_for_tile(&first_tile, 1);
// /// # frequency_hints.set_weight_for_tile(&second_tile, 2);
// ///
// /// // Create new empty grid.
// /// let mut collapsible_grid = singular::CollapsibleTileGrid::new_empty(GridSize::new_xy(10, 10), &frequency_hints, &adjacency_rules);
// ///
// /// // We can prepopulate existing collapsible grid with some collapsed tiles using `CollapsedGrid`.
// /// let mut collapsed_grid = CollapsedGrid::new(GridSize::new_xy(10, 10));
// /// collapsed_grid.insert_data(&GridPosition::new_xy(0, 0), CollapsedTileData::new(0));
// /// collapsed_grid.insert_data(&GridPosition::new_xy(1, 0), CollapsedTileData::new(1));
// ///
// /// collapsible_grid.populate_from_collapsed(&collapsed_grid).unwrap();
// ///
// /// // The collapsible grid can be created directly from a `CollapsedGrid`.
// /// let mut collapsible_grid = singular::CollapsibleTileGrid::new_from_collapsed(&collapsed_grid, &frequency_hints, &adjacency_rules).unwrap();
// ///
// /// // If there is a need to change the rules after the grid is created, it can be done using `change` method.
// /// # let mut new_frequency_hints = singular::FrequencyHints::<BasicIdentTileData>::default();
// /// # new_frequency_hints.set_weight_for_tile(&first_tile, 2);
// /// # new_frequency_hints.set_weight_for_tile(&second_tile, 1);
// /// collapsible_grid = collapsible_grid.change(&new_frequency_hints, &adjacency_rules).unwrap();
// ///
// /// // The grid can be retrieved as a `CollapsedGrid`.
// /// let collapsed_grid = collapsible_grid.retrieve_collapsed();
// ///
// /// // The grid can be retrieved as a `GridMap2D` using compatible `IdentTileBuilder`.
// /// let ident_grid = collapsible_grid.retrieve_ident(&IdentTileTraitBuilder::<BasicIdentTileData>::default());
// /// ```

// pub trait CollapsibleTileGrid<
//     D: Dimensionality, 
//     CB: CollapseBounds<D>, 
//     Tile: IdentifiableTileData
//     >: CollapsibleGrid<D, CB> + Sized {
    
//     fn new_empty(
//         size: D::Size,
//         frequencies: &FrequencyHints<D, Tile>,
//         adjacencies: &AdjacencyRules<D, Tile>,
//     ) -> Self;

//     fn new_from_collapsed(
//         collapsed: &CB::CollapsedGrid,
//         frequencies: &FrequencyHints<D, Tile>,
//         adjacencies: &AdjacencyRules<D, Tile>,
//     ) -> Result<Self, CollapsibleGridError<D>>;

//     fn change(
//         self,
//         frequencies: &FrequencyHints<D, Tile>,
//         adjacencies: &AdjacencyRules<D, Tile>,
//     ) -> Result<Self, CollapsibleGridError<D>>;

//     fn retrieve_collapsed(&self) -> impl CollapsedGrid<D, CB>;

//     fn retrieve_ident<OT: IdentifiableTileData, B: IdentTileBuilder<OT>>(
//         &self,
//         builder: &B,
//     ) -> Result<impl GridMap<D, OT>, CollapsibleGridError<D>>;

//     fn retrieve_ident_default<OutputTile: IdDefault>(
//         &self,
//     ) -> impl GridMap<D, OutputTile>;

// }

pub (crate) mod two_d {
    use super::*;
    use crate::gen::collapse::two_d::TwoDimCollapseBounds;
    use crate::core::two_d::*;
    use crate::gen::collapse::option::two_d::*;

    #[derive(Clone, Debug)]
    pub struct CollapsibleTile2D {
        collapsed_option: Option<usize>,
        num_options: usize,
        ways_to_be_option: WaysToBeOption2D,
        weight: OptionWeights,
        entrophy_noise: f32,
    }

    impl TileData for CollapsibleTile2D {}

    impl CollapsibleTileData<TwoDim, TwoDimCollapseBounds> for CollapsibleTile2D {
        fn collapse_idx(&self) -> Option<usize> {
            self.collapsed_option
        }

        fn calc_entrophy(&self) -> f32 {
            Self::calc_entrophy_ext(self.weight.0, self.weight.1) + self.entrophy_noise
        }

        fn num_compatible_options(&self) -> usize {
            self.num_options
        }
        
        fn new_collapsed_data(option_idx: usize) -> Self {
            Self {
                collapsed_option: Some(option_idx),
                num_options: 0,
                ways_to_be_option: WaysToBeOption2D::default(),
                weight: OptionWeights::default(),
                entrophy_noise: 0.,
            }
        }
    }

    impl crate::gen::collapse::tile::private::CommonCollapsibleTileData<TwoDim, TwoDimCollapseBounds> for CollapsibleTile2D {
    
        fn new_uncollapsed_tile(
            num_options: usize,
            ways_to_be_option: WaysToBeOption2D,
            weight: OptionWeights,
            entrophy_noise: f32,
        ) -> Self {
            Self {
                collapsed_option: None,
                num_options,
                ways_to_be_option,
                weight,
                entrophy_noise,
            }
        }
    
        fn num_possible_options(&self) -> usize {
            self.num_options
        }
    
        fn ways_to_be_option(&self) -> &WaysToBeOption2D {
            &self.ways_to_be_option
        }
    
        fn mut_ways_to_be_option(&mut self) -> &mut WaysToBeOption2D {
            &mut self.ways_to_be_option
        }
    
        fn remove_option(&mut self, weights: OptionWeights) {
            self.num_options -= 1;
            self.weight -= weights;
        }
    
        fn mark_collapsed(&mut self, collapsed_idx: usize) {
            self.collapsed_option = Some(collapsed_idx);
            self.num_options = 0;
            self.weight = OptionWeights::default();
        }
    
        fn weight_sum(&self) -> u32 {
            self.weight.0
        }
    }

    pub struct CollapsibleTileGrid2D<Tile: IdentifiableTileData> {
        grid: GridMap2D<CollapsibleTile2D>,
        option_data: PerOptionData2D,
        tile_type: PhantomData<Tile>,
    }

    impl <Tile: IdentifiableTileData> crate::gen::collapse::grid::private::CommonCollapsibleGrid<TwoDim, TwoDimCollapseBounds> for CollapsibleTileGrid2D<Tile> {
        type CollapsibleData = CollapsibleTile2D;
        type CollapsibleGrid = GridMap2D<CollapsibleTile2D>;
    
        fn _grid(&self) -> &GridMap2D<CollapsibleTile2D> {
            &self.grid
        }
    
        fn _grid_mut(&mut self) -> &mut GridMap2D<CollapsibleTile2D> {
            &mut self.grid
        }
    
        fn _option_data(&self) -> &PerOptionData2D {
            &self.option_data
        }
    
        fn _get_initial_propagate_items(&self, to_collapse: &[GridPosition2D]) -> Vec<PropagateItem<TwoDim>> {
            let mut out = Vec::new();
            let mut cache = HashMap::new();
            let mut check_generated = HashSet::new();
            let check_provided: HashSet<_> = HashSet::from_iter(to_collapse.iter());

            for pos_to_collapse in to_collapse {
                for (pos, neighbour_tile) in self.grid.get_neighbours(pos_to_collapse) {
                    if !neighbour_tile.is_collapsed()
                        || check_provided.contains(&pos)
                        || check_generated.contains(&pos)
                    {
                        continue;
                    }
                    check_generated.insert(pos);
                    let collapsed_idx = neighbour_tile.collapse_idx().unwrap();
                    for opt_to_remove in cache.entry(collapsed_idx).or_insert_with(|| {
                        (0..self.option_data.option_count())
                            .filter(|option_idx| option_idx != &collapsed_idx)
                            .collect::<Vec<usize>>()
                    }) {
                        out.push(PropagateItem::new(pos, *opt_to_remove))
                    }
                }
            }
            out
        }
    }

    impl <Tile: IdentifiableTileData> CollapsibleGrid<TwoDim, TwoDimCollapseBounds, Tile> for CollapsibleTileGrid2D<Tile> {}
}

pub (crate) mod three_d {
    use super::*;
    use crate::gen::collapse::three_d::ThreeDimCollapseBounds;
    use crate::core::three_d::*;
    use crate::gen::collapse::option::three_d::*;

    #[derive(Clone, Debug)]
    pub struct CollapsibleTile3D {
        collapsed_option: Option<usize>,
        num_options: usize,
        ways_to_be_option: WaysToBeOption3D,
        weight: OptionWeights,
        entrophy_noise: f32,
    }

    impl TileData for CollapsibleTile3D {}

    impl CollapsibleTileData<ThreeDim, ThreeDimCollapseBounds> for CollapsibleTile3D {
        fn collapse_idx(&self) -> Option<usize> {
            self.collapsed_option
        }

        fn calc_entrophy(&self) -> f32 {
            Self::calc_entrophy_ext(self.weight.0, self.weight.1) + self.entrophy_noise
        }

        fn num_compatible_options(&self) -> usize {
            self.num_options
        }
        
        fn new_collapsed_data(option_idx: usize) -> Self {
            Self {
                collapsed_option: Some(option_idx),
                num_options: 0,
                ways_to_be_option: WaysToBeOption3D::default(),
                weight: OptionWeights::default(),
                entrophy_noise: 0.,
            }
        }
    }

    impl crate::gen::collapse::tile::private::CommonCollapsibleTileData<ThreeDim, ThreeDimCollapseBounds> for CollapsibleTile3D {
    
        fn new_uncollapsed_tile(
            num_options: usize,
            ways_to_be_option: WaysToBeOption3D,
            weight: OptionWeights,
            entrophy_noise: f32,
        ) -> Self {
            Self {
                collapsed_option: None,
                num_options,
                ways_to_be_option,
                weight,
                entrophy_noise,
            }
        }
    
        fn num_possible_options(&self) -> usize {
            self.num_options
        }
    
        fn ways_to_be_option(&self) -> &WaysToBeOption3D {
            &self.ways_to_be_option
        }
    
        fn mut_ways_to_be_option(&mut self) -> &mut WaysToBeOption3D {
            &mut self.ways_to_be_option
        }
    
        fn remove_option(&mut self, weights: OptionWeights) {
            self.num_options -= 1;
            self.weight -= weights;
        }
    
        fn mark_collapsed(&mut self, collapsed_idx: usize) {
            self.collapsed_option = Some(collapsed_idx);
            self.num_options = 0;
            self.weight = OptionWeights::default();
        }
    
        fn weight_sum(&self) -> u32 {
            self.weight.0
        }
    }

    pub struct CollapsibleTileGrid3D<Tile: IdentifiableTileData> {
        grid: GridMap3D<CollapsibleTile3D>,
        option_data: PerOptionData3D,
        tile_type: PhantomData<Tile>,
    }

    impl <Tile: IdentifiableTileData> crate::gen::collapse::grid::private::CommonCollapsibleGrid<ThreeDim, ThreeDimCollapseBounds> for CollapsibleTileGrid3D<Tile> {
        type CollapsibleData = CollapsibleTile3D;
        type CollapsibleGrid = GridMap3D<CollapsibleTile3D>;
    
        fn _grid(&self) -> &GridMap3D<CollapsibleTile3D> {
            &self.grid
        }
    
        fn _grid_mut(&mut self) -> &mut GridMap3D<CollapsibleTile3D> {
            &mut self.grid
        }
    
        fn _option_data(&self) -> &PerOptionData3D {
            &self.option_data
        }
    
        fn _get_initial_propagate_items(&self, to_collapse: &[GridPosition3D]) -> Vec<PropagateItem<ThreeDim>> {
            let mut out = Vec::new();
            let mut cache = HashMap::new();
            let mut check_generated = HashSet::new();
            let check_provided: HashSet<_> = HashSet::from_iter(to_collapse.iter());

            for pos_to_collapse in to_collapse {
                for (pos, neighbour_tile) in self.grid.get_neighbours(pos_to_collapse) {
                    if !neighbour_tile.is_collapsed()
                        || check_provided.contains(&pos)
                        || check_generated.contains(&pos)
                    {
                        continue;
                    }
                    check_generated.insert(pos);
                    let collapsed_idx = neighbour_tile.collapse_idx().unwrap();
                    for opt_to_remove in cache.entry(collapsed_idx).or_insert_with(|| {
                        (0..self.option_data.option_count())
                            .filter(|option_idx| option_idx != &collapsed_idx)
                            .collect::<Vec<usize>>()
                    }) {
                        out.push(PropagateItem::new(pos, *opt_to_remove))
                    }
                }
            }
            out
        }
    }

    impl <Tile: IdentifiableTileData> CollapsibleGrid<ThreeDim, ThreeDimCollapseBounds, Tile> for CollapsibleTileGrid3D<Tile> {}
}