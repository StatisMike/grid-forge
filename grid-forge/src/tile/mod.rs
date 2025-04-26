use crate::{
    map::dimensions::{Dimensionality, GridPositionTrait},
    private,
};

pub mod identifiable;

pub mod two_dim {

    use super::*;
    use crate::map::dimensions::two_dim::*;

    pub struct PosData2D<Data: TileData>(pub GridPosition2D, pub Data);

    impl<Data: TileData> From<(GridPosition2D, Data)> for PosData2D<Data> {
        fn from(tuple: (GridPosition2D, Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<TwoDim> for PosData2D<Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosData2D<Data> {
        fn as_ref(&self) -> &Data {
            &self.1
        }
    }

    impl<Data: TileData> AsMut<Data> for PosData2D<Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }

    pub struct PosDataRef2D<'a, Data: TileData>(pub GridPosition2D, pub &'a Data);

    impl<'a, Data: TileData> From<(GridPosition2D, &'a Data)> for PosDataRef2D<'a, Data> {
        fn from(tuple: (GridPosition2D, &'a Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<TwoDim> for PosDataRef2D<'_, Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosDataRef2D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    pub struct PosDataMutRef2D<'a, Data: TileData>(pub GridPosition2D, pub &'a mut Data);

    impl<'a, Data: TileData> From<(GridPosition2D, &'a mut Data)> for PosDataMutRef2D<'a, Data> {
        fn from(tuple: (GridPosition2D, &'a mut Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<TwoDim> for PosDataMutRef2D<'_, Data> {
        fn grid_position(&self) -> GridPosition2D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosDataMutRef2D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    impl<Data: TileData> AsMut<Data> for PosDataMutRef2D<'_, Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }
}

pub mod three_dims {
    use super::{TileContainer, TileData};
    use crate::map::dimensions::three_dims::*;

    pub struct PosData3D<Data: TileData>(pub GridPosition3D, pub Data);

    impl<Data: TileData> From<(GridPosition3D, Data)> for PosData3D<Data> {
        fn from(tuple: (GridPosition3D, Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<ThreeDim> for PosData3D<Data> {
        fn grid_position(&self) -> GridPosition3D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosData3D<Data> {
        fn as_ref(&self) -> &Data {
            &self.1
        }
    }

    impl<Data: TileData> AsMut<Data> for PosData3D<Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }

    pub struct PosDataRef3D<'a, Data: TileData>(pub GridPosition3D, pub &'a Data);

    impl<'a, Data: TileData> From<(GridPosition3D, &'a Data)> for PosDataRef3D<'a, Data> {
        fn from(tuple: (GridPosition3D, &'a Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<ThreeDim> for PosDataRef3D<'_, Data> {
        fn grid_position(&self) -> GridPosition3D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosDataRef3D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    pub struct PosDataMutRef3D<'a, Data: TileData>(pub GridPosition3D, pub &'a mut Data);

    impl<'a, Data: TileData> From<(GridPosition3D, &'a mut Data)> for PosDataMutRef3D<'a, Data> {
        fn from(tuple: (GridPosition3D, &'a mut Data)) -> Self {
            Self(tuple.0, tuple.1)
        }
    }

    impl<Data: TileData> TileContainer<ThreeDim> for PosDataMutRef3D<'_, Data> {
        fn grid_position(&self) -> GridPosition3D {
            self.0
        }
    }

    impl<Data: TileData> AsRef<Data> for PosDataMutRef3D<'_, Data> {
        fn as_ref(&self) -> &Data {
            self.1
        }
    }

    impl<Data: TileData> AsMut<Data> for PosDataMutRef3D<'_, Data> {
        fn as_mut(&mut self) -> &mut Data {
            &mut self.1
        }
    }
}

/// Marker trait for structs that can be contained withing [`GridMap2D`](crate::map::GridMap2D) and [`TileContainer`]
pub trait TileData: Sized {}

/// Trait gathering the containers for [`TileData`] outside of the [`GridMap2D`](crate::map::GridMap2D).
///
/// Allows accessing all the data not contained within tile data, making sense only in context of the grid map.
pub trait TileContainer<D: Dimensionality> {
    fn grid_position(&self) -> D::Pos;
}
