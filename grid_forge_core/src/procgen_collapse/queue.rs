use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::utils::OrderedFloat;

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
    };
}

#[macro_export]
macro_rules! __impl_entrophy_queue {
    (
        struct_name: $name:ident,
        entrophy_item: $entrophy_item:ident,
        per_option_data: $per_option_data:ident,
        grid: $grid:ident,
        position: $position:ident,
    ) => {
        /// Select next position to collapse using smallest entrophy condition.
        ///
        /// Its state will be updated every time after tile entrophy changed by removing some of its options.
        pub struct $name {
            by_entrophy: BTreeSet<$entrophy_item>,
            by_pos: HashMap<$position, OrderedFloat>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    by_entrophy: BTreeSet::new(),
                    by_pos: HashMap::new(),
                }
            }
        }

        impl $name {
            pub(crate) fn get_next_position(&mut self) -> Option<$position> {
                if let Some(item) = self.by_entrophy.pop_first() {
                    self.by_pos.remove(&item.pos);
                    return Some(item.pos);
                }
                None
            }

            pub(crate) fn update_queue(&mut self, position: $position, entrophy: f32) {
                let item = $entrophy_item::new(position, entrophy);
                if let Some(existing_entrophy) = self.by_pos.remove(&item.pos) {
                    self.by_entrophy
                        .remove(&$entrophy_item::new(item.pos, existing_entrophy.into()));
                }
                self.by_pos.insert(item.pos, item.entrophy);
                self.by_entrophy.insert(item);
            }

            pub(crate) fn len(&self) -> usize {
                self.by_entrophy.len()
            }

            pub(crate) fn is_empty(&self) -> bool {
                self.by_entrophy.is_empty()
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_position_queue {
    (
        struct_name: $name:ident,
        per_option_data: $per_option_data:ident,
        queue_starting_point: $queue_starting_point:ident,
        queue_direction: $queue_direction:ident,
        queue_ordering: $queue_ordering:ident,
        position: $position:ident,
        grid: $grid:ident,
    ) => {
        /// A queue that collapses tiles consecutively in a fixed direction, based solely on their position.
        pub struct $name {
            cmp_fun: fn(&$position, &$position) -> std::cmp::Ordering,
            positions: Vec<$position>,
            changed: bool,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    cmp_fun: <$queue_ordering>::cmp_fun_default(),
                    positions: Vec::new(),
                    changed: false,
                }
            }
        }

        impl $name {
            pub fn new(starting: $queue_starting_point, direction: $queue_direction) -> Self {
                Self {
                    cmp_fun: <$queue_ordering>::cmp_fun(starting, direction),
                    ..Default::default()
                }
            }

            pub fn sort_elements(&mut self) {
                self.positions.sort_by(self.cmp_fun);
                self.positions.reverse();
            }

            pub fn get_next_position(&mut self) -> Option<$position> {
                if self.changed {
                    self.sort_elements()
                }
                self.positions.pop()
            }

            pub fn initialize_queue(&mut self, tiles: &[$position]) {
                for element in tiles {
                    self.update_queue(*element);
                }
            }

            pub fn update_queue(&mut self, tile: $position) {
                if !self.positions.contains(&tile) {
                    self.positions.push(tile);
                }
                self.changed = true;
            }

            pub fn len(&self) -> usize {
                self.positions.len()
            }

            pub fn is_empty(&self) -> bool {
                self.positions.is_empty()
            }
        }
    };
}
