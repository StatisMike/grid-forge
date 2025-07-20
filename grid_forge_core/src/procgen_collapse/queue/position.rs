#[macro_export]
macro_rules! __impl_position_queue {
    (
        struct_name: $name:ident,
        collapsible_tile_data: $collapsible_tile_data:ident,
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

            pub fn initialize_queue(&mut self, tiles: &[($position, $collapsible_tile_data)]) {
                for element in tiles {
                    self.update_queue((element.0, &element.1))
                }
            }
    
            pub fn update_queue(&mut self, tile: ($position, &$collapsible_tile_data)) {
                if !self.positions.contains(&tile.0) {
                    self.positions.push(tile.0);
                }
                self.changed = true;
            }
    
            pub fn len(&self) -> usize {
                self.positions.len()
            }
    
            pub fn is_empty(&self) -> bool {
                self.positions.is_empty()
            }

            pub fn populate_inner_grid(
                &mut self,
                grid: &mut impl $grid<$collapsible_tile_data>,
                positions: &[$position],
                options_data: &$per_option_data,
            ) {
                let tiles = $collapsible_tile_data::new_from_frequency(positions, options_data);
                self.initialize_queue(&tiles);
                for tile in tiles {
                    grid.insert_data(&tile.0, tile.1);
                }
            }
        }


    };
}