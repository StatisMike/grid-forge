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