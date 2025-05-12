/// Macro for defining a pathfinder grid struct with all needed implementations.
#[macro_export]
macro_rules! define_pathfinder_grid {
    (
        // Struct definition
        $(#[$struct_meta:meta])*
        $vis:vis struct $struct_name:ident
        
        // Dimension-specific types
        Dimension = {
            dimension_module: $module:ident,
            dimension_name: $dimension_name:ident,
            GridMap: $grid_map_type:ident,
            Position: $position_type:ident,
            Size: $size_type:ident,
            PathfinderTile: $pathfinder_tile_type:ident,
            Error: $error_type:ident,
            AstarPath: $astar_path_type:ident,
            RangePath: $range_path_type:ident
        }
    ) => {
        macro_rules! __size_type { () => { crate::$module::$size_type }; }
        macro_rules! __position_type { () => { crate::$module::$position_type }; }
        macro_rules! __tile_type { () => { crate::pathfinding::$module::$pathfinder_tile_type<Cost> }; }
        macro_rules! __grid_type { () => { crate::$module::$grid_map_type<__tile_type!()> }; }
        macro_rules! __error_type { () => { crate::pathfinding::$module::$error_type }; }
        macro_rules! __astar_path_type { () => { crate::pathfinding::$module::$astar_path_type }; }
        macro_rules! __range_path_type { () => { crate::pathfinding::$module::$range_path_type }; }

        use crate::common::GridMap as _;

        $(#[$struct_meta])*
        $vis struct $struct_name<Cost> 
        where Cost: crate::pathfinding::common::cost::CostType,
        {
            pub map: __grid_type!(),
        }

        impl<Cost> $struct_name<Cost> 
        where
            Cost: crate::pathfinding::common::cost::CostType,
        {
            #[inline]
            pub fn new(size: __size_type!()) -> Self {
                let map = <__grid_type!()>::new(size);
                Self { map }
            }

            #[inline]
            pub fn astar_find_path(
                &self,
                start: __position_type!(),
                end: __position_type!(),
                cost_type: Cost,
            ) -> Result<__astar_path_type!(), __error_type!()> {
                self._astar_find_path(start, end, cost_type)
            }

            #[inline]
            pub fn astar_find_path_limited(
                &self,
                start: __position_type!(),
                end: __position_type!(),
                cost_type: Cost,
                max_cost: u32,
            ) -> Result<__astar_path_type!(), __error_type!()> {
                self._astar_find_path_limited(start, end, cost_type, max_cost)
            }

            #[inline]
            pub fn range(
                &self,
                start: __position_type!(),
                cost_type: Cost,
                max_cost: u32,
            ) -> Result<__range_path_type!(), __error_type!()> {
                self._range(start, cost_type, max_cost)
            }

            #[inline]
            fn _astar_find_path(
                &self,
                start: __position_type!(),
                end: __position_type!(),
                cost_type: Cost,
            ) -> Result<__astar_path_type!(), __error_type!()> {
                // Implementation goes here
                unimplemented!()
            }

            #[inline]
            fn _astar_find_path_limited(
                &self,
                start: __position_type!(),
                end: __position_type!(),
                cost_type: Cost,
                max_cost: u32,
            ) -> Result<__astar_path_type!(), __error_type!()> {
                // Implementation goes here
                unimplemented!()
            }

            #[inline]
            fn _range(
                &self,
                start: __position_type!(),
                cost_type: Cost,
                max_cost: u32,
            ) -> Result<__range_path_type!(), __error_type!()> {
                // Implementation goes here
                unimplemented!()
            }
        }
    };
}