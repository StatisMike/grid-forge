//! Grid map library for Rust.
//! 
//! 

#[cfg(not(any(feature = "2d", feature = "3d")))]
compile_error!("Must enable at least one dimension feature (2d or 3d)");

pub mod prelude;
pub mod core;
pub mod id;

#[cfg(feature = "image")]
pub mod image;