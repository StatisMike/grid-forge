mod error;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::collections::HashSet;

pub use error::*;

use crate::core::common::{Dimensionality, Direction, GridMap, GridSize, TileData};

/// Struct implementing the random walker algorithm, producing the collection of [GridPositions](`Dimensionality::Pos`).
/// To be created with [`GridWalkerBuilder`].
///
/// [`GridWalker2DBuilder`].
pub struct GridWalker<D, R>
where
    D: Dimensionality,
    R: Rng,
{
    current_pos: D::Pos,
    walked: HashSet<D::Pos>,
    rng: R,
    dir_rng: Uniform<usize>,
    step_rng: Option<Uniform<usize>>,
    size: D::Size,
    step_size: usize,
    iters: u32,
}

impl<D, R> GridWalker<D, R>
where
    D: Dimensionality,
    R: Rng,
{
    /// Number of calls to the [Self::walk()] method.
    pub fn current_iters(&self) -> u32 {
        self.iters
    }

    pub fn walk(&mut self) -> bool {
        self.iters += 1;
        let idx: usize = self.dir_rng.sample(&mut self.rng);

        let step_size = if let Some(step_size_rng) = self.step_rng {
            step_size_rng.sample(&mut self.rng)
        } else {
            self.step_size
        };

        let mut current_pos = self.current_pos;
        let mut walked = Vec::new();

        for _ in 1..step_size {
            if let Some(pos) = D::Dir::all()[idx].march_step(&current_pos, &self.size) {
                current_pos = pos;
                walked.push(pos);
            } else {
                return false;
            }
        }

        self.current_pos = current_pos;
        for walked_pos in walked.iter() {
            self.walked.insert(*walked_pos);
        }
        true
    }

    pub fn walked(&self) -> &HashSet<D::Pos> {
        &self.walked
    }

    /// Generate [GridMap] out of gathered [GridPosition].
    ///
    /// # Arguments
    ///
    /// - `tile_fun` - function which will generate the [GridTile]-implementing objects with specified positions.
    pub fn gen_grid_map<Data, Grid>(&self, tile_fn: fn(D::Pos) -> Data) -> Grid
    where
        Data: TileData,
        Grid: GridMap<D, Data>,
    {
        let mut map = Grid::new(self.size);

        for pos in self.walked.iter() {
            map.insert_data(&pos, tile_fn(*pos));
        }
        map
    }

    pub fn set_current_pos(&mut self, current_pos: D::Pos) {
        self.current_pos = current_pos;
    }

    pub fn current_pos(&self) -> D::Pos {
        self.current_pos
    }

    pub fn reset(&mut self) {
        self.iters = 0;
        self.walked.clear();
    }
}

pub struct GridWalkerBuilder<D, R>
where
    D: Dimensionality,
    R: Rng,
{
    current_pos: Option<D::Pos>,
    rng: Option<R>,
    size: Option<D::Size>,
    min_step_size: usize,
    max_step_size: usize,
}

impl<D, R> Default for GridWalkerBuilder<D, R>
where
    D: Dimensionality,
    R: Rng,
{
    fn default() -> Self {
        Self {
            current_pos: None,
            rng: None,
            size: None,
            min_step_size: 1,
            max_step_size: 1,
        }
    }
}

impl<D, R> GridWalkerBuilder<D, R>
where
    D: Dimensionality,
    R: Rng,
{
    /// Set up starting position for the walker algorithm.
    pub fn with_current_pos(mut self, current_pos: D::Pos) -> Self {
        self.current_pos = Some(current_pos);
        self
    }

    /// Provide the [Rng] for random generation.
    pub fn with_rng(mut self, rng: R) -> Self {
        self.rng = Some(rng);
        self
    }

    /// Set up minimum step size: at every iteration the Walker will pick a [GridDir] and make `min..max` steps in that direction at random.
    pub fn with_min_step_size(mut self, min_step_size: usize) -> Self {
        self.min_step_size = min_step_size;
        self
    }

    /// Set up maximum step size: at every iteration the Walker will pick a [GridDir] and make `min..max` steps in that direction at random.
    pub fn with_max_step_size(mut self, max_step_size: usize) -> Self {
        self.max_step_size = max_step_size;
        self
    }

    /// Set up [GridSize] for walker to walk inside.
    pub fn with_size(mut self, size: D::Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> Result<GridWalker<D, R>, BuilderError> {
        let mut error = BuilderError::new();

        if self.size.is_none() {
            error.add_missing_field("size");
        }

        let current_pos = if let Some(pos) = self.current_pos {
            pos
        } else {
            self.size.unwrap().center()
        };

        if self.rng.is_none() {
            error.add_missing_field("rng");
        }

        error.try_throw()?;

        let dir_rng = rand::distributions::Uniform::new(0, D::Dir::N);
        let step_rng = self.get_step_rng();

        let mut walked = HashSet::new();
        walked.insert(current_pos);

        Ok(GridWalker {
            current_pos,
            walked,
            rng: self.rng.unwrap(),
            size: self.size.unwrap(),
            dir_rng,
            step_rng,
            step_size: self.min_step_size,
            iters: 0,
        })
    }

    fn get_step_rng(&self) -> Option<Uniform<usize>> {
        if self.min_step_size == self.max_step_size {
            return None;
        }

        Some(rand::distributions::Uniform::new(
            self.min_step_size,
            self.max_step_size + 1,
        ))
    }
}
