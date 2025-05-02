use crate::core::three_d::*;

pub struct Tile3D<Data: TileData>(pub GridPosition3D, pub Data);

impl <Data: TileData> Tile3D<Data> {
    pub fn new(grid_position: GridPosition3D, data: Data) -> Self {
        Self(grid_position, data)
    }
}

impl <Data: TileData> Tile<ThreeDim, Data> for Tile3D<Data> {
    fn into_data(self) -> Data {
        self.1
    }
}

impl<Data: TileData> From<(GridPosition3D, Data)> for Tile3D<Data> {
    fn from(tuple: (GridPosition3D, Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<ThreeDim> for Tile3D<Data> {
    fn grid_position(&self) -> GridPosition3D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for Tile3D<Data> {
    fn as_ref(&self) -> &Data {
        &self.1
    }
}

impl<Data: TileData> AsMut<Data> for Tile3D<Data> {
    fn as_mut(&mut self) -> &mut Data {
        &mut self.1
    }
}

pub struct TileRef3D<'a, Data: TileData>(pub GridPosition3D, pub &'a Data);

impl<'a, Data: TileData> From<(GridPosition3D, &'a Data)> for TileRef3D<'a, Data> {
    fn from(tuple: (GridPosition3D, &'a Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<ThreeDim> for TileRef3D<'_, Data> {
    fn grid_position(&self) -> GridPosition3D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for TileRef3D<'_, Data> {
    fn as_ref(&self) -> &Data {
        self.1
    }
}

pub struct TileMut3D<'a, Data: TileData>(pub GridPosition3D, pub &'a mut Data);

impl<'a, Data: TileData> From<(GridPosition3D, &'a mut Data)> for TileMut3D<'a, Data> {
    fn from(tuple: (GridPosition3D, &'a mut Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<ThreeDim> for TileMut3D<'_, Data> {
    fn grid_position(&self) -> GridPosition3D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for TileMut3D<'_, Data> {
    fn as_ref(&self) -> &Data {
        self.1
    }
}

impl<Data: TileData> AsMut<Data> for TileMut3D<'_, Data> {
    fn as_mut(&mut self) -> &mut Data {
        &mut self.1
    }
}
