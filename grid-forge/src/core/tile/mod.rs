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
    }
}
