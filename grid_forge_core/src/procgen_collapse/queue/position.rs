#[macro_export]
macro_rules! __impl_position_queue {
    (
        struct_name: $name:ident,
        collapsible_tile_data: $collapsible_tile_data:ident,
        queue_starting_point: $queue_starting_point:ident,
        queue_direction: $queue_direction:ident,
        queue_ordering: $queue_ordering:ident,
        position: $position:ident,
    ) => {

        /// A queue that collapses tiles consecutively in a fixed direction, based solely on their position.
        pub struct $name<$collapsible_tile_data> {
            cmp_fun: fn(&$position, &$position) -> std::cmp::Ordering,
            positions: Vec<$position>,
            changed: bool,
            phantom: PhantomData<$collapsible_tile_data>,
        }

        impl Default for $name<$collapsible_tile_data> {
            fn default() -> Self {
                Self {
                    cmp_fun: <$queue_ordering>::cmp_fun_default(),
                    positions: Vec::new(),
                    changed: false,
                    phantom: PhantomData,
                }
            }
        }

        impl $name<$collapsible_tile_data> {
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

            pub fn initialize_queue(&mut self, tiles: &[($position, &$collapsible_tile_data)]) {
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
                grid: &mut impl GridMap<$collapsible_tile_data>,
                positions: &[$position],
                options_data: &$collapsible_tile_data::PerOptionData,
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

/// A queue that collapses tiles consecutively in a fixed direction, based solely on their position.
pub struct PositionQueue<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
{
    cmp_fun: fn(&D::Pos, &D::Pos) -> Ordering,
    positions: Vec<D::Pos>,
    changed: bool,
    phantom: PhantomData<Data>,
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>> Default
    for PositionQueue<D, Data>
{
    fn default() -> Self {
        Self {
            cmp_fun: D::PositionQueueProcession::cmp_fun_default(),
            positions: Vec::new(),
            changed: false,
            phantom: PhantomData,
        }
    }
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    PositionQueue<D, Data>
{
    pub fn new(
        starting: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<
            D,
        >>::StartingPoint,
        direction: <<D as CollapseBounds>::PositionQueueProcession as PositionQueueProcession<D>>::Direction,
    ) -> Self {
        Self {
            cmp_fun: D::PositionQueueProcession::cmp_fun(starting, direction),
            ..Default::default()
        }
    }

    pub fn sort_elements(&mut self) {
        self.positions.sort_by(self.cmp_fun);
        self.positions.reverse();
    }
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    CollapseQueue<D, Data> for PositionQueue<D, Data>
{
    fn get_next_position(&mut self) -> Option<D::Pos> {
        if self.changed {
            self.sort_elements()
        }
        self.positions.pop()
    }

    fn initialize_queue(&mut self, tiles: &[(D::Pos, Data)]) {
        for element in tiles {
            self.update_queue((element.0, &element.1))
        }
    }

    fn update_queue(&mut self, tile: (D::Pos, &Data)) {
        if !self.positions.contains(&tile.0) {
            self.positions.push(tile.0);
        }
        self.changed = true;
    }

    fn len(&self) -> usize {
        self.positions.len()
    }

    fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}

impl<D: Dimensionality + CollapseBounds + ?Sized, Data: CollapsibleTileData<D>>
    super::private::Sealed<D, Data> for PositionQueue<D, Data>
{
    fn populate_inner_grid<R: Rng>(
        &mut self,
        _rng: &mut R,
        grid: &mut impl GridMap<D, Data>,
        positions: &[D::Pos],
        options_data: &D::PerOptionData,
    ) {
        let tiles = Data::new_from_frequency(positions, options_data);
        self.initialize_queue(&tiles);
        for tile in tiles {
            grid.insert_data(&tile.0, tile.1);
        }
    }
}