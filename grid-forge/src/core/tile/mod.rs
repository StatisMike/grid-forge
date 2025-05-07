pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use crate::core::common::*;

    pub trait TileContainer<D: Dimensionality> {
        fn grid_position(&self) -> D::Pos;
    }

    /// Marker trait for data kept inside [`GridMap`].
    /// 
    /// Many different types of the data can be stored in the [`GridMap`] - you can define your own struct
    /// to be kept inside the map, and then mark it with this trait.
    /// 
    /// There is also blanket implementation of this trait for `()` - if you would like to use the map only
    /// as indicators of which positions are taken and not keep any data.
    /// 
    /// Tile data inserted into the map can be retrieved as a direct reference, or as one of the
    /// container types: 
    /// - [`Tile`](crate::common::Tile) - holding owned data.
    /// - [`TileRef`](crate::common::TileRef) - holding reference to the data.
    /// - [`TileMut`](crate::common::TileMut) - holding mutable reference to the data.
    /// 
    /// # Example
    /// ```
    /// use grid_forge::common::{GridMap, TileData};
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

    /// Container for the data and the position of the tile.
    /// 
    /// This [`TileContainer`] owns the data - it can be constructed from tuple of the position and the data
    /// and inserted into the [`GridMap`] afterwards, or it will be retrieved from the map while removing the tile.
    pub trait Tile<D: Dimensionality, Data: TileData>:
        TileContainer<D> + AsRef<Data> + From<(D::Pos, Data)>
    {
        /// Consumes the container, retrieving underlying data.
        fn into_data(self) -> Data;

        /// Returns reference to the data.
        fn data(&self) -> &Data;
    }

    /// Container for the reference to the data and the position of the tile.
    /// 
    /// This [`TileContainer`] borrows the data - it can be retrieved from the map while keeping the tile
    /// intact.
    pub trait TileRef<'a, D: Dimensionality, Data: TileData + 'a>:
        TileContainer<D> + AsRef<Data> + From<(D::Pos, &'a Data)> {

            /// Returns reference to the data.
            fn data(&self) -> & Data;
        }

    /// Container for the mutable reference to the data and the position of the tile.
    /// 
    /// This [`TileContainer`] borrows the data - it can be retrieved from the map while keeping the tile
    /// intact, allowing for mutation of the data.
    pub trait TileMut<'a, D: Dimensionality, Data: TileData + 'a>:
        TileContainer<D> + AsRef<Data> + AsMut<Data> + From<(D::Pos, &'a mut Data)> {

            /// Returns mutable reference to the data.
            fn data(&mut self) -> &mut Data;
        }
}
