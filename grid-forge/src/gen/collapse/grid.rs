use std::collections::HashSet;

use crate::{core::common::*, id::*};

use super::{error::CollapsibleGridError, private::CollapseBounds, CollapsedTileData, CollapsibleTileData};

pub mod two_d {
    use std::collections::HashSet;

    use crate::{core::two_d::*, r#gen::collapse::{two_d::TwoDimCollapseBounds, CollapsedTileData}};

    use super::{private::CommonCollapsedGrid, CollapsedGrid};

    pub struct CollapsedGrid2D {
        grid: GridMap2D<CollapsedTileData>,
        tile_type_ids: HashSet<u64>,
    }

    impl CollapsedGrid<TwoDim, TwoDimCollapseBounds> for CollapsedGrid2D {
        fn new(size: &<TwoDim as Dimensionality>::Size) -> Self {
            Self {
                grid: GridMap2D::new(*size),
                tile_type_ids: HashSet::new(),
            }
        }
        
        fn tile_type_ids(&self) -> &HashSet<u64> {
            &self.tile_type_ids
        }
    }

    impl CommonCollapsedGrid<TwoDim, TwoDimCollapseBounds> for CollapsedGrid2D {

        #[allow(refining_impl_trait)]
        fn grid(&self) -> &GridMap2D<CollapsedTileData> {
            &self.grid
        }

        #[allow(refining_impl_trait)]
        fn grid_mut(&mut self) -> &mut GridMap2D<CollapsedTileData> {
            &mut self.grid
        }

        fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64> {
            &mut self.tile_type_ids
        }
    }
}

pub mod three_d {
    use std::collections::HashSet;

    use crate::{core::three_d::*, r#gen::collapse::{three_d::ThreeDimCollapseBounds, CollapsedTileData}};

    use super::{private::CommonCollapsedGrid, CollapsedGrid};

    pub struct CollapsedGrid3D {
        grid: GridMap3D<CollapsedTileData>,
        tile_type_ids: HashSet<u64>,
    }

    impl CollapsedGrid<ThreeDim, ThreeDimCollapseBounds> for CollapsedGrid3D {
        fn new(size: &GridSize3D) -> Self {
            Self {
                grid: GridMap3D::new(*size),
                tile_type_ids: HashSet::new(),
            }
        }
        
        fn tile_type_ids(&self) -> &HashSet<u64> {
            &self.tile_type_ids
        }
    }

    impl CommonCollapsedGrid<ThreeDim, ThreeDimCollapseBounds> for CollapsedGrid3D {
        #[allow(refining_impl_trait)]
        fn grid(&self) -> &GridMap3D<CollapsedTileData> {
            &self.grid
        }

        #[allow(refining_impl_trait)]
        fn grid_mut(&mut self) -> &mut GridMap3D<CollapsedTileData> {
            &mut self.grid
        }

        fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64> {
            &mut self.tile_type_ids
        }
    }
}

pub trait CollapsedGrid<D: Dimensionality, CD: CollapseBounds<D> + ?Sized>: private::CommonCollapsedGrid<D, CD> {
    fn new(size: &D::Size) -> Self;

    fn insert_data(&mut self, position: &D::Pos, data: CollapsedTileData) -> bool {
        let tile_id = data.tile_type_id();
        if self
            .grid_mut()
            .insert_data(position, data.clone()) {
            self.tile_type_ids_mut().insert(tile_id);
            true
        } else {
            false
        }
    }

    fn insert_tiles(&mut self, tiles: &[(D::Pos, CollapsedTileData)]) {
        for (pos, data) in tiles {
            self.insert_data(pos, *data);
        }
    }

    fn tile_type_ids(&self) -> &HashSet<u64>;
}

/// Trait shared by a structs holding a grid of [`CollapsibleTileData`], useable by dedicated resolvers to collapse
/// the grid.
pub trait CollapsibleGrid<D, CD, IT>
where 
    D: Dimensionality,
    CD: CollapseBounds<D>,
    IT: IdentifiableTileData,
    Self: private::CommonCollapsibleGrid<D, CD>,
{
    fn retrieve_collapsed(&self) -> CD::CollapsedGrid {
        let mut out = CD::CollapsedGrid::new(self._grid().size());

        for (pos, data) in self._grid().iter_tiles() {
            if !data.is_collapsed() {
                continue;
            }

            out.insert_data(&pos, CollapsedTileData::new(self._option_data().get_tile_type_id(
                &data.collapse_idx().expect("cannot get `collapse_idx` for uncollapsed tile")
            ).expect("cannot get `tile_type_id` for uncollapsed tile")));
        }

        out

    }

    fn retrieve_ident<
        OG: GridMap<D, IT>, 
        B: IdentTileBuilder<IT>
    >(
        &self,
        builder: &B,
    ) -> Result<OG, CollapsibleGridError<D>> {
        let mut out = OG::new(*self._grid().size());
        
        for (pos, data) in self._grid().iter_tiles() {
            if !data.is_collapsed() {
                continue;
            }
            out.insert_data(
                &pos, 
                builder.build_tile_unchecked(
                    self._option_data().get_tile_type_id(
                        &data.collapse_idx().expect("cannot get `collapse_idx` for uncollapsed tile")
                    ).expect("cannot get `tile_type_id` for uncollapsed tile")
                )
            );
        }
        
        Ok(out)
    }

    fn retrieve_ident_default<OG: GridMap<D, IT>>(&self) -> OG 
    where IT: IdDefault {
        let mut out = OG::new(*self._grid().size());

        for (pos, data) in self._grid().iter_tiles() {
            if !data.is_collapsed() {
                continue;
            }
            out.insert_data(
                &pos, 
                IT::tile_type_default(
                    self._option_data().get_tile_type_id(
                        &data.collapse_idx().expect("cannot get `collapse_idx` for uncollapsed tile")
                    ).expect("cannot get `tile_type_id` for uncollapsed tile")
                )
            );
        }

        out
    }

    /// Returns all empty positions in the internal grid.
    fn empty_positions(&self) -> Vec<D::Pos> {
        self._grid().get_all_empty_positions()
    }

    /// Returns all possitions in the internal grid holds collapsed or uncollapsed tiles are either collapsed.
    fn retrieve_positions(&self, collapsed: bool) -> Vec<D::Pos> {
        let func: fn((D::Pos, &Self::CollapsibleData)) -> bool = if collapsed {
            |(_, d)| d.is_collapsed()
        } else {
            |(_, d)| !d.is_collapsed()
        };
        self._grid()
            .iter_tiles()
            .filter_map(|t| {
                if func(t) {
                    Some(t.0)
                } else {
                    None
                }
            })
            .collect()
    }

    fn remove_uncollapsed(&mut self) {
        for t in self._grid_mut().iter_mut() {
            if let Some(d) = t {
                if d.is_collapsed() {
                    continue;
                }
                t.take();
            }
        }
    }
}

pub(crate) mod private {
    use std::collections::HashMap;

    use super::*;
    use crate::r#gen::collapse::{option::private::{PerOptionData, WaysToBeOption}, private::CollapseBounds, tile::private::CommonCollapsibleTileData, PropagateItem};

    pub trait CommonCollapsibleGrid<D: Dimensionality, CB: CollapseBounds<D>> {
        type CollapsibleData: CollapsibleTileData<D, CB>;

        type CollapsibleGrid: GridMap<D, Self::CollapsibleData>;

        #[doc(hidden)]
        fn _grid(&self) -> &Self::CollapsibleGrid;

        #[doc(hidden)]
        fn _grid_mut(&mut self) -> &mut Self::CollapsibleGrid;

        #[doc(hidden)]
        fn _option_data(&self) -> &CB::PerOption;

        #[doc(hidden)]
        fn _get_initial_propagate_items(&self, to_collapse: &[D::Pos]) -> Vec<PropagateItem<D>> {
            let mut out = Vec::new();
            let mut cache = HashMap::new();
            let mut check_generated = HashSet::new();
            let check_provided: HashSet<_> = HashSet::from_iter(to_collapse.iter());

            for pos_to_collapse in to_collapse {
                for (pos, neighbour_tile) in self._grid().get_neighbours(pos_to_collapse) {
                    if !neighbour_tile.is_collapsed()
                        || check_provided.contains(&pos)
                        || check_generated.contains(&pos)
                    {
                        continue;
                    }
                    check_generated.insert(pos);
                    let collapsed_idx = neighbour_tile.collapse_idx().unwrap();
                    for opt_to_remove in cache.entry(collapsed_idx).or_insert_with(|| {
                        (0..self._option_data().option_count())
                            .filter(|option_idx| option_idx != &collapsed_idx)
                            .collect::<Vec<usize>>()
                    }) {
                        out.push(PropagateItem::new(pos, *opt_to_remove))
                    }
                }
            }
            out
        }

        /// Removes options from tile neighbours after its collapse.
        fn purge_options_for_neighbours(
            grid: &mut Self::CollapsibleGrid,
            collapsed_option: usize,
            collapsed_position: &D::Pos,
            option_data: &CB::PerOption,
        )
        {
            for direction in D::Dir::all() {
                if let Some((_, tile)) = grid.get_mut_neighbour_at(collapsed_position, direction) {
                    if tile.is_collapsed() {
                        continue;
                    }

                    let enabled =
                        option_data.get_all_enabled_in_direction(collapsed_option, *direction);
                    for possible_option in tile
                        .ways_to_be_option()
                        .iter_possible()
                        .collect::<Vec<_>>()
                    {
                        if !enabled.contains(&possible_option)
                            && tile
                                .mut_ways_to_be_option()
                                .purge_option(possible_option)
                        {
                            let weights = option_data.get_weights(possible_option);
                            tile.remove_option(weights);
                        }
                    }
                }
            }
        }

        /// Removes options from tile based of possible options for its neighbours.
        fn purge_incompatible_options(
            grid: &mut Self::CollapsibleGrid,
            position: &D::Pos,
            option_data: &CB::PerOption,
        ) -> bool
        {
            let num_options = option_data.option_count();
            let mut possible_options = Vec::with_capacity(num_options);
            possible_options.resize(num_options, true);

            for direction in D::Dir::all() {
                if let Some((_, tile)) = grid.get_neighbour_at(position, direction) {
                    if let Some(collapsed_idx) = tile.collapse_idx() {
                        let enabled = option_data
                            .get_all_enabled_in_direction(collapsed_idx, direction.opposite());
                        for (option_idx, state) in possible_options.iter_mut().enumerate() {
                            if *state && !enabled.contains(&option_idx) {
                                *state = false;
                            }
                        }
                    } else if tile.num_compatible_options()
                        < option_data.possible_options_count()
                    {
                        let mut possible_in_any: HashSet<usize> = HashSet::new();
                        for neigbour_idx in tile.ways_to_be_option().iter_possible() {
                            possible_in_any.extend(
                                option_data
                                    .get_all_enabled_in_direction(
                                        neigbour_idx,
                                        direction.opposite(),
                                    )
                                    .iter(),
                            );
                        }
                        for (option_idx, state) in possible_options.iter_mut().enumerate() {
                            if *state && !possible_in_any.contains(&option_idx) {
                                *state = false;
                            }
                        }
                    }
                }
            }

            if !possible_options.iter().any(|state| *state) {
                return false;
            }

            let (_,tile) = grid.get_mut_tile_at_position(position).unwrap();
            for (possible, (option_idx, weights)) in
                possible_options.iter().zip(option_data.iter_weights())
            {
                if !possible
                    && tile
                        .mut_ways_to_be_option()
                        .purge_option(option_idx)
                {
                    tile.remove_option(*weights);
                }
            }
            true
        }
        
    }

    pub trait CommonCollapsedGrid<D: Dimensionality, CD: CollapseBounds<D> + ?Sized> {
        fn grid(&self) -> &impl GridMap<D, CollapsedTileData>;
        fn grid_mut(&mut self) -> &mut impl GridMap<D, CollapsedTileData>;
        fn tile_type_ids_mut(&mut self) -> &mut HashSet<u64>;
    }
}
