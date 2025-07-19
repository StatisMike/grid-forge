use crate::{id::{IdDefault, TypedData}, TileData};

#[derive(Clone, Copy, Debug)]
pub struct CollapsedTileData {
    tile_type_id: u64,
}

impl TileData for CollapsedTileData {}

impl TypedData for CollapsedTileData {
    fn tile_type_id(&self) -> u64 {
        self.tile_type_id
    }
}

impl IdDefault for CollapsedTileData {
    fn tile_type_default(tile_type_id: u64) -> Self {
        Self::new(tile_type_id)
    }
}

impl CollapsedTileData {
    #[inline]
    pub fn new(tile_type_id: u64) -> Self {
        Self { tile_type_id }
    }
}

// #[macro_export]
// macro_rules! __impl_common_collapsible_tile_trait {
//     (
//         trait_name: $name:ident,
//         position: $position:ident,
//         ways_to_be_option: $ways_to_be_option:ident,
//         per_option_data: $per_option_data:ident,
//     ) => {
//         use rand::distributions::Distribution as _;

//         pub trait $name: Sized + private::Sealed  {
//             fn new_uncollapsed_tile(
//                 num_options: usize,
//                 ways_to_be_option: $ways_to_be_option,
//                 weight: OptionWeights,
//                 entrophy_noise: f32,
//             ) -> Self;

//             fn new_from_frequency_with_entrophy<R: Rng>(
//                 rng: &mut R,
//                 positions: &[$position],
//                 options_data: &$per_option_data,
//             ) -> Vec<($position, Self)> {
//                 let rng_range = Self::entrophy_uniform();

//                 let weight = options_data.ways_to_be_option
//                     .iter_possible()
//                     .map(|option_idx| options_data.get_weights(option_idx))
//                     .fold(OptionWeights::default(), |sum, new| sum + new);

//                 positions
//                     .iter()
//                     .map(|pos| {
//                         (
//                             *pos,
//                             Self::new_uncollapsed_tile(
//                                 options_data.possible_options_count,
//                                 options_data.ways_to_be_option.clone(),
//                                 weight,
//                                 rng_range.sample(rng),
//                             ),
//                         )
//                     })
//                     .collect::<Vec<_>>()
//             }

//             fn new_from_frequency(
//                 positions: &[$position],
//                 options_data: &$per_option_data,
//             ) -> Vec<($position, Self)> {
//                 let weight = options_data.ways_to_be_option
//                     .iter_possible()
//                     .map(|option_idx| options_data.get_weights(option_idx))
//                     .fold(OptionWeights::default(), |sum, new| sum + new);

//                 positions
//                     .iter()
//                     .map(|pos| {
//                         (
//                             *pos,
//                             Self::new_uncollapsed_tile(
//                                 options_data.possible_options_count,
//                                 options_data.ways_to_be_option.clone(),
//                                 weight,
//                                 0.0,
//                             ),
//                         )
//                     })
//                 .collect::<Vec<_>>()
//             }

//             fn num_possible_options(&self) -> usize;

//             fn ways_to_be_option(&self) -> &$ways_to_be_option;

//             fn mut_ways_to_be_option(&mut self) -> &mut $ways_to_be_option;

//             fn remove_option(&mut self, weights: OptionWeights);

//             /// Range of uniformly distributed data for entrophy noise.
//             fn entrophy_uniform() -> Uniform<f32> {
//                 Uniform::<f32>::new(0., 0.00001)
//             }

//             fn mark_collapsed(&mut self, collapsed_idx: usize);

//             fn weight_sum(&self) -> u32;

//             fn is_collapsed(&self) -> bool {
//                 self.collapsed_idx().is_some()
//             }

//             fn collapsed_idx(&self) -> Option<usize>;

//             /// Collapses tile into one of possible options, returning the vector of the removed options.
//             fn collapse_gather_removed<R: Rng>(
//                 &mut self,
//                 rng: &mut R,
//                 options_data: &$per_option_data,
//             ) -> Vec<usize> {
//                 assert!(
//                     self.weight_sum() > 0,
//                     "weight sum should be positive when collapsing!"
//                 );
//                 let random = rng.gen_range(0..self.weight_sum());
//                 let mut current_sum = 0;
//                 let mut chosen = None;
//                 let mut out = Vec::new();
//                 for option_idx in self.ways_to_be_option().iter_possible() {
//                     current_sum += options_data.get_weights(option_idx).0;
//                     if chosen.is_some() || random > current_sum {
//                         out.push(option_idx);
//                         continue;
//                     }
//                     chosen = Some(option_idx);
//                 }
//                 self.mark_collapsed(chosen.expect("options should always be chosen"));
//                 out
//             }

//             /// Collapses tiles into one of possible options.
//             fn collapse_basic<R: Rng>(&mut self, rng: &mut R, options_data: &$per_option_data) {
//                 assert!(
//                     self.weight_sum() > 0,
//                     "weight sum should be positive when collapsing!"
//                 );
//                 let random = rng.gen_range(0..self.weight_sum());
//                 let mut current_sum = 0;
//                 let mut chosen = None;
//                 for option_idx in self.ways_to_be_option().iter_possible() {
//                     current_sum += options_data.get_weights(option_idx).0;
//                     if chosen.is_some() || random > current_sum {
//                         continue;
//                     }
//                     chosen = Some(option_idx);
//                 }
//                 self.mark_collapsed(chosen.expect("options should always be chosen"));
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! __impl_collapsible_tile_data {
//     (
//         struct_name: $name:ident,
//         trait_name: $trait_name:ident,
//         ways_to_be_option: $ways_to_be_option:ident,
//     ) => {

//         #[derive(Clone, Debug)]
//         pub struct $name {
//             collapsed_option: Option<usize>,
//             num_options: usize,
//             ways_to_be_option: $ways_to_be_option,
//             weight: OptionWeights,
//             entrophy_noise: f32,
//         }

//         impl TileData for $name {}

//         impl private::Sealed for $name {}

//         impl $trait_name for $name {
//             fn new_uncollapsed_tile(
//                 num_options: usize,
//                 ways_to_be_option: $ways_to_be_option,
//                 weight: OptionWeights,
//                 entrophy_noise: f32,
//             ) -> Self {
//                 Self {
//                     collapsed_option: None,
//                     num_options,
//                     ways_to_be_option,
//                     weight,
//                     entrophy_noise,
//                 }
//             }

//             fn num_possible_options(&self) -> usize {
//                 self.num_options
//             }

//             fn ways_to_be_option(&self) -> &$ways_to_be_option {
//                 &self.ways_to_be_option
//             }

//             fn mut_ways_to_be_option(&mut self) -> &mut $ways_to_be_option {
//                 &mut self.ways_to_be_option
//             }

//             fn remove_option(&mut self, weights: OptionWeights) {
//                 self.num_options -= 1;
//                 self.weight -= weights;
//             }

//             fn mark_collapsed(&mut self, collapsed_idx: usize) {
//                 self.collapsed_option = Some(collapsed_idx);
//                 self.num_options = 0;
//                 self.weight = OptionWeights::default();
//             }

//             fn weight_sum(&self) -> u32 {
//                 self.weight.0
//             }

//             fn collapsed_idx(&self) -> Option<usize> {
//                 self.collapsed_option
//             }
            
//         }
//     };
// }

#[macro_export]
macro_rules! __impl_collapsible_tile_data {
    (
        struct_name: $name:ident,
        position: $position:ident,
        ways_to_be_option: $ways_to_be_option:ident,
        per_option_data: $per_option_data:ident,
    ) => {

        #[derive(Clone, Debug)]
        pub struct $name {
            collapsed_option: Option<usize>,
            num_options: usize,
            ways_to_be_option: $ways_to_be_option,
            weight: OptionWeights,
            entrophy_noise: f32,
        }

        impl TileData for $name {}

        impl $name {
            pub fn new_uncollapsed_tile(
                num_options: usize,
                ways_to_be_option: $ways_to_be_option,
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

            pub fn new_from_frequency_with_entrophy<R: Rng>(
                rng: &mut R,
                positions: &[$position],
                options_data: &$per_option_data,
            ) -> Vec<($position, Self)> {
                let rng_range = Self::entrophy_uniform();

                let weight = options_data.ways_to_be_option
                    .iter_possible()
                    .map(|option_idx| options_data.get_weights(option_idx))
                    .fold(OptionWeights::default(), |sum, new| sum + new);

                positions
                    .iter()
                    .map(|pos| {
                        (
                            *pos,
                            Self::new_uncollapsed_tile(
                                options_data.possible_options_count,
                                options_data.ways_to_be_option.clone(),
                                weight,
                                rng_range.sample(rng),
                            ),
                        )
                    })
                    .collect::<Vec<_>>()
            }

            pub fn new_from_frequency(
                positions: &[$position],
                options_data: &$per_option_data,
            ) -> Vec<($position, Self)> {
                let weight = options_data.ways_to_be_option
                    .iter_possible()
                    .map(|option_idx| options_data.get_weights(option_idx))
                    .fold(OptionWeights::default(), |sum, new| sum + new);

                positions
                    .iter()
                    .map(|pos| {
                        (
                            *pos,
                            Self::new_uncollapsed_tile(
                                options_data.possible_options_count,
                                options_data.ways_to_be_option.clone(),
                                weight,
                                0.0,
                            ),
                        )
                    })
                .collect::<Vec<_>>()
            }

            pub fn num_possible_options(&self) -> usize {
                self.num_options
            }

            pub fn ways_to_be_option(&self) -> &$ways_to_be_option {
                &self.ways_to_be_option
            }

            pub fn mut_ways_to_be_option(&mut self) -> &mut $ways_to_be_option {
                &mut self.ways_to_be_option
            }

            pub fn remove_option(&mut self, weights: OptionWeights) {
                self.num_options -= 1;
                self.weight -= weights;
            }

            pub fn entrophy_uniform() -> Uniform<f32> {
                Uniform::<f32>::new(0., 0.00001)
            }

            pub fn mark_collapsed(&mut self, collapsed_idx: usize) {
                self.collapsed_option = Some(collapsed_idx);
                self.num_options = 0;
                self.weight = OptionWeights::default();
            }

            pub fn weight_sum(&self) -> u32 {
                self.weight.0
            }

            pub fn is_collapsed(&self) -> bool {
                self.collapsed_idx().is_some()
            }

            pub fn collapsed_idx(&self) -> Option<usize> {
                self.collapsed_option
            }

            pub fn collapse_gather_removed<R: Rng>(
                &mut self,
                rng: &mut R,
                options_data: &$per_option_data,
            ) -> Vec<usize> {
                assert!(
                    self.weight_sum() > 0,
                    "weight sum should be positive when collapsing!"
                );
                let random = rng.gen_range(0..self.weight_sum());
                let mut current_sum = 0;
                let mut chosen = None;
                let mut out = Vec::new();
                for option_idx in self.ways_to_be_option.iter_possible() {
                    current_sum += options_data.get_weights(option_idx).0;
                    if chosen.is_some() || random > current_sum {
                        out.push(option_idx);
                        continue;
                    }
                    chosen = Some(option_idx);
                }
                self.mark_collapsed(chosen.expect("options should always be chosen"));
                out
            }

            pub fn collapse_basic<R: Rng>(&mut self, rng: &mut R, options_data: &$per_option_data) {
                assert!(
                    self.weight_sum() > 0,
                    "weight sum should be positive when collapsing!"
                );
                let random = rng.gen_range(0..self.weight_sum());
                let mut current_sum = 0;
                let mut chosen = None;
                for option_idx in self.ways_to_be_option.iter_possible() {
                    current_sum += options_data.get_weights(option_idx).0;
                    if random > current_sum {
                        continue;
                    }
                    chosen = Some(option_idx);
                    break;
                }
                self.mark_collapsed(chosen.expect("options should always be chosen"));
            }

            pub fn calc_entrophy(&self) -> f32 {
                Self::calc_entrophy_ext(self.weight.0, self.weight.1) + self.entrophy_noise
            }

            #[inline]
            pub fn calc_entrophy_ext(weight_sum: u32, weight_log_sum: f32) -> f32 {
                (weight_sum as f32).log2() - weight_log_sum / (weight_sum as f32)
            }
        }
    };
}
