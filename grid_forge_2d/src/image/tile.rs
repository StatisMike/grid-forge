//! Objects and traits for the visual representation of the 2D rectangular tile.
//!
//! While working with two-dimensional grid maps, it is possible to represent the tiles and grid maps as a digital image.
//! This module contains the traits and objects allowing to represent the tiles as collection of pixels.
//!
//! There are two main structs that should cover most of the use cases:
//! - [`TilePixConst`] - for constant pixel array - faster, but requires the pixel size to be known at compile time.
//! - [`TilePixVar`] - for variable pixel array - slower, but allows to have the pixel size to be known at runtime.
//!
//! Both of these structs implement [`TilePixels`] trait, which informs the logic behind IO operations on the tiles.
//! If you need some other strategy for the pixel representation, you can implement your own struct implementing
//! this trait. All IO operations for the grid maps are performed on the basis of this trait, instead of the
//! specific structs.

use std::hash::{DefaultHasher, Hash, Hasher};

use grid_forge_core::id::SharedData;
use grid_forge_core::TileData;
use grid_forge_core::image::PixelWithDefault;

pub trait WithPixels<TP: TilePixels<P>, P>
where P: PixelWithDefault {
    fn tile_pixels(&self) -> &TP;
    fn tile_pixels_mut(&mut self) -> &mut TP;
}

// pub trait WithPixelsMut<TP: TilePixelsMut<P>, P>
// where P: PixelWithDefault {
//     fn tile_pixels(&self) -> &TP;
//     fn tile_pixels_mut(&mut self) -> &mut TP;
// }

/// Trait for the visual representation of the tile.
///
/// Two types of implementations of this trait are provided:
/// - [`TilePixConst`] - for constant pixel array - faster, but requires the pixel size to be known at compile time.
/// - [`TilePixVar`] - for variable pixel array - slower, but allows to have the pixel size to be known at runtime.
///
/// Moreover, trait can be applied manually to your own [`TileData`](crate::TileData) or [`SharedData`](crate::id::SharedData)
/// implementations, and be used from the two-dimensional grid map.
pub trait TilePixels<P: PixelWithDefault>: private::Sealed + Clone {
    /// Number of pixels in the horizontal direction.
    fn pix_width(&self) -> usize;

    /// Number of pixels in the vertical direction.
    fn pix_height(&self) -> usize;

    /// Returns a slice of the pixels.
    fn pixels(&self) -> &[P];

    /// Returns a pixel at the specified position.
    ///
    /// # Panics
    /// Panics if the position is out of bounds.
    ///
    /// If you need to handle out of bounds positions, use [`try_pixel()`](TilePixels::try_pixel()) instead.
    fn pixel(&self, x: usize, y: usize) -> P {
        self.pixels()[y * self.pix_width() + x]
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: P);

    /// Returns a pixel if the position is within the bounds.
    ///
    /// For faster but fallible operations, use [`pixel()`](TilePixels::pixel()) instead.
    ///
    /// # Returns
    /// - `Some(Pixel)` if position is within bounds
    /// - `None` otherwise
    fn try_pixel(&self, x: usize, y: usize) -> Option<P> {
        if x < self.pix_width() && y < self.pix_height() {
            Some(self.pixel(x, y))
        } else {
            None
        }
    }

    fn try_set_pixel(&mut self, x: usize, y: usize, pixel: P) -> bool {
        if x < self.pix_width() && y < self.pix_height() {
            self.set_pixel(x, y, pixel);
            true
        } else {
            false
        }
    }

    fn pix_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.pixels().hash(&mut hasher);
        hasher.finish()
    }
}

// pub trait TilePixelsMut<P: PixelWithDefault>: TilePixels<P> {
//     /// Sets a pixel at the specified position.
//     ///
//     /// # Panics
//     /// Panics if the position is out of bounds.
//     fn set_pixel(&mut self, x: usize, y: usize, pixel: P);

//     fn try_set_pixel(&mut self, x: usize, y: usize, pixel: P) -> bool {
//         if x < self.pix_width() && y < self.pix_height() {
//             self.set_pixel(x, y, pixel);
//             true
//         } else {
//             false
//         }
//     }
// }

/// Implementation of the [`TilePixels`] trait for constant pixel array.
///
/// Faster than [`TilePixVar`], but needs to have the pixel size defined at the compile
/// time.
///
/// # Example
/// ```
/// use grid_forge::vis::tile::TilePixConst;
/// use image::Rgb;
///
/// // Specify the pixels for the tiles.
/// let red_pixel = Rgb::<u8>([255, 0, 0]);
/// let blue_pixel = Rgb::<u8>([0, 0, 255]);
/// let green_pixel = Rgb::<u8>([0, 255, 0]);
///
/// // Specify the 10x10 pixel tiles.
/// let red_tile = TilePixConst::<10, 10, Rgb<u8>>::from_slice(&[red_pixel; 100]);
/// let blue_tile = TilePixConst::<10, 10, Rgb<u8>>::from_slice(&[blue_pixel; 100]);
/// let green_tile = TilePixConst::<10, 10, Rgb<u8>>::from_slice(&[green_pixel; 100]);
///
/// // Trait methods provide information about the tiles down the line.
/// use grid_forge::vis::tile::TilePixels;
///
/// assert_eq!(red_tile.pix_width(), 10);
/// assert_eq!(blue_tile.pix_height(), 10);
/// assert_eq!(green_tile.pixels(), &[green_pixel; 100]);
/// assert_eq!(red_tile.pixel(0, 0), red_pixel);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TilePixConst<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> {
    pixels: [[P; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> TilePixConst<WIDTH, HEIGHT, P> {
    pub fn from_slice(pixels: &[P]) -> Self {
        match pixels
            .chunks_exact(WIDTH)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(pixels) => Self { pixels },
            Err(_) => panic!("TilePixConst: pixels length is not divisible by WIDTH"),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> TilePixels<P>
    for TilePixConst<WIDTH, HEIGHT, P>
{
    fn pix_width(&self) -> usize {
        WIDTH
    }
    fn pix_height(&self) -> usize {
        HEIGHT
    }
    fn pixels(&self) -> &[P] {
        // Flatten the 2D array into a 1D slice
        unsafe { std::slice::from_raw_parts(self.pixels.as_ptr() as *const P, WIDTH * HEIGHT) }
    }
    fn pixel(&self, x: usize, y: usize) -> P {
        self.pixels[y][x]
    }
    fn set_pixel(&mut self, x: usize, y: usize, pixel: P) {
        self.pixels[y][x] = pixel;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> Default
    for TilePixConst<WIDTH, HEIGHT, P>
{
    fn default() -> Self {
        Self {
            pixels: [[P::pix_default(); WIDTH]; HEIGHT],
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> TileData
    for TilePixConst<WIDTH, HEIGHT, P>
{
}
impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> SharedData
    for TilePixConst<WIDTH, HEIGHT, P>
{
}
impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault>
    WithPixels<TilePixConst<WIDTH, HEIGHT, P>, P> for TilePixConst<WIDTH, HEIGHT, P>
{
    #[inline]
    fn tile_pixels(&self) -> &Self {
        self
    }
    #[inline]
    fn tile_pixels_mut(&mut self) -> &mut Self {
        self
    }
}

/// Implementation of the [`TilePixels`] trait for variable pixel array.
///
/// Slower than [`TilePixConst`], but allows to have the pixel size defined at the
/// runtime.
///
/// # Example
/// ```
/// use grid_forge::vis::tile::TilePixVar;
/// use image::Rgb;
///
/// // Specify the pixels for the tiles.
/// let red_pixel = Rgb::<u8>([255, 0, 0]);
/// let blue_pixel = Rgb::<u8>([0, 0, 255]);
/// let green_pixel = Rgb::<u8>([0, 255, 0]);
///
/// // Specify the 10x10 pixel tiles.
///
/// let pix_width = 10;
/// let pix_height = 10;
///
/// let red_tile = TilePixVar::<Rgb<u8>>::from_slice(&[red_pixel; 100], pix_width, pix_height);
/// let blue_tile = TilePixVar::<Rgb<u8>>::from_slice(&[blue_pixel; 100], pix_width, pix_height);
/// let green_tile = TilePixVar::<Rgb<u8>>::from_slice(&[green_pixel; 100], pix_width, pix_height);
///
/// // Trait methods provide information about the tiles down the line.
/// use grid_forge::vis::tile::TilePixels;
///
/// assert_eq!(red_tile.pix_width(), 10);
/// assert_eq!(blue_tile.pix_height(), 10);
/// assert_eq!(green_tile.pixels(), &[green_pixel; 100]);
/// assert_eq!(red_tile.pixel(0, 0), red_pixel);
/// ```
#[derive(Clone)]
pub struct TilePixVar<P: PixelWithDefault> {
    width: usize,
    height: usize,
    pixels: Vec<P>,
}

impl<P: PixelWithDefault> TilePixVar<P> {
    pub fn from_slice(pixels: &[P], width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: pixels.to_vec(),
        }
    }
    pub fn new_empty(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        pixels.resize(width * height, P::pix_default());
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl<P: PixelWithDefault> TilePixels<P> for TilePixVar<P> {
    fn pix_width(&self) -> usize {
        self.width
    }
    fn pix_height(&self) -> usize {
        self.height
    }
    fn pixels(&self) -> &[P] {
        &self.pixels
    }
    fn set_pixel(&mut self, x: usize, y: usize, pixel: P) {
        self.pixels[y * self.width + x] = pixel;
    }
}

impl<P: PixelWithDefault> Default for TilePixVar<P> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            pixels: vec![],
        }
    }
}

impl<P: PixelWithDefault> TileData for TilePixVar<P> {}
impl<P: PixelWithDefault> SharedData for TilePixVar<P> {}
impl<P: PixelWithDefault> WithPixels<TilePixVar<P>, P> for TilePixVar<P> {
    #[inline]
    fn tile_pixels(&self) -> &Self {
        self
    }
    #[inline]
    fn tile_pixels_mut(&mut self) -> &mut Self {
        self
    }
}

mod private {
    use grid_forge_core::image::PixelWithDefault;

    use super::{TilePixConst, TilePixVar};

    pub trait Sealed {}

    impl<const WIDTH: usize, const HEIGHT: usize, P: PixelWithDefault> Sealed
        for TilePixConst<WIDTH, HEIGHT, P>
    {
    }
    impl<P: PixelWithDefault> Sealed for TilePixVar<P> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    const TEST_PIXELS: [Rgb<u8>; 9] = [
        Rgb::<u8>([0, 0, 0]),
        Rgb::<u8>([0, 1, 0]),
        Rgb::<u8>([0, 1, 1]),
        Rgb::<u8>([1, 0, 0]),
        Rgb::<u8>([1, 1, 0]),
        Rgb::<u8>([1, 1, 1]),
        Rgb::<u8>([2, 0, 0]),
        Rgb::<u8>([2, 1, 0]),
        Rgb::<u8>([2, 1, 1]),
    ];

    #[test]
    fn test_tile_pix_const() {
        type Vis3x3Pix = TilePixConst<3, 3, Rgb<u8>>;

        let pixels = Vis3x3Pix::from_slice(&TEST_PIXELS);

        println!("{pixels:?}");

        assert_eq!(3, pixels.pix_width());
        assert_eq!(3, pixels.pix_height());
        assert_eq!(&TEST_PIXELS, pixels.pixels());
        assert_eq!(Rgb::<u8>([0, 0, 0]), pixels.pixel(0, 0));
        assert_eq!(Rgb::<u8>([1, 1, 0]), pixels.pixel(1, 1));
        assert_eq!(Rgb::<u8>([2, 1, 1]), pixels.pixel(2, 2));
    }

    #[test]
    fn test_tile_pix_var() {
        type Vis3x3Pix = TilePixVar<Rgb<u8>>;

        let pixels = Vis3x3Pix::from_slice(&TEST_PIXELS, 3, 3);

        assert_eq!(3, pixels.pix_width());
        assert_eq!(3, pixels.pix_height());
        assert_eq!(&TEST_PIXELS, pixels.pixels());
        assert_eq!(Rgb::<u8>([0, 0, 0]), pixels.pixel(0, 0));
        assert_eq!(Rgb::<u8>([1, 1, 0]), pixels.pixel(1, 1));
        assert_eq!(Rgb::<u8>([2, 1, 1]), pixels.pixel(2, 2));
    }

    #[test]
    fn test_tile_pix_const_eq_var() {
        type Vis3x3Pix = TilePixConst<3, 3, Rgb<u8>>;
        type Vis3x3PixVar = TilePixVar<Rgb<u8>>;

        let pixels = Vis3x3Pix::from_slice(&TEST_PIXELS);
        let pixels_var = Vis3x3PixVar::from_slice(&TEST_PIXELS, 3, 3);

        assert_eq!(pixels.pixels(), pixels_var.pixels());
    }
}
