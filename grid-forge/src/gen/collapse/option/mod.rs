use std::{
    collections::{BTreeMap, HashMap},
    ops::{Add, Index, IndexMut, Sub, SubAssign},
};

use private::WaysToBeOption;

use crate::{
    core::common::*,
    id::*,
    utils::OrderedFloat,
};

use super::private::AdjacencyTable;

#[derive(Debug, Clone)]
pub struct PerOptionTable<T> {
    table: Vec<T>,
}

impl<T> Index<usize> for PerOptionTable<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.table.index(index)
    }
}

impl<T> IndexMut<usize> for PerOptionTable<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.table.index_mut(index)
    }
}

impl<T> Default for PerOptionTable<T> {
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}

impl<T> AsRef<Vec<T>> for PerOptionTable<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.table
    }
}

impl<T> AsMut<Vec<T>> for PerOptionTable<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.table
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OptionWeights(pub u32, pub f32);

impl OptionWeights {
    pub fn new(option_weight: u32) -> Self {
        Self(option_weight, Self::calc_weigth(option_weight as f32))
    }

    fn calc_weigth(weight: f32) -> f32 {
        let weight = weight * weight.log2();
        (weight % OrderedFloat::EPSILON) * OrderedFloat::EPSILON
    }

    pub fn round(&mut self) {
        self.1 = (self.1 % OrderedFloat::EPSILON) * OrderedFloat::EPSILON;
    }
}

impl Add for OptionWeights {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for OptionWeights {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for OptionWeights {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.round();
    }
}

pub mod two_d {
    use super::*;
    use crate::core::two_d::{TwoDim, DirectionTable2D};
    pub struct WaysToBeOption2D {
        table: PerOptionTable<DirectionTable2D<usize>>,
    }

    impl WaysToBeOption<TwoDim> for WaysToBeOption2D {
        type Inner = DirectionTable2D<usize>;

        const EMPTY_DIR_TABLE: DirectionTable2D<usize> = DirectionTable2D::new([0; 4]);

        fn inner(&self) -> &PerOptionTable<DirectionTable2D<usize>> {
            &self.table
        }
        fn inner_mut(&mut self) -> &mut PerOptionTable<DirectionTable2D<usize>> {
            &mut self.table
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PerOptionData2D {
        option_map: HashMap<u64, usize>,
        option_map_rev: HashMap<u64, u64>,
        adjacencies: PerOptionTable<DirectionTable2D<Vec<usize>>>,
        ways_to_be_option: WaysToBeOption2D,
        opt_with_weight: PerOptionTable<OptionWeights>,
        option_count: usize,
        possible_options_count: usize,
    }

    impl private::PerOptionData<TwoDim> for PerOptionData2D {
        type OptionAdjacency = DirectionTable2D<Vec<usize>>;
        type Ways = WaysToBeOption2D;

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

        fn adjacencies(&self) -> &PerOptionTable<Self::OptionAdjacency> {
            &self.adjacencies
        }

        fn adjacencies_mut(&mut self) -> &mut PerOptionTable<Self::OptionAdjacency> {
            &mut self.adjacencies
        }

        fn ways_to_be_option(&self) -> &Self::Ways {
            &self.ways_to_be_option
        }

        fn ways_to_be_option_mut(&mut self) -> &mut Self::Ways {
            &mut self.ways_to_be_option
        }

        fn opt_with_weight(&self) -> &PerOptionTable<OptionWeights> {
            &self.opt_with_weight
        }

        fn opt_with_weight_mut(&mut self) -> &mut PerOptionTable<OptionWeights> {
            &mut self.opt_with_weight
        }

        fn option_count(&self) -> usize {
            self.option_count
        }

        fn possible_options_count(&self) -> usize {
            self.possible_options_count
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
}

pub mod three_d {
    use super::*;
    use crate::core::three_d::{ThreeDim, DirectionTable3D};
    pub struct WaysToBeOption3D {
        table: PerOptionTable<DirectionTable3D<usize>>,
    }

    impl WaysToBeOption<ThreeDim> for WaysToBeOption3D {
        type Inner = DirectionTable3D<usize>;

        const EMPTY_DIR_TABLE: DirectionTable3D<usize> = DirectionTable3D::new([0; 6]);

        fn inner(&self) -> &PerOptionTable<DirectionTable3D<usize>> {
            &self.table
        }
        fn inner_mut(&mut self) -> &mut PerOptionTable<DirectionTable3D<usize>> {
            &mut self.table
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PerOptionData3D {
        option_map: HashMap<u64, usize>,
        option_map_rev: HashMap<u64, u64>,
        adjacencies: PerOptionTable<DirectionTable3D<Vec<usize>>>,
        ways_to_be_option: WaysToBeOption3D,
        opt_with_weight: PerOptionTable<OptionWeights>,
        option_count: usize,
        possible_options_count: usize,
    }

    impl private::PerOptionData<ThreeDim> for PerOptionData3D {
        type OptionAdjacency = DirectionTable3D<Vec<usize>>;
        type Ways = WaysToBeOption3D;

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

        fn adjacencies(&self) -> &PerOptionTable<Self::OptionAdjacency> {
            &self.adjacencies
        }

        fn adjacencies_mut(&mut self) -> &mut PerOptionTable<Self::OptionAdjacency> {
            &mut self.adjacencies
        }

        fn ways_to_be_option(&self) -> &Self::Ways {
            &self.ways_to_be_option
        }

        fn ways_to_be_option_mut(&mut self) -> &mut Self::Ways {
            &mut self.ways_to_be_option
        }

        fn opt_with_weight(&self) -> &PerOptionTable<OptionWeights> {
            &self.opt_with_weight
        }

        fn opt_with_weight_mut(&mut self) -> &mut PerOptionTable<OptionWeights> {
            &mut self.opt_with_weight
        }

        fn option_count(&self) -> usize {
            self.option_count
        }

        fn possible_options_count(&self) -> usize {
            self.possible_options_count
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
}

pub (crate) mod private {

    use std::fmt::Debug;

    use super::*;
    use crate::core::direction::private::SealedDir;
    
    pub trait WaysToBeOption<D: Dimensionality> {
        const EMPTY_DIR_TABLE: Self::Inner;

        type Inner: DirectionTable<D, usize>;

        fn inner(&self) -> &PerOptionTable<Self::Inner>;
        fn inner_mut(&mut self) -> &mut PerOptionTable<Self::Inner>;

        /// Decrements number of ways to become option from given direction. If reaches
        /// 0, returns `true` and given option should be removed.
        fn decrement(&mut self, option_idx: usize, direction: D::Dir) -> bool {
            // let num_ways_by_dir = self.table.index_mut(option_idx);
            // let num_ways = num_ways_by_dir[direction];
            if self.inner()[option_idx][direction] == 0 {
                return false;
            }
            self.inner_mut()[option_idx][direction] -= 1;
            if self.inner()[option_idx][direction] > 0 {
                return false;
            }
            self.inner_mut()[option_idx] = Self::EMPTY_DIR_TABLE;
            true
        }

        fn iter_possible(&self) -> impl Iterator<Item = usize> + '_ {
            self.inner()
                .as_ref()
                .iter()
                .enumerate()
                .filter_map(|(idx, t)| if t[D::Dir::FIRST] == 0 { None } else { Some(idx) })
        }

        fn purge_others(&mut self, options: &[usize]) {
            for (option_id, ways) in self.inner_mut().as_mut().iter_mut().enumerate() {
                if options.contains(&option_id) {
                    continue;
                }
                *ways = Self::EMPTY_DIR_TABLE;
            }
        }

        fn purge_option(&mut self, option_idx: usize) -> bool {
            if self.inner()[option_idx].inner().as_ref().iter().all(|i| i == &0) {
                return false;
            }
            self.inner_mut()[option_idx] = Self::EMPTY_DIR_TABLE;
            true
        }
    }

    pub trait PerOptionData<D: Dimensionality>: IdentTileCollection<DATA = usize> + Debug + Default + Clone
    {
        type OptionAdjacency: DirectionTable<D, Vec<usize>>;
        type Ways: WaysToBeOption<D>;

        fn option_map(&self) -> &HashMap<u64, usize>;
        fn option_map_mut(&mut self) -> &mut HashMap<u64, usize>;
        fn option_map_rev(&self) -> &HashMap<u64, u64>;
        fn option_map_rev_mut(&mut self) -> &mut HashMap<u64, u64>;
        fn adjacencies(&self) -> &PerOptionTable<Self::OptionAdjacency>;
        fn adjacencies_mut(&mut self) -> &mut PerOptionTable<Self::OptionAdjacency>;
        fn ways_to_be_option(&self) -> &Self::Ways;
        fn ways_to_be_option_mut(&mut self) -> &mut Self::Ways;
        fn opt_with_weight(&self) -> &PerOptionTable<OptionWeights>;
        fn opt_with_weight_mut(&mut self) -> &mut PerOptionTable<OptionWeights>;
        fn option_count(&self) -> usize;
        fn possible_options_count(&self) -> usize;

        fn populate(
            &mut self,
            options_with_weights: &BTreeMap<u64, u32>,
            adjacencies: &AdjacencyTable<D>,
        ) {
            for (n, (option_id, option_weight)) in options_with_weights.iter().enumerate() {
                self.add_tile_data(*option_id, n);
    
                self.opt_with_weight
                    .as_mut()
                    .push(OptionWeights::new(*option_weight));
            }
    
            self.option_count = self.option_map.len();
            self.possible_options_count = self.option_count;
    
            for trans_id in 0..self.option_count {
                let original_id = self.get_tile_type_id(&trans_id).unwrap();
                self.adjacencies
                    .table
                    .push(self.translate_adjacency_table(original_id, adjacencies));
            }
    
            self.generate_ways_to_be_option();
        }

        fn generate_ways_to_be_option(&mut self) {
            let inner = self.ways_to_be_option_mut().mut_inner().as_mut();
            for adj in self.adjacencies.table.iter() {
                let inner_table = D::Dir::all().iter().map(|dir| adj[*dir].len()).collect::<Vec<usize>>();
                let table = DirectionTable::from_slice(&inner_table);
                if table.inner().contains(&0) {
                    self.possible_options_count -= 1;
                    inner.push(WaysToBeOption::EMPTY_DIR_TABLE)
                } else {
                    inner.push(table);
                }
            }
        }
    
        fn translate_adjacency_table(
            &self,
            original_id: u64,
            adjacencies: &AdjacencyTable<D>,
        ) -> Self::OptionAdjacency {
            let mut table = Self::OptionAdjacency::default();
            for direction in D::Dir::all() {
                table[*direction] = Vec::from_iter(
                    adjacencies
                        .get_all_adjacencies_in_direction(&original_id, direction)
                        .map(|original_id: &u64| {
                            self.get_tile_data(original_id)
                                .expect("cannot get mapped id")
                        })
                        .copied(),
                );
            }
            table
        }
    }
}