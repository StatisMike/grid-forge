//! Various IO operations transforming between [`GridMap2D`] and [`ImageBuffer`] representation of grid map.

use std::collections::hash_map::Entry;

use image::{ImageBuffer, Pixel};

use crate::core::{Grid2D, GridShared2D, GridMap2D, GridMapShared2D, GridPosition2D, GridSize2D};
use grid_forge_core::id::{IdentTileBuilder, SharedData, TypeIdMap, TypedData};
use grid_forge_core::image::PixelWithDefault;
use grid_forge_core::TileData;

use super::{TilePixConst, TilePixVar, TilePixels, WithPixels};

use super::error::VisError2D;

/// Load [`GridMap2D`] with [`TileData`] containing [`TilePixConst`] data from provided image buffer.
///
///
/// Use this function if the tile itself implements [`WithPixels`], using [`TilePixConst`] as its pixel container. If the tile
/// struct implements [`TypedData`] and the pixels data is separate from the tile and corresponds to some
/// `tile_type_id`, you can use [`load_from_image_const_typed()`] or
/// [`load_from_image_const_typed_auto()`] functions.
///
/// For non-compile time known pixel representation size, use [`load_from_image_var()`] function.
///
/// # Returns
/// - [`GridMap2D`] if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_const<const WIDTH: usize, const HEIGHT: usize, T, P>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
) -> Result<GridMap2D<T>, VisError2D>
where
    T: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P> + TileData + Default,
    P: PixelWithDefault + 'static,
{
    let size = check_grid_vis_size(image, (WIDTH, HEIGHT))?;
    let mut grid = GridMap2D::<T>::new(size);
    for (pos, pix) in grid.enumerate_mut() {
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

/// Load [`GridMap2D`] with [`TileData`] containing [`TilePixVar`] data from provided image buffer.
///
/// Use this function if the tile itself implements [`WithPixels`], using [`TilePixVar`] as its pixel container. If the tile
/// struct implements [`TypedData`] and the pixels data is separate from the tile and corresponds to some
/// `tile_type_id`, you can use [`load_from_image_var_typed()`] or
/// [`load_from_image_var_typed_auto()`] functions.
///
/// For non-compile time known pixel representation size, use [`load_from_image_var()`] function.
///
/// # Returns
/// - [`GridMap2D`] if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_var<T, P>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    pixel_size: (usize, usize),
) -> Result<GridMap2D<T>, VisError2D>
where
    T: WithPixels<TilePixVar<P>, P> + TileData + Default,
    P: PixelWithDefault + 'static,
{
    let size = check_grid_vis_size(image, pixel_size)?;
    let mut grid = GridMap2D::<T>::new(size);
    let mut checked = false;
    for (pos, pix) in grid.enumerate_mut() {
        let mut pix_tile = TilePixVar::<P>::new_empty(pixel_size.0, pixel_size.1);

        if !checked {
            check_tile_vis_size(pixel_size, &pix_tile)?;
            checked = true;
        }

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

/// Load [`GridMap2D`] with [`TypedData`] from provided image buffer.
///
/// Loads the map from the image buffer automatically computing the `tile_type_id` on basis
/// of pixel representation of the tile. Interpretes the pixels as [`TilePixConst`], so the
/// pixel size of tile needs to be known at compile time.
///
/// For non-compile time known pixel representation size, use
/// [`load_from_image_var_typed_auto`](Self::load_from_image_var_typed_auto) function.
///
/// Use this loading function, if:
/// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise, you
///   can use [`load_from_image_const`] instead.
/// - the tile type id and its corresponding pixels are not strictly known while loading the map.
///   Otherwise, if the `tile_type_id`s are fixed and their visual representation is known, you
///   can use [`load_from_image_const_typed`] instead.
///
/// As the `tile_type_id` **is automatically calculated** with this function on basis of pixels,
/// it won't work if the builder needs to known the `tile_type_id` beforehand - it is recommended to use
/// [`IdentTileDefaultBuilder`] and implement [`IdDefault`]
/// for the tile struct.
///
/// # Errors
/// Function can throw an error if:
/// - the given [`TilePixels`] type is not compatible with the provided `pix_size`.
/// - the provided `image` size don't allow the creation of the map with the given `pix_size`.
/// - the provided `builder` does not have possibility to construct tile of given `tile_type_id`.
///
/// # Returns
/// - tuple of [`GridMap2D`] and [`TypeIdMap`] containing the pixel data per `tile_type_id` if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_const_typed_auto<const WIDTH: usize, const HEIGHT: usize, T, B, P>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    builder: &B,
    id_pixel_map: &mut TypeIdMap<TilePixConst<WIDTH, HEIGHT, P>>,
) -> Result<GridMap2D<T>, VisError2D>
where
    T: TypedData,
    B: IdentTileBuilder<T>,
    P: PixelWithDefault + 'static,
{
    let size = check_grid_vis_size(image, (WIDTH, HEIGHT))?;
    let mut grid = GridMap2D::new(size);

    for (pos, tile_slot) in grid.enumerate_mut() {
        let mut pix_tile = TilePixConst::<WIDTH, HEIGHT, P>::default();
        read_tile(&mut pix_tile, image, &pos)?;
        let tile_id = pix_tile.pix_hash();
        match id_pixel_map.entry(tile_id) {
            Entry::Occupied(_) => {}
            Entry::Vacant(e) => {
                e.insert(pix_tile);
            }
        }
        let tile = builder
            .build_tile(tile_id)
            .map_err(|e| VisError2D::new_nopix(e.get_missing_tile_type_ids(), (WIDTH, HEIGHT)))?;
        tile_slot.replace(tile);
    }

    Ok(grid)
}

/// Load [`GridMap2D`] with [`TypedData`] from provided image buffer.
///
/// Loads the map from the image buffer automatically computing the `tile_type_id` on basis
/// of pixel representation of the tile. Interpretes the pixels as [`TilePixVar`], so the
/// pixel size of the tile don't need to be known at compile time.
///
/// For compile time known pixel representation size, use
/// [`load_from_image_var_typed_auto`](Self::load_from_image_var_typed_auto) function.
///
/// Use this loading function, if:
/// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise, you
///   can use [`load_from_image_var`] instead.
/// - the tile type id and its corresponding pixels are not strictly known while loading the map.
///   Otherwise, if the `tile_type_id`s are fixed and their visual representation is known, you
///   can use [`load_from_image_var_typed`] instead.
///
/// As the `tile_type_id` **is automatically calculated** with this function on basis of pixels,
/// it won't work if the builder needs to known the `tile_type_id` beforehand - it is recommended to use
/// [`IdentTileDefaultBuilder`] and implement [`IdDefault`] for the tile struct.
///
/// # Errors
/// Function can throw an error if:
/// - the given [`TilePixels`] type is not compatible with the provided `pix_size`.
/// - the provided `image` size don't allow the creation of the map with the given `pix_size`.
/// - the provided `builder` does not have possibility to construct tile of given `tile_type_id`.
///
/// # Returns
/// - tuple of [`GridMap2D`] and [`TypeIdMap`] containing the pixel data per `tile_type_id` if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_var_typed_auto<T, B, P>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    builder: &B,
    pix_size: (usize, usize),
    id_pixel_map: &mut TypeIdMap<TilePixVar<P>>,
) -> Result<GridMap2D<T>, VisError2D>
where
    T: TypedData,
    B: IdentTileBuilder<T>,
    P: PixelWithDefault + 'static,
{
    let size = check_grid_vis_size(image, pix_size)?;
    let mut grid = GridMap2D::new(size);

    let mut checked = false;

    for (pos, tile_slot) in grid.enumerate_mut() {
        let mut pix_tile = TilePixVar::new_empty(pix_size.0, pix_size.1);

        if !checked {
            check_tile_vis_size(pix_size, &pix_tile)?;
            checked = true;
        }

        read_tile(&mut pix_tile, image, &pos)?;
        let tile_id = pix_tile.pix_hash();
        match id_pixel_map.entry(tile_id) {
            Entry::Occupied(_) => {}
            Entry::Vacant(e) => {
                e.insert(pix_tile);
            }
        }
        let tile = builder
            .build_tile(tile_id)
            .map_err(|e| VisError2D::new_nopix(e.get_missing_tile_type_ids(), pix_size))?;
        tile_slot.replace(tile);
    }

    Ok(grid)
}

/// Load [`GridMap2D`] with [`TypedData`](crate::id::TypedData) from provided image buffer
/// and [`TilePixConst`] mapped to each tile type.
///
/// Loads the grid map using the [`TypeIdMap`] to map visual representation of the tile
/// to the corresponding `tile_type_id`. Interprets the pixels as [`TilePixConst`], so the
/// pixel size of tile needs to be known at compile time.
///
/// For non-compile time known pixel representation size, use
/// [`load_from_image_var_typed`] function.
///
/// Use this loading function, if:
/// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise,
///   you can use [`load_from_image_const`] instead.
/// - the tile type id and its corresponding pixels are known while loading the map.
///   Otherwise, if the specific `tile_type_id` is not known or is not necessary to be fixed
///   to some value, you can use
///   [`load_from_image_const_typed_auto`] instead.
///
/// # Errors
/// Function can throw an error if:
/// - the given [`TilePixels`] are not present in the `id_pixel_map`.
/// - the given [`TilePixels`] are not compatible with the provided `pix_size`.
/// - the provided `image` size don't allow the creation of the map with the given `pix_size`.
/// - the provided `builder` does not have possibility to construct tile of given `tile_type_id`.
///
/// # Returns
/// - [`GridMap2D`] if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_const_typed<const WIDTH: usize, const HEIGHT: usize, T, WP, P, B>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    builder: &B,
    id_pixel_map: &TypeIdMap<WP>,
) -> Result<GridMap2D<T>, VisError2D>
where
    T: TypedData,
    WP: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
    B: IdentTileBuilder<T>,
    P: PixelWithDefault + 'static,
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

    for (pos, tile_slot) in grid.enumerate_mut() {
        let mut pix_tile = TilePixConst::<WIDTH, HEIGHT, P>::default();
        read_tile(&mut pix_tile, image, &pos)?;
        match pix_id_map.get(&pix_tile.pix_hash()) {
            Some(id) => tile_slot.replace(builder.build_tile_unchecked(*id)),
            None => return Err(VisError2D::new_nonexist(pos, (WIDTH, HEIGHT))),
        };
    }

    Ok(grid)
}

/// Load [`GridMap2D`] with [`TypedData`](crate::id::TypedData) from provided image buffer
/// and [`TilePixVar`] mapped to each tile type.
///
/// Loads the grid map using the [`TypeIdMap`] to map visual representation of the tile
/// to the corresponding `tile_type_id`. Interprets the pixels as [`TilePixVar`], so the
/// pixel size of tile doesn't need to be known at compile time.
///
/// For compile-time known pixel representation size, use
/// [`load_from_image_const_typed`](Self::load_from_image_const_typed) function.
///
/// Use this loading function, if:
/// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise,
///   you can use [`load_from_image_var`](Self::load_from_image_var) instead.
/// - the tile type id and its corresponding pixels are known while loading the map.
///   Otherwise, if the specific `tile_type_id` is not known or is not necessary to be fixed
///   to some value, you can use
///   [`load_from_image_var_typed_auto`](Self::load_from_image_var_typed_auto) instead.
///
/// # Errors
/// Function can throw an error if:
/// - the given [`TilePixels`] are not present in the `id_pixel_map`.
/// - the given [`TilePixels`] are not compatible with the provided `pix_size`.
/// - the provided `image` size don't allow the creation of the map with the given `pix_size`.
/// - the provided `builder` does not have possibility to construct tile of given `tile_type_id`.
///
/// # Returns
/// - [`GridMap2D`] if successful
/// - [`VisError2D`] on errors.
pub fn load_from_image_var_typed<T, B, WP, P>(
    image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
    id_pixel_map: &TypeIdMap<WP>,
    builder: &B,
    pix_size: (usize, usize),
) -> Result<GridMap2D<T>, VisError2D>
where
    T: TypedData,
    WP: WithPixels<TilePixVar<P>, P>,
    B: IdentTileBuilder<T>,
    P: PixelWithDefault + 'static,
{
    builder
        .check_missing_ids(&id_pixel_map.keys().copied().collect::<Vec<_>>())
        .map_err(|e| return VisError2D::new_nopix(e.get_missing_tile_type_ids(), pix_size))?;

    let size = check_grid_vis_size(image, pix_size)?;
    let mut grid = GridMap2D::<T>::new(size);

    let mut pix_id_map = TypeIdMap::<u64>::default();
    for (id, pix) in id_pixel_map.iter() {
        check_tile_vis_size(pix_size, pix.tile_pixels())?;
        pix_id_map.insert(pix.tile_pixels().pix_hash(), *id);
    }

    for (pos, tile_slot) in grid.enumerate_mut() {
        let mut pix_tile = TilePixVar::<P>::new_empty(pix_size.0, pix_size.1);
        read_tile(&mut pix_tile, image, &pos)?;
        match pix_id_map.get(&pix_tile.pix_hash()) {
            Some(id) => tile_slot.replace(builder.build_tile_unchecked(*id)),
            None => return Err(VisError2D::new_nonexist(pos, pix_size)),
        };
    }

    Ok(grid)
}

/// Writes [`GridMap2D`] comprised of tiles containing [`TilePixels`] into provided [`ImageBuffer`].
///
/// To create a new image buffer with the correct size,
///
/// For maps with tiles not implementing [`TilePixels`] themselves, use [`write_to_image_typed`](Self::write_to_image_typed).
pub fn write_to_image_const<const WIDTH: usize, const HEIGHT: usize, G, T, P>(
    map: &G,
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
) -> Result<(), VisError2D>
where
    G: Grid2D<T>,
    T: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P> + TileData,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, (WIDTH, HEIGHT), map.size())?;
    for (pos, slot) in map.enumerate() {
        let Some(tile) = slot else { continue };
        write_tile(image, pos, tile.tile_pixels())?;
    }
    Ok(())
}

pub fn write_to_image_var<G, T, P>(
    map: &G,
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    pix_size: (usize, usize),
) -> Result<(), VisError2D>
where
    G: Grid2D<T>,
    T: WithPixels<TilePixVar<P>, P> + TileData,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, pix_size, map.size())?;
    for (pos, slot) in map.enumerate() {
        let Some(tile) = slot else { continue };
        write_tile(image, pos, tile.tile_pixels())?;
    }
    Ok(())
}

pub fn write_to_image_const_typed<const WIDTH: usize, const HEIGHT: usize, G, T, WP, P>(
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    map: &G,
    tile_pixels: &TypeIdMap<WP>,
) -> Result<(), VisError2D>
where
    G: Grid2D<T>,
    T: TypedData,
    WP: WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, (WIDTH, HEIGHT), map.size())?;
    for (pos, slot) in map.enumerate() {
        let Some(tile) = slot else { continue };
        let tile_id = tile.tile_type_id();
        let Some(tile_pix) = tile_pixels.get(&tile_id) else {
            return Err(VisError2D::new_nopix(&[tile_id], (WIDTH, HEIGHT)));
        };
        write_tile(image, pos, tile_pix.tile_pixels())?;
    }
    Ok(())
}

pub fn write_to_image_var_typed<G, T, WP, P>(
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    map: &G,
    tile_pixels: &TypeIdMap<WP>,
    pix_size: (usize, usize),
) -> Result<(), VisError2D>
where
    G: Grid2D<T>,
    T: TypedData,
    WP: WithPixels<TilePixVar<P>, P>,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, pix_size, map.size())?;
    for (pos, slot) in map.enumerate() {
        let Some(tile) = slot else { continue };
        let tile_id = tile.tile_type_id();
        let Some(tile_pix) = tile_pixels.get(&tile_id) else {
            return Err(VisError2D::new_nopix(&[tile_id], pix_size));
        };
        check_tile_vis_size(pix_size, tile_pix.tile_pixels())?;
        write_tile(image, pos, tile_pix.tile_pixels())?;
    }
    Ok(())
}

pub fn write_to_image_const_shared<const WIDTH: usize, const HEIGHT: usize, T, S, P>(
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    map: &GridMapShared2D<T, S>,
) -> Result<(), VisError2D>
where
    T: TypedData,
    S: SharedData + WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P>,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, (WIDTH, HEIGHT), map.size())?;
    for position in map.iter_positions() {
        let Some(tile_pix) = map.shared_at(&position) else {
            continue;
        };
        write_tile(image, position, tile_pix.tile_pixels())?;
    }
    Ok(())
}

pub fn write_to_image_var_shared<T, S, P>(
    image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    map: &GridMapShared2D<T, S>,
    pix_size: (usize, usize),
) -> Result<(), VisError2D>
where
    T: TypedData,
    S: SharedData + WithPixels<TilePixVar<P>, P>,
    P: PixelWithDefault + 'static,
{
    check_grid_image_size(image, pix_size, map.size())?;
    for position in map.iter_positions() {
        let Some(tile_pix) = map.shared_at(&position) else {
            continue;
        };
        check_tile_vis_size(pix_size, tile_pix.tile_pixels())?;
        write_tile(image, position, tile_pix.tile_pixels())?;
    }
    Ok(())
}

/// Utility function to generate [`ImageBuffer`] of correct size for specific size of [`GridMap2D`] to write into.
pub fn init_map_image_buffer<P>(
    grid_size: &GridSize2D,
    pixel_size: (u32, u32),
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    P: PixelWithDefault,
{
    ImageBuffer::new(grid_size.x() * pixel_size.0, grid_size.y() * pixel_size.1)
}

/// Checks the size of the [`ImageBuffer`] while loading [`GridMap2D`] from its visual representation, and produces
/// the [`GridSize2D`] inferred from the image size. Results in [`VisError2D`] if the image size is not compatible
/// with provided tile size in pixels.
pub fn check_grid_vis_size<P: Pixel + 'static>(
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    pix_size: (usize, usize),
) -> Result<GridSize2D, VisError2D> {
    if image.height() as usize % pix_size.0 != 0 || image.width() as usize % pix_size.1 != 0 {
        Err(VisError2D::new_grid_load(
            image.width(),
            image.height(),
            pix_size,
        ))
    } else {
        Ok(GridSize2D::new(
            image.width() / pix_size.0 as u32,
            image.height() / pix_size.1 as u32,
        ))
    }
}

/// Reads pixels from image that represents the tile at specified [`GridPosition2D`].
pub fn read_tile<P>(
    tile_pix: &mut impl TilePixels<P>,
    image_buffer: &ImageBuffer<P, Vec<P::Subpixel>>,
    pos: &GridPosition2D,
) -> Result<(), VisError2D>
where
    P: PixelWithDefault,
{
    let [mut x_pos, mut y_pos] = pos.coords();
    x_pos *= tile_pix.pix_width() as u32;
    y_pos *= tile_pix.pix_height() as u32;

    for y in 0..tile_pix.pix_height() {
        for x in 0..tile_pix.pix_width() {
            if let Some(pixel) = image_buffer.get_pixel_checked(x_pos + x as u32, y_pos + y as u32)
            {
                tile_pix.set_pixel(x, y, *pixel);
            } else {
                return Err(VisError2D::new_io(
                    true,
                    *pos,
                    (x_pos + x as u32, y_pos + y as u32),
                    (tile_pix.pix_width(), tile_pix.pix_height()),
                ));
            }
        }
    }
    Ok(())
}

/// Writes tile pixels to the image buffer, in place defined by the provided [`GridPosition2D`].
pub fn write_tile<P>(
    image_buffer: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    pos: GridPosition2D,
    tile_pix: &impl TilePixels<P>,
) -> Result<(), VisError2D>
where
    P: PixelWithDefault,
{
    let [mut x_pos, mut y_pos] = pos.coords();
    x_pos *= tile_pix.pix_width() as u32;
    y_pos *= tile_pix.pix_height() as u32;

    for y in 0..tile_pix.pix_height() {
        for x in 0..tile_pix.pix_width() {
            if let Some(pixel) =
                image_buffer.get_pixel_mut_checked(x_pos + x as u32, y_pos + y as u32)
            {
                *pixel = tile_pix.pixel(x, y);
            } else {
                return Err(VisError2D::new_io(
                    false,
                    pos,
                    (x_pos + x as u32, y_pos + y as u32),
                    (tile_pix.pix_width(), tile_pix.pix_height()),
                ));
            }
        }
    }
    Ok(())
}

/// Checks the size of the [`ImageBuffer`] before writing [`GridMap2D`] visual representation into it. Results in
/// [`VisError2D`] if the image buffer size and [`GridSize2D`] does not match.
fn check_grid_image_size<P: Pixel + 'static>(
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    pix_size: (usize, usize),
    size: &GridSize2D,
) -> Result<(), VisError2D> {
    let expected = (size.x() * pix_size.0 as u32, size.y() * pix_size.1 as u32);

    if expected != (image.width(), image.height()) {
        Err(VisError2D::new_grid_save(
            expected,
            (image.width(), image.height()),
            pix_size,
        ))
    } else {
        Ok(())
    }
}

#[inline]
fn check_tile_vis_size<P: PixelWithDefault, TP: TilePixels<P>>(
    pix_size: (usize, usize),
    tile: &TP,
) -> Result<(), VisError2D> {
    if tile.pix_width() != pix_size.0 || tile.pix_height() != pix_size.1 {
        Err(VisError2D::new_tile_load(
            tile.pix_width(),
            tile.pix_height(),
            pix_size,
        ))
    } else {
        Ok(())
    }
}
