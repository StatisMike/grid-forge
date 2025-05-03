use std::{any::Any, fs::File, io::Write};

use crate::core::common::*;

/// When applied to the struct allows injecting it into [`singular::Resolver`](Resolver) to react on each tile being collapsed.
pub trait Subscriber<D: Dimensionality>: Any {
    /// Called when the generation process starts. No-op by default, should be overridden to clear the state of the subcscriber
    /// if it retains any state.
    fn on_generation_start(&mut self) {
        // no-op
    }

    /// Called when a tile is collapsed.
    fn on_collapse(&mut self, position: &D::Pos, tile_type_id: u64);

    /// To retrieve the concrete subscriber type from [`singular::Resolver`](Resolver).
    fn as_any(&self) -> &dyn Any;
}

/// Basic Subscriber for debugging purposes.
///
/// Implements both [`overlap::Subscriber`] and [`singular::Subscriber`], making it usable with both resolvers.
/// Upon collapsing a tile, it will print the collapsed `GridPosition`, `tile_type_id` and (if applicable) `pattern_id`.
#[derive(Debug, Default)]
pub struct DebugSubscriber {
    file: Option<File>,
}

impl DebugSubscriber {
    pub fn new(file: Option<File>) -> Self {
        Self { file }
    }
}

impl<D: Dimensionality> Subscriber<D> for DebugSubscriber {
    fn on_collapse(&mut self, position: &D::Pos, tile_type_id: u64) {
        if let Some(file) = &mut self.file {
            writeln!(
                file,
                "collapsed tile_type_id: {tile_type_id} on position: {position:?}"
            )
            .unwrap();
        } else {
            println!("collapsed tile_type_id: {tile_type_id} on position: {position:?}");
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Event in the history of tile generation process, containing the [`GridPosition`] of the tile alongside its collapsed
/// `tile_type_id`.
#[derive(Debug, Clone)]
pub struct CollapseHistoryItem<D: Dimensionality> {
    pub position: D::Pos,
    pub tile_type_id: u64,
}

/// Simple subscriber to collect history of tile generation process.
///
/// Every new generation began by the resolver will clear the history.
#[derive(Debug, Clone, Default)]
pub struct CollapseHistorySubscriber<D: Dimensionality> {
    history: Vec<CollapseHistoryItem<D>>,
}

impl<D: Dimensionality> CollapseHistorySubscriber<D> {
    /// Returns history of tile generation process.
    pub fn history(&self) -> &[CollapseHistoryItem<D>] {
        &self.history
    }
}

impl<D: Dimensionality> Subscriber<D> for CollapseHistorySubscriber<D> {
    fn on_generation_start(&mut self) {
        self.history.clear();
    }

    fn on_collapse(&mut self, position: &D::Pos, tile_type_id: u64) {
        self.history.push(CollapseHistoryItem {
            position: *position,
            tile_type_id,
        });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
