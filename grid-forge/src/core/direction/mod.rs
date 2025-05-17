//! Contains the implementation of the core direction structs.
//!
//! Their purpose is to allow:
//! - retrieving the neighbouring tiles in a map
//! - iterating over the directions in dimensionality
//! - fast lookup of some data bound to the specific direction.
//!
//! The handy macros for defining the direction for a new dimensionality are:
//! - [__impl_direction_table](crate::__impl_direction_table)
//! - [__impl_direction_tests](crate::__impl_direction_tests)

pub(crate) mod macros;
pub(crate) mod three_d;
pub(crate) mod two_d;
