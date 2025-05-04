use super::*;
use crate::{
    core::three_d::{DirectionTable3D, ThreeDim},
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

impl private::PerOptionData<ThreeDim> for PerOptionData3D {
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
