use std::collections::HashSet;

use crate::{
    core::common::*,
    id::*,
};

use super::{error::CollapsibleGridError, CollapsedTileData, CollapsibleTileData};

pub trait CollapsedGrid<D: Dimensionality>: private::CollapsedGrid<D> {

    fn new(size: &D::Size) -> Self;
    fn grid(&self) -> &Self::Grid;

    fn insert_data(&mut self, position: &D::Pos, data: CollapsedTileData) -> bool {
        let tile_id = data.tile_type_id();
        if self.grid().insert_data(position, data) {
            self.tile_type_ids_mut().insert(tile_id);
            true
        } else {
            false
        }
    }
}

/// Trait shared by a structs holding a grid of [`CollapsibleTileData`], useable by dedicated resolvers to collapse
/// the grid.
pub trait CollapsibleGrid<
    D: Dimensionality,
    IdT: IdentifiableTileData,
>: private::CollapsibleGrid<D> {

    fn retrieve_collapsed(&self) -> Self::Grid;
    fn retrieve_ident<OT: IdentifiableTileData, B: IdentTileBuilder<OT>>(
        &self,
        builder: &B,
    ) -> Result<dyn GridMap<D, OT>, CollapsibleGridError<D>>;

    /// Returns all empty positions in the internal grid.
    fn empty_positions(&self) -> Vec<D::Pos> {
        self._grid().get_all_empty_positions()
    }

    /// Returns all possitions in the internal grid holds collapsed or uncollapsed tiles are either collapsed.
    fn retrieve_positions(&self, collapsed: bool) -> Vec<D::Pos> {
        let func: fn((D::Pos, Self::CollapsibleData)) -> bool = if collapsed {
            |(_, d)| d.is_collapsed()
        } else {
            |(_, d)| !d.is_collapsed()
        };
        self._grid()
            .iter_tiles()
            .filter_map(|t| {
                if func(&t) {
                    Some(t.grid_position())
                } else {
                    None
                }
            })
            .collect()
    }


}


pub(crate) mod private {
    use crate::{core::common::*, r#gen::collapse::{option::private::PerOptionData, PropagateItem}};
    use super::*;


    pub trait CollapsibleGrid<D: Dimensionality> {
        type CollapsibleData: CollapsibleTileData<D>;
        type Grid: GridMap<D, Self::CollapsibleData>;

        #[doc(hidden)]
        fn _grid(&self) -> &Self::Grid;

        #[doc(hidden)]
        fn _grid_mut(&mut self) -> &mut Self::Grid;

        #[doc(hidden)]
        fn _option_data(&self) -> &impl PerOptionData<D>;

        #[doc(hidden)]
        fn _get_initial_propagate_items(&self, to_collapse: &[D::Pos]) -> Vec<PropagateItem<D>>;
    }

    pub trait CollapsedGrid<D: Dimensionality> {
        type Grid: GridMap<D, CollapsedTileData>;

        fn new(size: &D::Size) -> Self;
        fn grid(&self) -> &Self::Grid;
        fn tile_type_ids(&self) -> &HashSet<u64>;
        fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64>;

    }
}
