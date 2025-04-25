use crate::{dimensions::{Dimensionality, GridPositionTrait}, private};

pub mod identifiable;

pub mod two_dim {

    use crate::dimensions::two_dim::*; 
    use super::*;

    pub struct PosData2D<Data: TileData>(pub GridPosition2D, pub Data);

    impl <Data: TileData> From<(GridPosition2D, Data)> for PosData2D<Data> {
        fn from(tuple: (GridPosition2D, Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl <Data: TileData>TileContainer<TwoDim> for PosData2D<Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl <Data: TileData> AsRef<Data> for PosData2D<Data> {
        fn as_ref(&self) -> &Data {
            &self.1
        }
    }

    impl <Data: TileData> AsMut<Data> for PosData2D<Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }


    pub struct PosDataRef2D<'a, Data: TileData>(pub GridPosition2D, pub &'a Data);

    impl <'a, Data: TileData> From<(GridPosition2D, &'a Data)> for PosDataRef2D<'a, Data> {
        fn from(tuple: (GridPosition2D, &'a Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl <Data: TileData>TileContainer<TwoDim> for PosDataRef2D<'_, Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl <Data: TileData> AsRef<Data> for PosDataRef2D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    pub struct PosDataMutRef2D<'a, Data: TileData>(pub GridPosition2D, pub &'a mut Data);

    impl <'a, Data: TileData> From<(GridPosition2D, &'a mut Data)> for PosDataMutRef2D<'a, Data> {
        fn from(tuple: (GridPosition2D, &'a mut Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }
    
    impl <Data: TileData>TileContainer<TwoDim> for PosDataMutRef2D<'_, Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl <Data: TileData> AsRef<Data> for PosDataMutRef2D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    impl <Data: TileData> AsMut<Data> for PosDataMutRef2D<'_, Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }

}

pub mod three_dims {
    use crate::dimensions::three_dims::*;
    use crate::private;
    use super::{GridTile, TileData};

    pub struct GridTile3D<Data>
    where
        Data: TileData,
    {
        position: GridPosition3D,
        data: Data,
    }

    impl<Data: TileData> private::SealedContainer for GridTile3D<Data> {}

    impl<Data: TileData> GridTile<ThreeDim, Data> for GridTile3D<Data> {

        fn new(pos: GridPosition3D, data: Data) -> Self {
            Self { position: pos, data }
        }

        fn grid_position(&self) -> &GridPosition3D {
            &self.position
        }

        fn data(&self) -> &Data {
            &self.data
        }

        fn data_mut(&mut self) -> &mut Data {
            &mut self.data
        }

        fn into_data(self) -> Data {
            self.data
        }
    }

}

pub trait GridTile<D: Dimensionality, Data>: private::SealedContainer {

    fn new(pos: D::Pos, data: Data) -> Self;
    fn grid_position(&self) -> &D::Pos;
    fn data(&self) -> &Data;
    fn data_mut(&mut self) -> &mut Data;
    fn into_data(self) -> Data;
}

pub trait GridTileRef<'a, D: Dimensionality, Data: TileData>: private::SealedRef {

    fn new(pos: D::Pos, data: &'a Data) -> Self;
    fn grid_position(&self) -> D::Pos;
    fn data(&'a self) -> &'a Data;
}

pub trait GridTileRefMut<'a , D: Dimensionality, Data: TileData>: private::SealedRefMut {

    fn new(pos: D::Pos, data: &'a mut Data) -> Self;
    fn grid_position(&self) -> D::Pos;
    fn data(&'a self) -> &Data;
    fn data_mut(&'a mut self) -> &mut Data;
}





/// Marker trait for structs that can be contained withing [`GridMap2D`](crate::map::GridMap2D) and [`TileContainer`]
pub trait TileData: Sized {}


/// Trait gathering the containers for [`TileData`] outside of the [`GridMap2D`](crate::map::GridMap2D).
///
/// Allows accessing all the data not contained within tile data, making sense only in context of the grid map.
pub trait TileContainer<D: Dimensionality> {
    fn grid_position(&self) -> D::Pos;
}
