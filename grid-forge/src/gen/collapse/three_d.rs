use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::core::common::*;
use crate::core::three_d::*;
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

impl crate::gen::collapse::private::CollapseBounds for ThreeDim {
    type WaysToBeOption = WaysToBeOption3D;
    type PerOptionData = PerOptionData3D;
    type OptionAdjacency = DirectionTable3D<Vec<usize>>;
    type CollapsedGrid = CollapsedGrid3D;
    type PositionQueueProcession = PositionQueueProcession3D;
}

// -------------------- Options -------------------- //

#[derive(Clone, Default, Debug)]
pub struct WaysToBeOption3D {
    table: Vec<DirectionTable3D<usize>>,
}

impl super::option::private::WaysToBeOption<ThreeDim> for WaysToBeOption3D {
    type Inner = DirectionTable3D<usize>;

    const EMPTY_DIR_TABLE: DirectionTable3D<usize> = DirectionTable3D::new([0; 6]);

    fn inner(&self) -> &Vec<DirectionTable3D<usize>> {
        &self.table
    }
    fn inner_mut(&mut self) -> &mut Vec<DirectionTable3D<usize>> {
        &mut self.table
    }
}

#[derive(Clone, Debug, Default)]
pub struct PerOptionData3D {
    option_map: HashMap<u64, usize>,
    option_map_rev: HashMap<u64, u64>,
    adjacencies: Vec<DirectionTable3D<Vec<usize>>>,
    ways_to_be_option: WaysToBeOption3D,
    opt_with_weight: Vec<OptionWeights>,
    option_count: usize,
    possible_options_count: usize,
}

impl super::option::private::PerOptionData<ThreeDim> for PerOptionData3D {
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

    fn adjacencies(&self) -> &Vec<DirectionTable3D<Vec<usize>>> {
        &self.adjacencies
    }

    fn adjacencies_mut(&mut self) -> &mut Vec<DirectionTable3D<Vec<usize>>> {
        &mut self.adjacencies
    }

    fn ways_to_be_option(&self) -> &WaysToBeOption3D {
        &self.ways_to_be_option
    }

    fn ways_to_be_option_mut(&mut self) -> &mut WaysToBeOption3D {
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

    fn set_option_count(&mut self, count: usize) {
        self.option_count = count;
    }

    fn possible_options_count(&self) -> usize {
        self.possible_options_count
    }

    fn possible_options_count_mut(&mut self) -> &mut usize {
        &mut self.possible_options_count
    }

    fn generate_ways_to_be_option(&mut self) {
        for adj in self.adjacencies.iter() {
            let table = Direction3D::all()
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

    fn get_all_enabled_in_direction(&self, option_id: usize, direction: Direction3D) -> &[usize] {
        &self.adjacencies[option_id][direction]
    }
}

impl IdentTileCollection for PerOptionData3D {
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

// -------------------- Grid -------------------- //

pub struct CollapsedGrid3D {
    grid: GridMap3D<CollapsedTileData>,
    tile_type_ids: HashSet<u64>,
}

impl CollapsedGrid<ThreeDim> for CollapsedGrid3D {
    fn new(size: &GridSize3D) -> Self {
        Self {
            grid: GridMap3D::new(*size),
            tile_type_ids: HashSet::new(),
        }
    }

    #[allow(refining_impl_trait)]
    fn grid(&self) -> &GridMap3D<CollapsedTileData> {
        &self.grid
    }

    fn tile_type_ids(&self) -> &HashSet<u64> {
        &self.tile_type_ids
    }
}

impl CommonCollapsedGrid<ThreeDim> for CollapsedGrid3D {
    #[allow(refining_impl_trait)]
    fn grid_mut(&mut self) -> &mut GridMap3D<CollapsedTileData> {
        &mut self.grid
    }

    fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64> {
        &mut self.tile_type_ids
    }
}

// -------------------------- Singular Grid ------------------------ //

#[derive(Clone, Debug)]
pub struct CollapsibleTile3D {
    collapsed_option: Option<usize>,
    num_options: usize,
    ways_to_be_option: WaysToBeOption3D,
    weight: OptionWeights,
    entrophy_noise: f32,
}

impl TileData for CollapsibleTile3D {}

impl CollapsibleTileData<ThreeDim> for CollapsibleTile3D {
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

impl crate::gen::collapse::tile::private::CommonCollapsibleTileData<ThreeDim>
    for CollapsibleTile3D
{
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

impl<Tile: IdentifiableTileData>
    crate::gen::collapse::grid::private::CommonCollapsibleGrid<ThreeDim>
    for CollapsibleTileGrid3D<Tile>
{
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

    fn _get_initial_propagate_items(
        &self,
        to_collapse: &[GridPosition3D],
    ) -> Vec<PropagateItem<ThreeDim>> {
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

impl<Tile: IdentifiableTileData> CollapsibleGrid<ThreeDim, Tile> for CollapsibleTileGrid3D<Tile> {}

impl<Tile: IdentifiableTileData> CollapsibleTileGrid<ThreeDim, Tile>
    for CollapsibleTileGrid3D<Tile>
{
    fn new_empty(
        size: GridSize3D,
        frequencies: &FrequencyHints<ThreeDim, Tile>,
        adjacencies: &AdjacencyRules<ThreeDim, Tile>,
    ) -> Self {
        let mut option_data = PerOptionData3D::default();
        option_data.populate(&frequencies.get_all_weights_cloned(), adjacencies.inner());

        Self {
            grid: GridMap3D::new(size),
            option_data,
            tile_type: PhantomData,
        }
    }
}

// -------------------- Queue -------------------- //

/// Enum defining the starting point of the collapse wave in 3D space
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueStartingPoint3D {
    #[default]
    /// Starts at the `(0, 0, 0)` position
    UpLeftFront,
    /// Starts at the `(0, max, 0)` position
    UpRightFront,
    /// Starts at the `(max, 0, 0)` position
    DownLeftFront,
    /// Starts at the `(max, max, 0)` position
    DownRightFront,
    /// Starts at the `(0, 0, max)` position
    UpLeftBack,
    /// Starts at the `(0, max, max)` position
    UpRightBack,
    /// Starts at the `(max, 0, max)` position
    DownLeftBack,
    /// Starts at the `(max, max, max)` position
    DownRightBack,
}

impl PositionQueueStartingPoint<ThreeDim> for PositionQueueStartingPoint3D {}

/// Enum defining the direction in which the tiles will be collapsed in 3D
#[derive(Default, Eq, PartialEq)]
pub enum PositionQueueDirection3D {
    #[default]
    /// Collapses tiles in a rowwise fashion (X-axis primary)
    Rowwise,
    /// Collapses tiles in a columnwise fashion (Y-axis primary)
    Columnwise,
    /// Collapses tiles in a heightwise fashion (Z-axis primary)
    Heightwise,
}

impl PositionQueueDirection<ThreeDim> for PositionQueueDirection3D {}

pub struct PositionQueueProcession3D;

impl PositionQueueProcession<ThreeDim> for PositionQueueProcession3D {
    type StartingPoint = PositionQueueStartingPoint3D;
    type Direction = PositionQueueDirection3D;

    fn cmp_fun(
        point: PositionQueueStartingPoint3D,
        direction: PositionQueueDirection3D,
    ) -> fn(&GridPosition3D, &GridPosition3D) -> Ordering {
        match (point, direction) {
            // Front layer comparisons
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Rowwise) => {
                |a, b| {
                    a.z()
                        .cmp(&b.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
            }
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Columnwise) => {
                |a, b| {
                    a.z()
                        .cmp(&b.z())
                        .then_with(|| a.x().cmp(&b.x()))
                        .then_with(|| a.y().cmp(&b.y()))
                }
            }
            (PositionQueueStartingPoint3D::UpLeftFront, PositionQueueDirection3D::Heightwise) => {
                |a, b| {
                    a.x()
                        .cmp(&b.x())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.z().cmp(&b.z()))
                }
            }

            // Back layer comparisons
            (PositionQueueStartingPoint3D::UpLeftBack, PositionQueueDirection3D::Rowwise) => {
                |a, b| {
                    b.z()
                        .cmp(&a.z())
                        .then_with(|| a.y().cmp(&b.y()))
                        .then_with(|| a.x().cmp(&b.x()))
                }
            }

            // Add other combinations following this pattern...
            // This would need 8 starting points × 3 directions = 24 match arms

            // Default case
            _ => |a, b| {
                a.z()
                    .cmp(&b.z())
                    .then_with(|| a.y().cmp(&b.y()))
                    .then_with(|| a.x().cmp(&b.x()))
            },
        }
    }
}
