use crate::tile::identifiable::builders::ConstructableViaIdentifierTile;
use crate::tile::identifiable::IdentifiableTileData;
use crate::tile::TileData;

pub struct CollapsedTileData {
    tile_type_id: u64,
}

impl TileData for CollapsedTileData {}

impl IdentifiableTileData for CollapsedTileData {
    fn tile_type_id(&self) -> u64 {
        self.tile_type_id
    }
}

impl ConstructableViaIdentifierTile for CollapsedTileData {
    fn tile_new(tile_type_id: u64) -> Self {
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
pub trait CollapsibleTileData: TileData + private::Sealed {
    fn num_compatible_options(&self) -> usize;

    fn has_compatible_options(&self) -> bool {
        self.num_compatible_options() > 0
    }

    fn is_collapsed(&self) -> bool {
        self.collapse_idx().is_some()
    }

    fn collapse_idx(&self) -> Option<usize>;

    /// Create new collapsed tile data.
    fn new_collapsed_data(option_idx: usize) -> Self;

    /// Calculate entrophy.
    fn calc_entrophy(&self) -> f32;

    /// Associate function to calculate entrophy.
    #[inline]
    fn calc_entrophy_ext(weight_sum: u32, weight_log_sum: f32) -> f32 {
        (weight_sum as f32).log2() - weight_log_sum / (weight_sum as f32)
    }
}

pub(crate) mod private {
    use rand::Rng;

    use crate::r#gen::collapse::{entrophy::EntrophyUniform, option::OptionWeights};
    use crate::{
        gen::collapse::option::{PerOptionData, WaysToBeOption},
        tile::{self, GridPosition, GridTile},
    };

    pub trait Sealed {
        fn new_uncollapsed_tile(
            position: GridPosition,
            num_options: usize,
            ways_to_be_option: WaysToBeOption,
            weight: OptionWeights,
            entrophy_noise: f32,
        ) -> GridTile<Self>
        where
            Self: tile::TileData;

        fn new_from_frequency_with_entrophy<R: Rng>(
            rng: &mut R,
            positions: &[GridPosition],
            options_data: &PerOptionData,
        ) -> Vec<GridTile<Self>>
        where
            Self: tile::TileData,
        {
            let rng_range = EntrophyUniform::new();
            let ways_to_be_option = options_data.get_ways_to_become_option();

            let mut weight = ways_to_be_option
                .iter_possible()
                .map(|option_idx| options_data.get_weights(option_idx))
                .fold(OptionWeights::default(), |sum, other| sum + other);
            weight.round();

            positions
                .iter()
                .map(|position| {
                    Self::new_uncollapsed_tile(
                        *position,
                        options_data.num_possible_options(),
                        ways_to_be_option.clone(),
                        weight,
                        rng_range.sample(rng),
                    )
                })
                .collect::<Vec<_>>()
        }

        fn new_from_frequency(
            positions: &[GridPosition],
            options_data: &PerOptionData,
        ) -> Vec<GridTile<Self>>
        where
            Self: tile::TileData,
        {
            let ways_to_be_option = options_data.get_ways_to_become_option();

            let mut weight = ways_to_be_option
                .iter_possible()
                .map(|option_idx| options_data.get_weights(option_idx))
                .fold(OptionWeights::default(), |sum, other| sum + other);

            weight.round();

            positions
                .iter()
                .map(|pos| {
                    Self::new_uncollapsed_tile(
                        *pos,
                        options_data.num_possible_options(),
                        ways_to_be_option.clone(),
                        weight,
                        0.0,
                    )
                })
                .collect::<Vec<_>>()
        }

        fn ways_to_be_option(&self) -> &WaysToBeOption;

        fn mut_ways_to_be_option(&mut self) -> &mut WaysToBeOption;

        fn remove_option(&mut self, weights: OptionWeights);

        fn collapse<R: Rng>(
            &mut self,
            rng: &mut R,
            options_data: &PerOptionData,
        ) -> Option<Vec<usize>>;
    }
}
