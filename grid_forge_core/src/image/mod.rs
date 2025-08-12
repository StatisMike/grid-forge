//! Allows operating on image representations of grid maps.

use std::hash::Hash;

use image::{Luma, LumaA, Pixel, Rgb, Rgba};

/// Trait for retrieving default value for pixels.
///
/// Implemented for common pixel types.
pub trait PixelWithDefault
where
    Self: Pixel + Hash + PartialEq,
{
    fn pix_default() -> Self;
}

impl<T> PixelWithDefault for Rgb<T>
where
    T: image::Primitive,
    Self: Pixel + Hash + PartialEq,
{
    fn pix_default() -> Self {
        Rgb([T::DEFAULT_MIN_VALUE; 3])
    }
}

impl<T> PixelWithDefault for Rgba<T>
where
    T: image::Primitive,
    Self: Pixel + Hash + PartialEq,
{
    fn pix_default() -> Self {
        Rgba([T::DEFAULT_MIN_VALUE; 4])
    }
}

impl<T> PixelWithDefault for Luma<T>
where
    T: image::Primitive,
    Self: Pixel + Hash + PartialEq,
{
    fn pix_default() -> Self {
        Luma([T::DEFAULT_MIN_VALUE])
    }
}

impl<T> PixelWithDefault for LumaA<T>
where
    T: image::Primitive,
    Self: Pixel + Hash + PartialEq,
{
    fn pix_default() -> Self {
        LumaA([T::DEFAULT_MIN_VALUE; 2])
    }
}
