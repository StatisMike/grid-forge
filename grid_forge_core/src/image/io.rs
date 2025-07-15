use std::collections::hash_map::Entry;

use crate::{id::{IdentTileBuilder, TypeIdMap, TypedData}, two_d::GridMap2D, vis::tile::TilePixels, TileData};

use super::{grid::{check_grid_vis_size, read_tile, VisError2D}, tile::{TilePixConst, WithPixels}, PixelWithDefault};

pub struct VisGridLoaderConst2D<const WIDTH: usize, const HEIGHT: usize, T, PD, P>
where  
T: TileData,
PD: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
P: PixelWithDefault + 'static,
{
    phantom: std::marker::PhantomData<(P, T, PD)>,
}

impl<const WIDTH: usize, const HEIGHT: usize, T, PD, P> VisGridLoaderConst2D<WIDTH, HEIGHT, T, PD, P> 
where
T: TileData,
PD: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
P: PixelWithDefault + 'static,
{    
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T, P> VisGridLoaderConst2D<WIDTH, HEIGHT, T, T, P> 
where
T: TileData + WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P> + Default,
P: PixelWithDefault + 'static,
{
    pub fn load(
        &self,
        image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    ) -> Result<GridMap2D<T>, VisError2D>
    {
        let size = check_grid_vis_size(image, (WIDTH, HEIGHT))?;
        let mut grid = GridMap2D::<T>::new(size);
        for (pos, pix) in grid.indexed_iter_mut() {
            let mut pix_tile = TilePixConst::<WIDTH, HEIGHT, P>::default();
            read_tile(&mut pix_tile, image, &pos)?;
            match pix {
                Some(ref mut existing_pix) => {
                    *existing_pix.tile_pixels_mut() = pix_tile;
                }
                None => {
                    *pix = Some(T::default());
                    if let Some(ref mut existing_pix) = pix {
                        *existing_pix.tile_pixels_mut() = pix_tile;
                    }
                }
            }
        }
        Ok(grid)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T, PD, P> VisGridLoaderConst2D<WIDTH, HEIGHT, T, PD, P> 
where
T: TypedData,
PD: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
P: PixelWithDefault + 'static,
{
    pub fn load_typed<B>(
        image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
        builder: &B,
        id_pixel_map: &TypeIdMap<PD>,
    ) -> Result<GridMap2D<T>, VisError2D>
    where B: IdentTileBuilder<T>,
    {
        builder
            .check_missing_ids(&id_pixel_map.keys().copied().collect::<Vec<_>>())
            .map_err(|e| {
                return VisError2D::new_nopix(e.get_missing_tile_type_ids(), (WIDTH, HEIGHT));
            })?;

        let size = check_grid_vis_size(image, (WIDTH, HEIGHT))?;
        let mut grid = GridMap2D::<T>::new(size);

        let mut pix_id_map = TypeIdMap::<u64>::default();
        for (id, pix) in id_pixel_map.iter() {
            pix_id_map.insert(pix.tile_pixels().pix_hash(), *id);
        }

        for (pos, tile_slot) in grid.indexed_iter_mut() {
            let mut pix_tile = TilePixConst::<WIDTH, HEIGHT, P>::default();
            read_tile(&mut pix_tile, image, &pos)?;
            match pix_id_map.get(&pix_tile.pix_hash()) {
                Some(id) => tile_slot.replace(builder.build_tile_unchecked(*id)),
                None => return Err(VisError2D::new_nonexist(pos, (WIDTH, HEIGHT))),
            };
        }

        Ok(grid)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T, PD, P> VisGridLoaderConst2D<WIDTH, HEIGHT, T, PD, P> 
where
T: TypedData,
P: PixelWithDefault + 'static,
{
    pub fn load_from_image_const_typed_auto<B>(
        image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
        builder: &B,
    ) -> Result<(GridMap2D<T>, TypeIdMap<TilePixConst<WIDTH, HEIGHT, P>>), VisError2D>
    where
    B: IdentTileBuilder<T>,

    {
        let mut id_pixel_map = TypeIdMap::<TilePixConst<WIDTH, HEIGHT, P>>::default();

        let size = check_grid_vis_size(image, (WIDTH, HEIGHT))?;
        let mut grid = GridMap2D::new(size);

        for (pos, tile_slot) in grid.indexed_iter_mut() {
            let mut pix_tile = TilePixConst::<WIDTH, HEIGHT, P>::default();
            read_tile(&mut pix_tile, image, &pos)?;
            let tile_id = pix_tile.pix_hash();
            match id_pixel_map.entry(tile_id) {
                Entry::Occupied(_) => {}
                Entry::Vacant(e) => {
                    e.insert(pix_tile);
                }
            }
            let tile = builder.build_tile(tile_id).map_err(|e| {
                VisError2D::new_nopix(e.get_missing_tile_type_ids(), (WIDTH, HEIGHT))
            })?;
            tile_slot.replace(tile);
        }

        Ok((grid, id_pixel_map))
    }
}