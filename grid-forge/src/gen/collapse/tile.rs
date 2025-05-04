use crate::id::*;
use crate::two_d::*;

use super::private::CollapseBounds;

/// Simple [`TileData`] containing only the `tile_type_id`.
///
/// Identical in most cases to [`BasicIdentTileData`](crate::tile::identifiable::BasicIdentTileData), but used consistently within the
/// collapse algorithms - both as input for some initial constraints for the generation process, and as an collapse process output.
#[derive(Clone, Copy, Debug)]
pub struct CollapsedTileData {
    tile_type_id: u64,
}

impl TileData for CollapsedTileData {}

impl IdentifiableTileData for CollapsedTileData {
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

/// Trait shared by [`TileData`] used within collapsible generative algorithms.
pub trait CollapsibleTileData<D: Dimensionality + CollapseBounds + ?Sized>:
    TileData + private::CommonCollapsibleTileData<D>
{
    /// Returns number of possible options for the tile.
    fn num_compatible_options(&self) -> usize;

    /// Checks if the tile has any possible options.
    fn has_compatible_options(&self) -> bool {
        self.num_possible_options() > 0
    }

    /// Checks if the tile is collapsed.
    fn is_collapsed(&self) -> bool {
        self.collapse_idx().is_some()
    }

    /// Returns the index of the collapsed option.
    fn collapse_idx(&self) -> Option<usize>;

    /// Create new collapsed tile data.
    fn new_collapsed_data(option_idx: usize) -> Self;

    /// Calculate entrophy.
    fn calc_entrophy(&self) -> f32;

    /// Associated function to calculate entrophy.
    #[inline]
    fn calc_entrophy_ext(weight_sum: u32, weight_log_sum: f32) -> f32 {
        (weight_sum as f32).log2() - weight_log_sum / (weight_sum as f32)
    }
}

pub(crate) mod private {

    use rand::{distributions::Uniform, Rng};

    use crate::{
        core::common::*,
        r#gen::collapse::common::{entrophy::EntrophyUniform, CollapseBounds},
        r#gen::collapse::option::private::{PerOptionData, WaysToBeOption},
    };

    use crate::gen::collapse::option::OptionWeights;

    /// Sealed trait for the [`CollapsibleTileData`] trait. It contains most of the shared logic for its implementors,
    /// which should be kept private.
    pub trait CommonCollapsibleTileData<D: Dimensionality + CollapseBounds + ?Sized>:
        TileData
    {
        /// Creates new uncollapsed tile.
        fn new_uncollapsed_tile(
            num_options: usize,
            ways_to_be_option: D::WaysToBeOption,
            weight: OptionWeights,
            entrophy_noise: f32,
        ) -> Self;

        /// Creates vector of uncollapsed tiles with entrophy noise.
        fn new_from_frequency_with_entrophy<R: Rng>(
            rng: &mut R,
            positions: &[D::Pos],
            options_data: &D::PerOptionData,
        ) -> Vec<(D::Pos, Self)> {
            let rng_range = EntrophyUniform::new();
            let ways_to_be_option = options_data.ways_to_be_option();

            let weight = ways_to_be_option
                .iter_possible()
                .map(|option_idx| options_data.get_weights(option_idx))
                .fold(OptionWeights::default(), |sum, new| sum + new);

            positions
                .iter()
                .map(|position| {
                    (
                        *position,
                        Self::new_uncollapsed_tile(
                            options_data.possible_options_count(),
                            ways_to_be_option.clone(),
                            weight,
                            rng_range.sample(rng),
                        ),
                    )
                })
                .collect::<Vec<_>>()
        }

        /// Creates vector of uncollapsed tiles.
        fn new_from_frequency(
            positions: &[D::Pos],
            options_data: &D::PerOptionData,
        ) -> Vec<(D::Pos, Self)> {
            let ways_to_be_option = options_data.ways_to_be_option();

            let weight = ways_to_be_option
                .iter_possible()
                .map(|option_idx| options_data.get_weights(option_idx))
                .fold(OptionWeights::default(), |sum, new| sum + new);

            positions
                .iter()
                .map(|pos| {
                    (
                        *pos,
                        Self::new_uncollapsed_tile(
                            options_data.possible_options_count(),
                            ways_to_be_option.clone(),
                            weight,
                            0.0,
                        ),
                    )
                })
                .collect::<Vec<_>>()
        }

        fn num_possible_options(&self) -> usize;

        fn ways_to_be_option(&self) -> &D::WaysToBeOption;

        fn mut_ways_to_be_option(&mut self) -> &mut D::WaysToBeOption;

        /// Removes single option from tile.
        fn remove_option(&mut self, weights: OptionWeights);

        /// Range of uniformly distributed data for entrophy noise.
        fn entrophy_uniform() -> Uniform<f32> {
            Uniform::<f32>::new(0., 0.00001)
        }

        fn mark_collapsed(&mut self, collapsed_idx: usize);

        fn weight_sum(&self) -> u32;

        /// Collapses tile into one of possible options, returning the vector of the removed options.
        fn collapse_gather_removed<R: Rng>(
            &mut self,
            rng: &mut R,
            options_data: &D::PerOptionData,
        ) -> Vec<usize> {
            assert!(
                self.weight_sum() > 0,
                "weight sum should be positive when collapsing!"
            );
            let random = rng.gen_range(0..self.weight_sum());
            let mut current_sum = 0;
            let mut chosen = None;
            let mut out = Vec::new();
            for option_idx in self.ways_to_be_option().iter_possible() {
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

        /// Collapses tiles into one of possible options.
        fn collapse_basic<R: Rng>(&mut self, rng: &mut R, options_data: &D::PerOptionData) {
            assert!(
                self.weight_sum() > 0,
                "weight sum should be positive when collapsing!"
            );
            let random = rng.gen_range(0..self.weight_sum());
            let mut current_sum = 0;
            let mut chosen = None;
            for option_idx in self.ways_to_be_option().iter_possible() {
                current_sum += options_data.get_weights(option_idx).0;
                if chosen.is_some() || random > current_sum {
                    continue;
                }
                chosen = Some(option_idx);
            }
            self.mark_collapsed(chosen.expect("options should always be chosen"));
        }
    }
}
