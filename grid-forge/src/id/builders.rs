//! [`IdentifiableTileData`] instance builders.
//!
//! Many methods in `grid_forge` needs to construct new instances of [`GridMap2D`](crate::map::GridMap2D) and fill them with
//! new instances of tiles by their `tile_id`. For them to be flexible, they need to use a builder struct, using some strategy
//! to create new tile instances.
//!
//! User can create their own [`IdentTileBuilder`]-implementing struct to use their own method of building new tiles, though
//! there are already some builders provided, using some basic strategies.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use super::*;
use crate::core::common::*;
// use crate::tile::{GridPosition, GridTile};

/// [`IdentTileBuilder`] which creates new tiles of [`Clone`]-implementing tile struct. Prototype of tile with each `tile_id` need to be
/// provided to the builder via [`add_tiles`](Self::add_tiles).
///
/// # Examples
/// ```
/// use grid_forge::two_d::*;
/// use grid_forge::id::*;
///
/// // Implementing custom TileData supported by `IdentTileCloneBuilder`.
/// #[derive(Clone)]
/// struct MyTileData {
///     tile_type_id: u64,
///     string: String
/// }
///
/// impl TileData for MyTileData {};
///
/// impl IdentifiableTileData for MyTileData {
///     fn tile_type_id(&self) -> u64 {
///         self.tile_type_id
///     }
/// }
///
/// let mut builder = IdentTileCloneBuilder::<TwoDim, MyTileData>::default();
/// let tiles = vec![
///     MyTileData { tile_type_id: 1, string: "First".to_string() },
///     MyTileData { tile_type_id: 2, string: "Second".to_string() },
/// ];
///
/// builder.add_tiles(&tiles, false);
///
/// if let Err(err) = builder.check_missing_ids(&[1,2,3]) {
///     assert_eq!(&[3], err.get_missing_tile_type_ids());
/// } else {
///     panic!("Should return error!");
/// }
///
/// let tile_1st: Tile2D<MyTileData> = builder.build_tile_unchecked(GridPosition2D::new(2,3), 1).into();
/// assert_eq!(
///     (
///         GridPosition2D::new(2,3), 
///         1, 
///         &"First".to_string()
///     ),
///     (
///         tile_1st.grid_position(), 
///         tile_1st.as_ref().tile_type_id, 
///         &tile_1st.as_ref().string
///     )
/// );
///
/// let tile_2nd: Tile2D<MyTileData> = builder.build_tile_unchecked(GridPosition2D::new(3,4), 2).into();
/// assert_eq!(
///     (
///         GridPosition2D::new(3,4), 
///         2, 
///         &"Second".to_string()
///     ),
///     (
///         tile_2nd.grid_position(), 
///         tile_2nd.as_ref().tile_type_id, 
///         &tile_2nd.as_ref().string
///     )
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IdentTileCloneBuilder<Data: IdentifiableTileData + Clone> {
    tiles: BTreeMap<u64, Data>,
}

impl<T: IdentifiableTileData + Clone> Default for IdentTileCloneBuilder<T> {
    fn default() -> Self {
        Self {
            tiles: BTreeMap::new(),
        }
    }
}

impl<Data: IdentifiableTileData + Clone> IdentTileCloneBuilder<Data> {
    /// Provide tile prototypes to the builder, which will be used to create new tile instances.
    ///
    /// If `overwrite` is `true`, then if prototype for given `tile_id` has been already saved, it will be overwritten.
    pub fn add_tiles(&mut self, tiles: &[Data], overwrite: bool) {
        for tile in tiles {
            if !overwrite && self.tiles.contains_key(&tile.tile_type_id()) {
                continue;
            }
            self.tiles.insert(tile.tile_type_id(), tile.clone());
        }
    }
}

impl<Data> IdentTileBuilder<Data> for IdentTileCloneBuilder<Data>
where
    Data: Clone + IdentifiableTileData,
{
    fn build_tile_unchecked(&self, tile_type_id: u64) -> Data {
        let tile_data = self
            .tiles
            .get(&tile_type_id)
            .unwrap_or_else(|| panic!("can't get tile_data with `tile_type_id`: {tile_type_id}"))
            .clone();

        tile_data
    }

    fn build_tile(
        &self,
        tile_type_id: u64,
    ) -> Result<Data, TileBuilderError> {
        if let Some(tile) = self.tiles.get(&tile_type_id) {
            let data = tile.clone();
            Ok(data)
        } else {
            Err(TileBuilderError::new(&[tile_type_id]))
        }
    }

    fn check_missing_ids(&self, tile_type_ids: &[u64]) -> Result<(), TileBuilderError> {
        let missing_ids = tile_type_ids
            .iter()
            .filter(|tile_id| !self.tiles.contains_key(tile_id))
            .copied()
            .collect::<Vec<_>>();

        if !missing_ids.is_empty() {
            Err(TileBuilderError::new(&missing_ids))
        } else {
            Ok(())
        }
    }
}

/// [`IdentTileBuilder`] which creates new tiles with given identifier based on the contructor functions provided to the
/// to the builder via [`set_tile_constructor`](Self::set_tile_constructor).
///
/// # Examples
/// ```
/// use grid_forge::{GridPosition, TileData, TileContainer};
/// use grid_forge::identifiable::IdentifiableTileData;
/// use grid_forge::identifiable::builders::{IdentTileBuilder, IdentTileFunBuilder};
///
/// // Implementing custom TileData supported by `IdentTileCloneBuilder`.
/// #[derive(Clone)]
/// struct MyTileData {
///     tile_type_id: u64,
///     traversible: bool
/// }
///
/// impl TileData for MyTileData {};
///
/// impl IdentifiableTileData for MyTileData {
///     fn tile_type_id(&self) -> u64 {
///         self.tile_type_id
///     }
/// }
///
/// let mut builder = IdentTileFunBuilder::<MyTileData>::default();
/// builder.set_tile_constructor(1, ||( MyTileData { tile_type_id: 1, traversible: true} ));
/// builder.set_tile_constructor(2, ||( MyTileData { tile_type_id: 2, traversible: false} ));
///
/// if let Err(err) = builder.check_missing_ids(&[1,2,3]) {
///     assert_eq!(&[3], err.get_missing_tile_type_ids());
/// } else {
///     panic!("Should return error!");
/// }
///
/// let tile_1st = builder.build_tile_unchecked(GridPosition::new_xy(2,3), 1);
/// assert_eq!((GridPosition::new_xy(2,3), 1, true), (tile_1st.grid_position(), tile_1st.as_ref().tile_type_id(), tile_1st.as_ref().traversible));
///
/// let tile_2nd = builder.build_tile_unchecked(GridPosition::new_xy(3,4), 2);
/// assert_eq!((GridPosition::new_xy(3,4), 2, false), (tile_2nd.grid_position(), tile_2nd.as_ref().tile_type_id(), tile_2nd.as_ref().traversible));
/// ```
#[derive(Debug, Clone)]
pub struct IdentTileFunBuilder<T: IdentifiableTileData> {
    funs: BTreeMap<u64, fn() -> T>,
}

impl<Data: IdentifiableTileData> IdentTileFunBuilder<Data> {
    pub fn set_tile_constructor(&mut self, tile_id: u64, constructor: fn() -> Data) {
        self.funs.insert(tile_id, constructor);
    }

    pub fn clear(&mut self) {
        self.funs.clear();
    }
}

impl<Data: IdentifiableTileData> Default for IdentTileFunBuilder<Data> {
    fn default() -> Self {
        Self {
            funs: BTreeMap::new(),
        }
    }
}

impl<Data: IdentifiableTileData> IdentTileBuilder<Data>
    for IdentTileFunBuilder<Data>
{
    fn build_tile_unchecked(&self, tile_type_id: u64) -> Data {
        let fun = self.funs.get(&tile_type_id).unwrap_or_else(|| {
            panic!("can't get tile constructor function for `tile_type_id`: {tile_type_id}")
        });

        fun()
    }

    fn build_tile(
        &self,
        tile_id: u64,
    ) -> Result<Data, TileBuilderError> {
        if let Some(fun) = self.funs.get(&tile_id) {
            Ok(fun())
        } else {
            Err(TileBuilderError::new(&[tile_id]))
        }
    }

    fn check_missing_ids(&self, tile_ids: &[u64]) -> Result<(), TileBuilderError> {
        let missing_ids = tile_ids
            .iter()
            .filter(|tile_id| !self.funs.contains_key(tile_id))
            .copied()
            .collect::<Vec<_>>();

        if !missing_ids.is_empty() {
            Err(TileBuilderError::new(&missing_ids))
        } else {
            Ok(())
        }
    }
}

/// Trait shared by objects which on basis of the grid position and tile identifier of given [`IdentifiableTileData`]-implementing struct can
/// create correct instance of the tile. Necessary for many [`GridMap2D`](crate::map::GridMap2D) creating methods.
///
/// Three different builders are available in the `grid_forge`:
/// - [`IdentTileFunBuilder`] - for tiles not implementing any additional traits.
/// - [`IdentTileCloneBuilder`] - for tiles implementing [`Clone`].
///
/// The logic for building tile is encapsulated in [`build_tile_unchecked`](IdentTileBuilder::build_tile_unchecked) and
/// [`build_tile`](IdentTileBuilder::build_tile) methods. The `unchecked` version is usually faster and is recommended
/// to be used when creating a batch of tiles at once - can be used reliably if [`check_missing_ids`](IdentTileBuilder::check_missing_ids)
/// is called before beginning the batch operation.
pub trait IdentTileBuilder<Data: IdentifiableTileData> {
    /// Creates tile data with given tile identifier at given grid position.
    ///
    /// # Panics
    /// Can panic if builder does not have possibility to construct tile of given `tile_id` based on the gathered information. You can check
    /// for missing tile ids with [`check_missing_ids`](IdentTileBuilder::check_missing_ids) or use its fallible version:
    /// [`build_tile`](IdentTileBuilder::build_tile).
    fn build_tile_unchecked(&self, tile_type_id: u64) -> Data;

    /// Creates tile with given tile identifier at given grid position. Returns error if cannot construct tile of given `tile_id`.
    fn build_tile(
        &self,
        tile_type_id: u64,
    ) -> Result<Data, TileBuilderError>;

    /// Checks for missing tile creators out of provided slice of `tile_id`.
    fn check_missing_ids(&self, tile_type_ids: &[u64]) -> Result<(), TileBuilderError>;
}

/// Error stemming from missing tiles in [`IdentTileBuilder`].
#[derive(Debug, Clone)]
pub struct TileBuilderError {
    tile_type_ids: Vec<u64>,
}

impl TileBuilderError {
    fn new(tile_ids: &[u64]) -> Self {
        Self {
            tile_type_ids: Vec::from(tile_ids),
        }
    }

    pub fn get_missing_tile_type_ids(&self) -> &[u64] {
        &self.tile_type_ids
    }
}

impl Display for TileBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "missing tile ids from builder: {missing_ids}",
            missing_ids = self
                .tile_type_ids
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Error for TileBuilderError {}
