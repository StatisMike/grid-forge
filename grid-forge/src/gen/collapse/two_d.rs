use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::core::common::*;
use crate::core::two_d::*;
use crate::id::IdentTileCollection;
use crate::id::IdentifiableTileData;

use super::common::CommonCollapsedGrid;
use super::grid::CollapsedGrid;
use super::grid::CollapsibleGrid;
use super::option::private::PerOptionData;
use super::option::private::WaysToBeOption;
use super::option::OptionWeights;
use super::queue::propagator::PropagateItem;
use super::queue::PositionQueueDirection;
use super::queue::PositionQueueProcession;
use super::queue::PositionQueueStartingPoint;
use super::singular::analyzer::AdjacencyRules;
use super::singular::analyzer::FrequencyHints;
use super::singular::CollapsibleTileGrid;
use super::tile::CollapsedTileData;
use super::tile::CollapsibleTileData;

// ----------------------------- Bounds ----------------------------- //

impl crate::gen::collapse::private::CollapseBounds for TwoDim {
    type WaysToBeOption = WaysToBeOption2D;
    type PerOptionData = PerOptionData2D;
    type OptionAdjacency = DirectionTable2D<Vec<usize>>;
    type CollapsedGrid = CollapsedGrid2D;
    type PositionQueueProcession = PositionQueueProcession2D;
}

// ----------------------------- Options ----------------------------- //

#[derive(Debug, Clone, Default)]
pub struct WaysToBeOption2D {
    table: Vec<DirectionTable2D<usize>>,
}

impl super::option::private::WaysToBeOption<TwoDim> for WaysToBeOption2D {
    type Inner = DirectionTable2D<usize>;

    const EMPTY_DIR_TABLE: DirectionTable2D<usize> = DirectionTable2D::new([0; 4]);

    fn inner(&self) -> &Vec<DirectionTable2D<usize>> {
        &self.table
    }
    fn inner_mut(&mut self) -> &mut Vec<DirectionTable2D<usize>> {
        &mut self.table
    }
}

#[derive(Clone, Debug, Default)]
pub struct PerOptionData2D {
    option_map: HashMap<u64, usize>,
    option_map_rev: HashMap<u64, u64>,
    adjacencies: Vec<DirectionTable2D<Vec<usize>>>,
    ways_to_be_option: WaysToBeOption2D,
    opt_with_weight: Vec<OptionWeights>,
    option_count: usize,
    possible_options_count: usize,
}

impl super::option::private::PerOptionData<TwoDim> for PerOptionData2D {
    fn option_map(&self) -> &HashMap<u64, usize> {
        &self.option_map
    }

    fn option_map_mut(&mut self) -> &mut HashMap<u64, usize> {
        &mut self.option_map
    }

    fn option_map_rev(&self) -> &HashMap<u64, u64> {
        &self.option_map_rev
    }

    fn option_map_rev_mut(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.option_map_rev
    }

    fn adjacencies(&self) -> &Vec<DirectionTable2D<Vec<usize>>> {
        &self.adjacencies
    }

    fn adjacencies_mut(&mut self) -> &mut Vec<DirectionTable2D<Vec<usize>>> {
        &mut self.adjacencies
    }

    fn ways_to_be_option(&self) -> &WaysToBeOption2D {
        &self.ways_to_be_option
    }

    fn ways_to_be_option_mut(&mut self) -> &mut WaysToBeOption2D {
        &mut self.ways_to_be_option
    }

    fn opt_with_weight(&self) -> &Vec<OptionWeights> {
        &self.opt_with_weight
    }

    fn opt_with_weight_mut(&mut self) -> &mut Vec<OptionWeights> {
        &mut self.opt_with_weight
    }

    fn option_count(&self) -> usize {
        self.option_count
    }

    fn possible_options_count(&self) -> usize {
        self.possible_options_count
    }

    fn set_option_count(&mut self, count: usize) {
        self.option_count = count;
    }

    fn possible_options_count_mut(&mut self) -> &mut usize {
        &mut self.possible_options_count
    }

    fn generate_ways_to_be_option(&mut self) {
        for adj in self.adjacencies.iter() {
            let table = Direction2D::all()
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

    fn get_all_enabled_in_direction(&self, option_id: usize, direction: Direction2D) -> &[usize] {
        &self.adjacencies[option_id][direction]
    }
}

impl IdentTileCollection for PerOptionData2D {
    type DATA = usize;

    fn inner(&self) -> &HashMap<u64, Self::DATA> {
        &self.option_map
    }

    fn inner_mut(&mut self) -> &mut HashMap<u64, Self::DATA> {
        &mut self.option_map
    }

    fn rev(&self) -> &HashMap<u64, u64> {
        &self.option_map_rev
    }

    fn rev_mut(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.option_map_rev
    }
}

// ------------------------ Collapsed Grid ------------------------ //

pub struct CollapsedGrid2D {
    grid: GridMap2D<CollapsedTileData>,
    tile_type_ids: HashSet<u64>,
}

impl CollapsedGrid<TwoDim> for CollapsedGrid2D {
    fn new(size: &<TwoDim as Dimensionality>::Size) -> Self {
        Self {
            grid: GridMap2D::new(*size),
            tile_type_ids: HashSet::new(),
        }
    }

    #[allow(refining_impl_trait)]
    fn grid(&self) -> &GridMap2D<CollapsedTileData> {
        &self.grid
    }

    fn tile_type_ids(&self) -> &HashSet<u64> {
        &self.tile_type_ids
    }
}

impl CommonCollapsedGrid<TwoDim> for CollapsedGrid2D {
    #[allow(refining_impl_trait)]
    fn grid_mut(&mut self) -> &mut GridMap2D<CollapsedTileData> {
        &mut self.grid
    }

    fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64> {
        &mut self.tile_type_ids
    }
}

// -------------------------- Singular Grid ------------------------ //

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

// ----------------------------- Queue ----------------------------- //

/// Enum defining the starting point of the collapse wave.
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueStartingPoint2D {
    #[default]
    /// Starts at the `(0, 0)` position.
    UpLeft,
    /// Starts at the `(0, max)` position.
    UpRight,
    /// Starts at the `(max, 0)` position.
    DownLeft,
    /// Starts at the `(max, max)` position.
    DownRight,
}

impl PositionQueueStartingPoint<TwoDim> for PositionQueueStartingPoint2D {}

/// Enum defining the direction in which the tiles will be collapsed.
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueDirection2D {
    #[default]
    /// Collapses tiles in a rowwise fashion.
    Rowwise,
    /// Collapses tiles in a columnwise fashion.
    Columnwise,
}

impl PositionQueueDirection<TwoDim> for PositionQueueDirection2D {}

pub struct PositionQueueProcession2D;

impl PositionQueueProcession<TwoDim> for PositionQueueProcession2D {
    type StartingPoint = PositionQueueStartingPoint2D;
    type Direction = PositionQueueDirection2D;

    fn cmp_fun(
        point: PositionQueueStartingPoint2D,
        direction: PositionQueueDirection2D,
    ) -> fn(&GridPosition2D, &GridPosition2D) -> Ordering {
        match (point, direction) {
            (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| a.x().cmp(&b.x()))
                }
            }
            (PositionQueueStartingPoint2D::UpLeft, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint2D::UpRight, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).then_with(|| b.x().cmp(&a.x()))
                }
            }
            (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).then_with(|| b.y().cmp(&a.y()).reverse())
                }
            }
            (PositionQueueStartingPoint2D::DownLeft, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y().cmp(&b.y()).reverse().then_with(|| b.x().cmp(&a.x()))
                }
            }
            (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Columnwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.x().cmp(&b.x()).reverse().then_with(|| b.y().cmp(&a.y()))
                }
            }
            (PositionQueueStartingPoint2D::DownRight, PositionQueueDirection2D::Rowwise) => {
                |a: &GridPosition2D, b: &GridPosition2D| -> Ordering {
                    a.y()
                        .cmp(&b.y())
                        .reverse()
                        .then_with(|| a.x().cmp(&b.x()).reverse())
                }
            }
        }
    }
}
