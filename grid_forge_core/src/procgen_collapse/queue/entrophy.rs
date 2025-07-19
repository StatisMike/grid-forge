use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::utils::OrderedFloat;

#[macro_export]
macro_rules! __impl_entrophy_item {
    (
        struct_name: $name:ident,
        position: $position:ident,
    ) => {
        #[derive(Clone, Copy)]
        pub(crate) struct $name {
            pos: $position,
            entrophy: OrderedFloat,
        }
    
        impl $name {
            pub fn new(pos: $position, entrophy: f32) -> Self {
                Self {
                    pos,
                    entrophy: entrophy.into(),
                }
            }
        
            pub fn pos(&self) -> $position {
                self.pos
            }
        
            pub fn entrophy(&self) -> OrderedFloat {
                self.entrophy
            }
        }

        impl Eq for $name {}

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.entrophy == other.entrophy && self.pos == other.pos
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.entrophy
                    .cmp(&other.entrophy)
                    .then_with(|| self.pos.cmp(&other.pos))
            }
        }
    }
}

#[macro_export]
macro_rules! __impl_entrophy_queue {
    (
        struct_name: $name:ident,
        entrophy_item: $entrophy_item:ident,
        collapsible_tile_data: $collapsible_tile_data:ident,
        per_option_data: $per_option_data:ident,
        grid: $grid:ident,
        position: $position:ident,
    ) => {

        /// Select next position to collapse using smallest entrophy condition.
        ///
        /// Its state will be updated every time after tile entrophy changed by removing some of its options.
        pub struct $name<$collapsible_tile_data> {
            by_entrophy: BTreeSet<$entrophy_item>,
            by_pos: HashMap<$position, OrderedFloat>,
            phantom: PhantomData<$collapsible_tile_data>,
        }

        impl Default for $name<$collapsible_tile_data> {
            fn default() -> Self {
                Self {
                    by_entrophy: BTreeSet::new(),
                    by_pos: HashMap::new(),
                    phantom: PhantomData,
                }
            }
        }

        impl $name<$collapsible_tile_data> {
            pub (crate) fn get_next_position(&mut self) -> Option<$position> {
                if let Some(item) = self.by_entrophy.pop_first() {
                    self.by_pos.remove(&item.pos);
                    return Some(item.pos);
                }
                None
            }

            pub (crate) fn update_queue(&mut self, tile: ($position, &$collapsible_tile_data)) {
                let item = $entrophy_item::new(tile.0, tile.1.calc_entrophy());
                if let Some(existing_entrophy) = self.by_pos.remove(&item.pos) {
                    self.by_entrophy
                        .remove(&$entrophy_item::new(item.pos, existing_entrophy.into()));
                }
                self.by_pos.insert(item.pos, item.entrophy);
                self.by_entrophy.insert(item);
            }

            pub (crate) fn len(&self) -> usize {
                self.by_entrophy.len()
            }

            pub (crate) fn is_empty(&self) -> bool {
                self.by_entrophy.is_empty()
            }

            pub (crate) fn initialize_queue(&mut self, tiles: &[($position, $collapsible_tile_data)]) {
                for element in tiles {
                    self.update_queue((element.0, &element.1))
                }
            }

            pub (crate) fn populate_inner_grid<R: Rng>(
                &mut self,
                rng: &mut R,
                grid: &mut impl $grid<$collapsible_tile_data>,
                positions: &[$position],
                options_data: &$per_option_data,
            ) {
                let tiles = $collapsible_tile_data::new_from_frequency_with_entrophy(rng, positions, options_data);

                self.initialize_queue(&tiles);

                for tile in tiles {
                    grid.insert_data(&tile.0, tile.1);
                }
            }
        }
    };
}

pub struct EntrophyUniform {
    inner: Uniform<u8>,
}

impl EntrophyUniform {
    const MULTIPLIER: u8 = 124;

    pub fn new() -> Self {
        Self {
            inner: Uniform::<u8>::new(0, EntrophyUniform::MULTIPLIER),
        }
    }

    pub fn sample<R: Rng>(&self, rng: &mut R) -> f32 {
        self.inner.sample(rng) as f32 * OrderedFloat::EPSILON
    }
}