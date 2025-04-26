use crate::core::three_d::*;

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
