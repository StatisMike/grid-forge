use std::{collections::{hash_map::Entry, HashMap}, fmt::Display};

use image::{ImageBuffer, Pixel};
use nohash_hasher::{BuildNoHashHasher, NoHashHasher};

use crate::{id::{IdDefault, IdentTileBuilder, SharedData, TypeIdMap, TileIdSet, TypedData}, two_d::{GridMap2D, GridMapShared2D, GridPosition2D, GridSize2D}, TileData};

use super::{tile::{TilePixConst, TilePixels}, PixelWithDefault};

impl <T: TileData> GridMap2D<T> {
    /// Load [`GridMap2D`] with TileData implementing [`TilePixels`] from provided image buffer.
    /// 
    /// Use this function if the tile itself implements [`TilePixels`]. If the tile struct implements [`TypedData`](crate::id::TypedData) and 
    /// the pixels data is separate from the tile, only corresponding to some `tile_type_id`, you can use [`load_from_image_typed`]
    /// or [`load_from_image_typed_auto`] functions.
    ///
    /// # Arguments
    /// - `pix_size` - tuple of the pixel width and height in pixels.
    /// - `image` - an [`ImageBuffer`] containing the source image data.
    /// 
    /// # Returns
    /// - [`GridMap2D`] if successful
    /// - [`VisError2D`] on errors.
    pub fn load_from_image<P>(pix_size: (usize, usize), image: &image::ImageBuffer<P, Vec<P::Subpixel>>) -> Result<Self, VisError2D>
    where T: TilePixels<P>, P: PixelWithDefault + 'static,
    {
        let size = check_grid_vis_size(image, pix_size)?;
        let mut grid = Self::new(size);
        for (pos, pix) in grid.indexed_iter_mut() {
            let mut pix_tile = T::new_empty(pix_size.0, pix_size.1);
            check_tile_vis_size(pix_size, &pix_tile)?;
            read_tile(&mut pix_tile, image, &pos)?;
            pix.replace(pix_tile);
        }
        Ok(grid)
    }

    /// Load [`GridMap2D`] with [`TypedData`](crate::id::TypedData) from provided image buffer and [`TilePixels`] for each tile type.
    /// 
    /// Use this loading function, if:
    /// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise, you can use [`load_from_image`] instead.
    /// - the tile type id and it's corresponding pixels are known while loading the map. Otherwise, if the specific `tile_type_id` is not 
    /// known or is not necessary to be fixed to some value, you can use [`load_from_image_typed_auto`] instead.
    /// 
    /// # Arguments
    /// - `pix_size` - tuple of the pixel width and height in pixels.
    /// - `image` - an [`ImageBuffer`] containing the source image data.
    /// - `builder` - a struct which can be used to construct new tiles on basis of their `tile_id`. One of [`IdentTileBuilder`]
    ///   implementing objects. For [`IdDefault`]-implementors, you can use [`IdentTileDefaultBuilder`](crate::id::IdentTileDefaultBuilder).
    /// - `id_pixel_map` - a slice of tuples of the form `(tile_type_id, tile_pixels)`. The `tile_type_id` **must** be constructable
    ///   by the builder.
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
    pub fn load_from_image_typed<TP, B, P>(
        pix_size: (usize, usize),
        image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
        builder: &B,
        id_pixel_map: &TypeIdMap<TP>,
    ) -> Result<GridMap2D<T>, VisError2D>
    where
        T: TypedData,
        B: IdentTileBuilder<T>,
        TP: TilePixels<P>,
        P: PixelWithDefault + 'static,
    {
        builder.check_missing_ids(&id_pixel_map.keys().copied().collect::<Vec<_>>()).map_err(|e| {
            return VisError2D::new_nopix(e.get_missing_tile_type_ids(), pix_size)
        })?;

        let size = check_grid_vis_size(image, pix_size)?;
        let mut grid = Self::new(size);

        let mut pix_id_map = TypeIdMap::<u64>::default();
        for (id, pix) in id_pixel_map.iter() {
            check_tile_vis_size(pix_size, pix)?;
            pix_id_map.insert(pix.pix_hash(), *id);
        }
        
        for (pos, tile_slot) in grid.indexed_iter_mut() {
            let mut pix_tile = TP::new_empty(pix_size.0, pix_size.1);
            read_tile(&mut pix_tile, image, &pos)?;
            match pix_id_map.get(&pix_tile.pix_hash()) {
                Some(id) => tile_slot.replace(builder.build_tile_unchecked(*id)),
                None => return Err(VisError2D::new_nonexist(pos, pix_size)),
            };
    
        }

        Ok(grid)
    }

    /// Load [`GridMap2D`] with [`TypedData`](crate::id::TypedData) from provided image buffer.
    /// 
    /// Use this loading function, if:
    /// - the tile struct doesn't contain the [`TilePixels`] in its data itself. Otherwise, you can use [`load_from_image`] instead.
    /// - the tile type id and it's corresponding pixels are not strictly known while loading the map. Otherwise, if the `tile_type_id`s
    /// are fixed and their visual representation is known, you can use [`load_from_image_typed`] instead.
    /// 
    /// # Arguments
    /// - `pix_size` - tuple of the pixel width and height in pixels.
    /// - `image` - an [`ImageBuffer`] containing the source image data.
    /// - `builder` - a struct which can be used to construct new tiles on basis of their `tile_id`. One of [`IdentTileBuilder`]
    ///   implementing objects. 
    /// 
    /// As the `tile_type_id` **is automatically calculated** with this function on basis of pixels, 
    /// it won't work if the builder needs to known the `tile_type_id` beforehand - it is recommended to use 
    /// [`IdentTileDefaultBuilder`](crate::id::IdentTileDefaultBuilder) and implement [`IdDefault`] for the tile struct. 
    /// 
    /// # Errors
    /// Function can throw an error if:
    /// - the given [`TilePixels`] type is not compatible with the provided `pix_size`.
    /// - the provided `image` size don't allow the creation of the map with the given `pix_size`.
    /// - the provided `builder` does not have possibility to construct tile of given `tile_type_id`.
    /// 
    /// # Returns
    /// - [`GridMap2D`] if successful
    /// - [`VisError2D`] on errors.
    pub fn load_from_image_typed_auto<TP, B, P>(
        pix_size: (usize, usize),
        image: &image::ImageBuffer<P, Vec<P::Subpixel>>,
        builder: &B,
    ) -> Result<(GridMap2D<T>, TypeIdMap<TP>), VisError2D>
    where
        T: TypedData,
        B: IdentTileBuilder<T>,
        TP: TilePixels<P>,
        P: PixelWithDefault + 'static,
    {
        let mut id_pixel_map = TypeIdMap::<TP>::default();

        let size = check_grid_vis_size(image, pix_size)?;
        let mut grid = GridMap2D::new(size);
        
        for (pos, tile_slot) in grid.indexed_iter_mut() {
            let mut pix_tile = TP::new_empty(pix_size.0, pix_size.1);
            check_tile_vis_size(pix_size, &pix_tile)?;
            read_tile(&mut pix_tile, image, &pos)?;
            let tile_id = pix_tile.pix_hash();
            match id_pixel_map.entry(tile_id) {
                Entry::Occupied(_) => {},
                Entry::Vacant(e) => {
                    e.insert(pix_tile);
                }
            }
            let tile = builder.build_tile(tile_id).map_err(|e| VisError2D::new_nopix(e.get_missing_tile_type_ids(), pix_size))?;
            tile_slot.replace(tile);
        }
        
        Ok((grid, id_pixel_map))
    }

    /// Writes [`GridMap2D`] comprised of tiles containing [`TilePixels`] into provided [`ImageBuffer`].
    /// 
    /// To create a new image buffer with the correct size, 
    /// 
    /// For maps with tiles not implementing [`TilePixels`] themselves, use [`write_to_image_typed`](Self::write_to_image_typed).
    pub fn write_to_image<P>(
        &self,
        image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
        pix_size: (usize, usize),
    ) -> Result<(), VisError2D>
    where
        T: TilePixels<P> + TileData,
        P: PixelWithDefault + 'static,
    {
        check_grid_image_size(image, pix_size, self.size())?;
        for (pos, slot) in self.indexed_iter() {
            let Some(tile) = slot else { continue };
            check_tile_vis_size(pix_size, tile)?;
            write_tile(image, pos, tile)?;
        }
        Ok(())
    }
    
    pub fn write_to_image_typed<TP, P>(
        &self,
        image: &mut ImageBuffer<P, Vec<P::Subpixel>>,
        tile_pixels: &TypeIdMap<TP>,
        pix_size: (usize, usize),
    ) -> Result<(), VisError2D>
    where
        T: TypedData,
        TP: TilePixels<P>,
        P: PixelWithDefault + 'static,
    {
        check_grid_image_size(image, pix_size, self.size())?;
        for (pos, slot) in self.indexed_iter() {
            let Some(tile) = slot else { continue };
            let tile_id = tile.tile_type_id();
            let Some(tile_pix) = tile_pixels.get(&tile_id) else { 
                return Err(VisError2D::new_nopix(&[tile_id], pix_size));
            };
            check_tile_vis_size(pix_size, tile_pix)?;
            write_tile(image, pos, tile_pix)?;
        }
        Ok(())
    }
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

impl <T, S> GridMapShared2D<T, S>
where
    T: TypedData,
    S: SharedData,
{
    pub fn write_to_image<P>(&self, image: &mut ImageBuffer<P, Vec<P::Subpixel>>, pix_size: (usize, usize)) -> Result<(), VisError2D>
    where S: SharedData + TilePixels<P>, P: PixelWithDefault + 'static,
    {
        check_grid_image_size(image, pix_size, self.size())?;
        for position in self.iter_all_positions() {
            let Some(tile_pix) = self.get_shared_data_at_position(&position) else {
                continue;
            };
            check_tile_vis_size(pix_size, tile_pix)?;
            write_tile(image, position, tile_pix)?;
        }
        Ok(())
    }
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

/// Checks the size of the [`ImageBuffer`] while loading [`GridMap2D`] from its visual representation, and produces
/// the [`GridSize2D`] inferred from the image size. Results in [`VisError2D`] if the image size is not compatible
/// with provided tile size in pixels.
pub fn check_grid_vis_size<P: Pixel + 'static>(
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    pix_size: (usize, usize)
) -> Result<GridSize2D, VisError2D> {
    if image.height() as usize % pix_size.0 != 0 || image.width() as usize % pix_size.1 != 0 {
        Err(VisError2D::new_grid_load(image.width(), image.height(), pix_size))
    } else {
        Ok(GridSize2D::new(
            image.width() / pix_size.0 as u32,
            image.height() / pix_size.1 as u32,
        ))
    }
}

#[inline]
fn check_tile_vis_size<P: PixelWithDefault, TP: TilePixels<P>>(
    pix_size: (usize, usize),
    tile: &TP,
) -> Result<(), VisError2D> {
    if tile.pix_width() != pix_size.0 || tile.pix_height() != pix_size.1 {
        Err(VisError2D::new_tile_load(tile.pix_width(), tile.pix_height(), pix_size))
    } else {
        Ok(())
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
    x_pos *= tile_pix.pix_width()as u32;
    y_pos *= tile_pix.pix_height() as u32;

    for y in 0..tile_pix.pix_height() {
        for x in 0..tile_pix.pix_width() {
            if let Some(pixel) = image_buffer.get_pixel_checked(x_pos + x as u32, y_pos + y as u32) {
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
            if let Some(pixel) = image_buffer.get_pixel_mut_checked(x_pos + x as u32, y_pos + y as u32) {
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


/// Error returned by operations on image representations of [`GridMap2D`](crate::map::GridMap2D).
#[derive(Debug, Clone)]
pub struct VisError2D {
    pix_size: (usize, usize),
    kind: VisErrorKind,
}

impl VisError2D {
    pub(crate) fn new_nonexist(pos: GridPosition2D, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::NonExistingTile(pos),
        }
    }

    pub(crate) fn new_grid_load(x: u32, y: u32, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeGridLoad { x, y },
        }
    }

    pub(crate) fn new_grid_save(expected: (u32, u32), actual: (u32, u32), pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeGridSave { expected, actual },
        }
    }
    
    pub(crate) fn new_tile_load(width: usize, height: usize, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeTileLoad { width, height },
        }
    }

    pub(crate) fn new_nopix(tile_ids: &[u64], pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::NoPixelsForIdent(tile_ids.to_vec()),
        }
    }

    pub(crate) fn new_io(read: bool, tile_pos: GridPosition2D, pixel_pos: (u32, u32), pix_size: (usize, usize)) -> Self {
        if read {
            Self {
                pix_size,
                kind: VisErrorKind::PixelRead {
                    tile_pos,
                    pixel_pos,
                },
            }
        } else {
            Self {
                pix_size,
                kind: VisErrorKind::PixelWrite {
                    tile_pos,
                    pixel_pos,
                },
            }
        }
    }
}

impl Display for VisError2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
          VisErrorKind::NonExistingTile(pos) => {
            write!(f, "tile at position: {pos:?} is not contained within used `VisCollection`. Make sure to register it first manually")
          }
            VisErrorKind::WrongSizeGridLoad { x, y } => {
                write!(f, "expected tile pixel size (x: {width}; y: {height}) is incompatible with GridMap image size: (x: {x}, y: {y})", width = self.pix_size.0, height = self.pix_size.1)
            }
            VisErrorKind::NoPixelsForIdent(tile_ids) => write!(
              f,
              "cannot draw tile: no pixels for tile of ids: {tile_ids:?} is present"
          ),
            VisErrorKind::WrongSizeTileLoad { width, height } => write!(f, "cannot load tile: expected tile pixel size (width: {pix_width}; height: {pix_height}), got: width: {width}; height: {height}", pix_width = self.pix_size.0, pix_height = self.pix_size.1),
      VisErrorKind::PixelRead {
          tile_pos,
          pixel_pos,
      } => write!(f, "cannot read tile pixels: image buffer is out of bounds for tile on position: {tile_pos:?}, with pixel: {pixel_pos:?}"),
      VisErrorKind::PixelWrite {
          tile_pos,
          pixel_pos,
      } => write!(f, "cannot draw tile: image buffer is out of bounds for tile on position: {tile_pos:?}, with pixel: {pixel_pos:?}"),
            VisErrorKind::WrongSizeGridSave { expected, actual } => write!(f, "actual image buffer size: {actual:?} differs from expected: {expected:?}"),
        }
    }
}

#[derive(Debug, Clone)]
enum VisErrorKind {
    NonExistingTile(GridPosition2D),
    NoPixelsForIdent(Vec<u64>),
    PixelRead {
        tile_pos: GridPosition2D,
        pixel_pos: (u32, u32),
    },
    PixelWrite {
        tile_pos: GridPosition2D,
        pixel_pos: (u32, u32),
    },
    WrongSizeGridLoad {
        x: u32,
        y: u32,
    },
    WrongSizeTileLoad {
        width: usize,
        height: usize,
    },
    WrongSizeGridSave {
        expected: (u32, u32),
        actual: (u32, u32),
    },
}