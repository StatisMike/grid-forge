macro_rules! __impl_tile {
    (
        tile: $tile_type:ident,
        tile_ref: $tile_ref_type:ident,
        tile_mut: $tile_mut_type:ident,
        position: $position:ty,
    ) => {
        use crate::core::tile::common::TileData;

        pub struct $tile_type<Data: TileData>(pub $position, pub Data);

        impl<Data: TileData> $tile_type<Data> {
            pub fn new(grid_position: $position, data: Data) -> Self {
                Self(grid_position, data)
            }
            pub fn grid_position(&self) -> $position {
                self.0
            }
            pub fn into_data(self) -> Data {
                self.1
            }
            pub fn data(&self) -> &Data {
                &self.1
            }
        }

        impl<Data: TileData> From<($position, Data)> for $tile_type<Data> {
            fn from(tuple: ($position, Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        pub struct $tile_ref_type<'a, Data: TileData>(pub $position, pub &'a Data);

        impl<'a, Data: TileData> $tile_ref_type<'a, Data> {
            pub fn new(grid_position: $position, data: &'a Data) -> Self {
                Self(grid_position, data)
            }
            pub fn grid_position(&self) -> $position {
                self.0
            }
            pub fn data(&self) -> &Data {
                &self.1
            }
        }

        impl<'a, Data: TileData> From<($position, &'a Data)> for $tile_ref_type<'a, Data> {
            fn from(tuple: ($position, &'a Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        pub struct $tile_mut_type<'a, Data: TileData>(pub $position, pub &'a mut Data);

        impl<'a, Data: TileData> $tile_mut_type<'a, Data> {
            pub fn new(grid_position: $position, data: &'a mut Data) -> Self {
                Self(grid_position, data)
            }
            pub fn grid_position(&self) -> $position {
                self.0
            }
            pub fn data(&mut self) -> &mut Data {
                &mut self.1
            }
        }

        impl<'a, Data: TileData> From<($position, &'a mut Data)> for $tile_mut_type<'a, Data> {
            fn from(tuple: ($position, &'a mut Data)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }
    };
}

pub(crate) use __impl_tile;
