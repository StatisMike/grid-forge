#[cfg(not(feature = "2d"))]
compile_error!("image feature works only with 2d grid maps - feature `2d` needs to be enabled");

#[doc(inline)]
pub use grid_forge_core::image::PixelWithDefault;

#[cfg(feature = "2d")]
#[doc(inline)]
pub use grid_forge_2d::image::{error, ops, TilePixConst, TilePixVar, TilePixels, WithPixels};
