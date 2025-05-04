use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use super::*;
use crate::core::two_d::*;
use crate::gen::collapse::option::two_d::*;
use crate::id::IdentifiableTileData;
use crate::r#gen::collapse::common::{CollapsibleGrid, CollapsibleTileData, PropagateItem};
use crate::r#gen::collapse::option::private::PerOptionData;
use crate::r#gen::collapse::option::OptionWeights;

pub use super::analyzer;
pub use super::resolver;
pub use super::subscriber;
pub use super::CollapsibleTileGrid;

#[derive(Clone, Debug)]
pub struct CollapsibleTile2D {
    collapsed_option: Option<usize>,
    num_options: usize,
    ways_to_be_option: WaysToBeOption2D,
    weight: OptionWeights,
    entrophy_noise: f32,
}

impl TileData for CollapsibleTile2D {}

impl CollapsibleTileData<TwoDim> for CollapsibleTile2D {
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

impl crate::gen::collapse::tile::private::CommonCollapsibleTileData<TwoDim> for CollapsibleTile2D {
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

impl<Tile: IdentifiableTileData> crate::gen::collapse::grid::private::CommonCollapsibleGrid<TwoDim>
    for CollapsibleTileGrid2D<Tile>
{
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

    fn _get_initial_propagate_items(
        &self,
        to_collapse: &[GridPosition2D],
    ) -> Vec<PropagateItem<TwoDim>> {
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

impl<Tile: IdentifiableTileData> CollapsibleGrid<TwoDim, Tile> for CollapsibleTileGrid2D<Tile> {}

impl<Tile: IdentifiableTileData> CollapsibleTileGrid<TwoDim, Tile> for CollapsibleTileGrid2D<Tile> {
    fn new_empty(
        size: GridSize2D,
        frequencies: &FrequencyHints<TwoDim, Tile>,
        adjacencies: &AdjacencyRules<TwoDim, Tile>,
    ) -> Self {
        let mut option_data = PerOptionData2D::default();
        option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner());

        Self {
            grid: GridMap2D::new(size),
            option_data,
            tile_type: PhantomData,
        }
    }
}
