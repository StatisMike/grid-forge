//! Generative algorithms for procedural generation of gridmaps.

pub mod collapse;
pub mod walker;

pub mod two_d {
    pub mod collapse {
        pub use crate::gen::collapse::two_d::*;
    }
}

pub mod three_d {
    pub mod collapse {
        pub use crate::gen::collapse::three_d::*;
    }
}
