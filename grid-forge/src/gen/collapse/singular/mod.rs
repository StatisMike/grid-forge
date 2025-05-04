//! # Singular collapsible generation
//!
//! Collapsible generation algorithm working with rulesets that are based on tile type adjacencies. Often named within other
//! sources as *single-tiled* algorithms, both *Model Synthesis* and *Wave-Function Collapse* can fall into this category.
//!
//! As it is based on tile type adjacencies, it is more viable to describe the rules manually than pattern-based
//! [`overlap`](crate::gen::collapse::overlap).
//!
//! ## Struct types
//!
//! In general the types are described in the documentation for [`collapse`](crate::gen::collapse) module.
//!
//! - [`AdjacencyRules`] and [`FrequencyHints`] are self-descriptive. The latter are not produced by the *analyzer*, but the method
//!   for their derivation from the sample gridmap is exposed..
//! - [`Analyzer`] is a trait implemented by two distincts analyzers. The [`IdentityAnalyzer`] in general produced more restrictive rules,
//!   as it search for exact neigbours on the sample gridmap. The [`BorderAnalyzer`] is more liberal, as it takes an extra step and derives
//!   more rules based on the distinct tile borders, making additional options available if they *could be* placed on the sample gridmap
//!   next to each other.
//! - [`CollapsibleTileGrid`] is the collection of [`CollapsibleTile`].
//! - [`Resolver`] is the main executor of the algorithm.

pub mod analyzer;
pub mod resolver;
pub mod subscriber;

use {
    super::{common::CollapseBounds, grid::CollapsibleGrid},
    crate::{core::common::Dimensionality, id::IdentifiableTileData},
    analyzer::{AdjacencyRules, FrequencyHints},
};

pub trait CollapsibleTileGrid<D, IT>
where
    D: Dimensionality + CollapseBounds + ?Sized,
    IT: IdentifiableTileData,
    Self: CollapsibleGrid<D, IT>,
{
    fn new_empty(
        size: D::Size,
        frequencies: &FrequencyHints<D, IT>,
        adjacencies: &AdjacencyRules<D, IT>,
    ) -> Self;
}
