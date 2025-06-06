pub(crate) mod macros;
pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    /// Marker trait for data kept inside grid map.
    ///
    /// Many different types of the data can be stored in the grid map - you can define your own struct
    /// to be kept inside the map, and then mark it with this trait.
    ///
    /// There is also blanket implementation of this trait for `()` - if you would like to use the map only
    /// as indicators of which positions are taken and not keep any data.
    ///
    /// Tile data inserted into the map can be retrieved as a direct reference, or as one of the
    /// [`TileContainer`] types. Each one is implemented for grids of specific dimensions.
    /// - `Tile` - holding owned data. For operations outside of the grid context.
    /// - `TileRef` - holding reference to the data.
    /// - `TileMut` - holding mutable reference to the data.
    ///
    /// # Example in 2D
    /// ```
    /// use grid_forge::two_d::{GridMap2D, GridPosition2D, GridSize2D};
    ///
    /// struct MyTileData {
    ///     foo: u32,
    ///     bar: String,
    /// }
    ///
    /// impl TileData for MyTileData {}
    ///
    /// let mut map = GridMap2D::<MyTileData>::new(GridSize2D::new(10, 10));
    /// map.insert_data(&GridPosition2D::new(0, 0), MyTileData { foo: 0, bar: String::from("foo") });
    /// assert!(map.get_data_at_position(&GridPosition2D::new(0, 0)).is_some());
    /// assert!(map.get_data_at_position(&GridPosition2D::new(1, 0)).is_none());
    /// ```
    pub trait TileData {}

    impl TileData for () {}

    pub trait TileContainer<P, D: TileData>: super::private::Sealed + AsRef<D> {
        fn grid_position(&self) -> P;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}
