#[macro_export]
#[doc(hidden)]
macro_rules! __impl_tile {
    (
        tile: $tile_type:ident,
        tile_ref: $tile_ref_type:ident,
        tile_mut: $tile_mut_type:ident,
        tile_ref_shared: $tile_ref_shared_type:ident,
        tile_mut_shared: $tile_mut_shared_type:ident,
        position: $position:ty,
        container_trait: $container_trait:ident,
    ) => {
        use grid_forge_core::TileData;
        use grid_forge_core::id::{SharedData, WithSharedData};
        use private::Sealed;

        /// Container for the owned [`TileData`].
        ///
        /// For usage outside of the grid context - be it before inserting the data into
        /// the grid or after removing specific positions or draining the grid completely.
        ///
        /// Most of the time, you will want to use the reference types instead:
        #[doc = concat!("- [`", stringify!($tile_ref_type), "`]\n")]
        #[doc = concat!("- [`", stringify!($tile_mut_type), "`]\n")]
        pub struct $tile_type<Data: TileData>(pub $position, pub Data);

        impl<Data: TileData> $tile_type<Data> {
            /// Creates a new tile with the specified position and data.
            pub fn new(grid_position: $position, data: Data) -> Self {
                Self(grid_position, data)
            }

            /// Consumes the tile and returns the underlying data.
            pub fn into_data(self) -> Data {
                self.1
            }

            /// Returns a reference to the underlying data.
            pub fn data(&self) -> &Data {
                &self.1
            }
        }

        impl<Data: TileData> From<($position, Data)> for $tile_type<Data> {
            fn from(tuple: ($position, Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        impl<Data: TileData> AsRef<Data> for $tile_type<Data> {
            fn as_ref(&self) -> &Data {
                &self.1
            }
        }

        impl<Data: TileData> $container_trait for $tile_type<Data> {
            fn grid_position(&self) -> $position {
                self.0
            }
        }

        impl<Data: TileData> Sealed for $tile_type<Data> {}


        /// Container for the reference to the [`TileData`].
        ///
        /// Keeps the reference to the data as well as the position of the tile coupled,
        /// allowing to keep track of what the data references. Useful especially in scenarios
        /// where the data is retrieved from the grid in batches for further use.
        ///
        #[doc = concat!("For mutable operations, see [`", stringify!($tile_mut_type), "`].\n")]
        pub struct $tile_ref_type<'a, Data: TileData>(pub $position, pub &'a Data);

        impl<'a, Data: TileData> $tile_ref_type<'a, Data> {
            pub fn new(grid_position: $position, data: &'a Data) -> Self {
                Self(grid_position, data)
            }

            /// Returns a reference to the underlying data.
            pub fn data(&self) -> &Data {
                &self.1
            }
        }

        impl<'a, Data: TileData> From<($position, &'a Data)> for $tile_ref_type<'a, Data> {
            fn from(tuple: ($position, &'a Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        impl<'a, D: TileData> AsRef<D> for $tile_ref_type<'a, D> {
            fn as_ref(&self) -> &D {
                &self.1
            }
        }

        impl<'a , D: TileData> $container_trait for $tile_ref_type<'a, D> {
            fn grid_position(&self) -> $position {
                self.0
            }
        }

        impl<'a, D: TileData> Sealed for $tile_ref_type<'a, D> {}

        /// Container for the mutable reference to the [`TileData`].
        ///
        /// Keeps the reference to the data as well as the position of the tile coupled,
        /// allowing to keep track of what the data references.
        ///
        #[doc = concat!("For immutable operations, see [`", stringify!($tile_ref_type), "`].\n")]
        pub struct $tile_mut_type<'a, Data: TileData>(pub $position, pub &'a mut Data);

        impl<'a, Data: TileData> $tile_mut_type<'a, Data> {
            pub fn new(grid_position: $position, data: &'a mut Data) -> Self {
                Self(grid_position, data)
            }

            /// Returns a mutable reference to the underlying data.
            pub fn data(&mut self) -> &mut Data {
                &mut self.1
            }
        }

        impl<'a, Data: TileData> From<($position, &'a mut Data)> for $tile_mut_type<'a, Data> {
            fn from(tuple: ($position, &'a mut Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        impl<'a, D: TileData> AsRef<D> for $tile_mut_type<'a, D> {
            fn as_ref(&self) -> &D {
                &self.1
            }
        }

        impl<'a , D: TileData> $container_trait for $tile_mut_type<'a, D> {
            fn grid_position(&self) -> $position {
                self.0
            }
        }

        impl<'a , D: TileData> Sealed for $tile_mut_type<'a, D> {}

        /// Container for the reference to the [`TileData`] with [`SharedData`].
        ///
        /// Keeps the reference to the tile data and its type shared data, as well as the position of the tile.
        /// Can be retrieved from the grid types containing [`SharedData`].
        ///
        #[doc = concat!("For similiar structure with mutable reference to the [`TileData`] see: [`", stringify!($tile_mut_shared_type), "`].\n")]
        pub struct $tile_ref_shared_type<'a, Data: TileData, Shared: SharedData>(pub $position, pub &'a Data, pub &'a Shared);

        impl<'a, Data: TileData, Shared: SharedData> $tile_ref_shared_type<'a, Data, Shared> {
            pub(crate) fn new(grid_position: $position, data: &'a Data, shared: &'a Shared) -> Self {
                Self(grid_position, data, shared)
            }

            /// Returns a reference to the underlying data.
            pub fn data(&self) -> &Data {
                &self.1
            }
        }

        impl<'a, Data: TileData, Shared: SharedData> WithSharedData<Shared> for $tile_ref_shared_type<'a, Data, Shared> {
            fn shared_data(&self) -> &Shared {
                &self.2
            }
        }

        impl<'a, Data: TileData, Shared: SharedData> From<($tile_ref_type<'a, Data>, &'a Shared)> for $tile_ref_shared_type<'a, Data, Shared> {
            fn from(tuple: ($tile_ref_type<'a, Data>, &'a Shared)) -> Self {
                Self(tuple.0.0, tuple.0.1, tuple.1)
            }
        }

        impl<'a, D: TileData, Shared: SharedData> AsRef<D> for $tile_ref_shared_type<'a, D, Shared> {
            fn as_ref(&self) -> &D {
                &self.1
            }
        }

        impl<'a , D: TileData, Shared: SharedData> $container_trait for $tile_ref_shared_type<'a, D, Shared> {
            fn grid_position(&self) -> $position {
                self.0
            }
        }

        impl<'a, D: TileData, Shared: SharedData> Sealed for $tile_ref_shared_type<'a, D, Shared> {}

        /// Container for the mutable reference to the [`TileData`] with [`SharedData`].
        ///
        /// Keeps the reference to the tile data and its type shared data, as well as the position of the tile.
        /// Can be retrieved from the grid types containing [`SharedData`].
        ///
        /// It is the only way to retrieve and keept both mutable reference to the [`TileData`] and its type shared data
        /// at the same time.
        ///
        #[doc = concat!("For similiar structure with immutable reference to the [`TileData`] see: [`", stringify!($tile_ref_shared_type), "`].\n")]
        pub struct $tile_mut_shared_type<'a, Data: TileData, Shared: SharedData>(pub $position, pub &'a mut Data, pub &'a Shared);

        impl<'a, Data: TileData, Shared: SharedData> $tile_mut_shared_type<'a, Data, Shared> {
            pub(crate) fn new(grid_position: $position, data: &'a mut Data, shared: &'a Shared) -> Self {
                Self(grid_position, data, shared)
            }

            /// Returns a mutable reference to the underlying data.
            pub fn data(&mut self) -> &mut Data {
                &mut self.1
            }
        }

        impl<'a, Data: TileData, Shared: SharedData> WithSharedData<Shared> for $tile_mut_shared_type<'a, Data, Shared> {
            fn shared_data(&self) -> &Shared {
                &self.2
            }
        }

        impl<'a, Data: TileData, Shared: SharedData> From<($position, &'a mut Data, &'a Shared)> for $tile_mut_shared_type<'a, Data, Shared> {
            fn from(tuple: ($position, &'a mut Data, &'a Shared)) -> Self {
                Self(tuple.0, tuple.1, tuple.2)
            }
        }

        impl<'a, D: TileData, Shared: SharedData> AsRef<D> for $tile_mut_shared_type<'a, D, Shared> {
            fn as_ref(&self) -> &D {
                &self.1
            }
        }

        impl<'a , D: TileData, Shared: SharedData> $container_trait for $tile_mut_shared_type<'a, D, Shared> {
            fn grid_position(&self) -> $position {
                self.0
            }
        }

        impl<'a , D: TileData, Shared: SharedData> Sealed for $tile_mut_shared_type<'a, D, Shared> {}

        mod private {
            pub trait Sealed {}
        }
    };
}