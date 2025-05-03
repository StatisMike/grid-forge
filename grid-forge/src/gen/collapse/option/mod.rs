use std::{
    collections::{BTreeMap, HashMap},
    ops::{Add, Sub, SubAssign},
};

use private::WaysToBeOption;

use crate::{core::common::*, id::*, utils::OrderedFloat};

use super::private::AdjacencyTable;

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
    use crate::{
        core::two_d::{DirectionTable2D, TwoDim},
        r#gen::collapse::two_d::TwoDimCollapseBounds,
        two_d::Direction2D,
    };

    #[derive(Debug, Clone, Default)]
    pub struct WaysToBeOption2D {
        table: Vec<DirectionTable2D<usize>>,
    }

    impl WaysToBeOption<TwoDim> for WaysToBeOption2D {
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

    impl private::PerOptionData<TwoDim, TwoDimCollapseBounds> for PerOptionData2D {
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

        fn get_all_enabled_in_direction(
            &self,
            option_id: usize,
            direction: Direction2D,
        ) -> &[usize] {
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
}

pub mod three_d {
    use super::*;
    use crate::{
        core::three_d::{DirectionTable3D, ThreeDim},
        r#gen::collapse::three_d::ThreeDimCollapseBounds,
        three_d::Direction3D,
    };

    #[derive(Clone, Default, Debug)]
    pub struct WaysToBeOption3D {
        table: Vec<DirectionTable3D<usize>>,
    }

    impl WaysToBeOption<ThreeDim> for WaysToBeOption3D {
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

    impl private::PerOptionData<ThreeDim, ThreeDimCollapseBounds> for PerOptionData3D {
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

        fn get_all_enabled_in_direction(
            &self,
            option_id: usize,
            direction: Direction3D,
        ) -> &[usize] {
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
}

pub(crate) mod private {

    use std::fmt::Debug;

    use super::*;
    use crate::{core::direction::private::SealedDir, r#gen::collapse::private::CollapseBounds};

    pub trait WaysToBeOption<D: Dimensionality>: Clone + Debug {
        const EMPTY_DIR_TABLE: Self::Inner;

        type Inner: DirectionTable<D, usize, Output = usize>;

        fn inner(&self) -> &Vec<Self::Inner>;
        fn inner_mut(&mut self) -> &mut Vec<Self::Inner>;

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
            self.inner().iter().enumerate().filter_map(|(idx, t)| {
                if t[D::Dir::FIRST] == 0 {
                    None
                } else {
                    Some(idx)
                }
            })
        }

        fn purge_others(&mut self, options: &[usize]) {
            for (option_id, ways) in self.inner_mut().iter_mut().enumerate() {
                if options.contains(&option_id) {
                    continue;
                }
                *ways = Self::EMPTY_DIR_TABLE;
            }
        }

        fn purge_option(&mut self, option_idx: usize) -> bool {
            if self.inner()[option_idx]
                .inner()
                .as_ref()
                .iter()
                .all(|i| i == &0)
            {
                return false;
            }
            self.inner_mut()[option_idx] = Self::EMPTY_DIR_TABLE;
            true
        }

        fn insert_from_slice(&mut self, slice: &[usize]) {
            self.inner_mut().push(Self::Inner::from_slice(slice));
        }

        fn insert_empty(&mut self) {
            self.inner_mut().push(Self::EMPTY_DIR_TABLE);
        }
    }

    pub trait PerOptionData<D: Dimensionality, CB: CollapseBounds<D> + ?Sized>:
        IdentTileCollection<DATA = usize> + Debug + Default + Clone
    {
        fn option_map(&self) -> &HashMap<u64, usize>;
        fn option_map_mut(&mut self) -> &mut HashMap<u64, usize>;
        fn option_map_rev(&self) -> &HashMap<u64, u64>;
        fn option_map_rev_mut(&mut self) -> &mut HashMap<u64, u64>;
        fn adjacencies(&self) -> &Vec<CB::OptionAdjacency>;
        fn adjacencies_mut(&mut self) -> &mut Vec<CB::OptionAdjacency>;
        fn ways_to_be_option(&self) -> &CB::Ways;
        fn ways_to_be_option_mut(&mut self) -> &mut CB::Ways;
        fn opt_with_weight(&self) -> &Vec<OptionWeights>;
        fn opt_with_weight_mut(&mut self) -> &mut Vec<OptionWeights>;
        fn option_count(&self) -> usize;
        fn set_option_count(&mut self, count: usize);
        fn possible_options_count(&self) -> usize;
        fn possible_options_count_mut(&mut self) -> &mut usize;

        fn populate(
            &mut self,
            options_with_weights: &BTreeMap<u64, u32>,
            adjacencies: &AdjacencyTable<D>,
        ) {
            for (n, (option_id, option_weight)) in options_with_weights.iter().enumerate() {
                self.add_tile_data(*option_id, n);

                self.opt_with_weight_mut()
                    .push(OptionWeights::new(*option_weight));
            }

            self.set_option_count(self.option_map().len());
            *self.possible_options_count_mut() = self.option_count();

            for trans_id in 0..self.option_count() {
                let original_id = self.get_tile_type_id(&trans_id).unwrap();
                let translated_table = self.translate_adjacency_table(original_id, adjacencies);
                self.adjacencies_mut().push(translated_table);
            }

            self.generate_ways_to_be_option();
        }

        fn generate_ways_to_be_option(&mut self);

        fn translate_adjacency_table(
            &self,
            original_id: u64,
            adjacencies: &AdjacencyTable<D>,
        ) -> CB::OptionAdjacency {
            let mut table = CB::OptionAdjacency::default();
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

        fn get_weights(&self, option_idx: usize) -> OptionWeights {
            self.opt_with_weight()[option_idx]
        }

        fn iter_weights(&self) -> impl Iterator<Item = (usize, &OptionWeights)> {
            self.opt_with_weight().iter().enumerate()
        }

        fn get_all_enabled_in_direction(&self, option_id: usize, direction: D::Dir) -> &[usize];
    }
}
