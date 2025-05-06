pub(crate) mod three_d;
pub(crate) mod two_d;

pub(crate) mod common {
    use crate::core::common::*;

    pub trait TileContainer<D: Dimensionality> {
        fn grid_position(&self) -> D::Pos;
    }

    pub trait TileData: Sized {}

    pub trait Tile<D: Dimensionality, Data: TileData>:
        TileContainer<D> + AsRef<Data> + From<(D::Pos, Data)>
    {
        fn into_data(self) -> Data;
        fn data(&self) -> &Data;
    }

    pub trait TileRef<'a, D: Dimensionality, Data: TileData + 'a>:
        TileContainer<D> + AsRef<Data> + From<(D::Pos, &'a Data)> {
            fn data(&self) -> & Data;
        }

    pub trait TileMut<'a, D: Dimensionality, Data: TileData + 'a>:
        TileContainer<D> + AsRef<Data> + AsMut<Data> + From<(D::Pos, &'a mut Data)> {
            fn data(&mut self) -> &mut Data;
        }
}
