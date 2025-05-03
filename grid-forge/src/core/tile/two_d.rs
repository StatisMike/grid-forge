use crate::core::two_d::*;

pub struct Tile2D<Data: TileData>(pub GridPosition2D, pub Data);

impl<Data: TileData> Tile2D<Data> {
    pub fn new(grid_position: GridPosition2D, data: Data) -> Self {
        Self(grid_position, data)
    }
}

impl<Data: TileData> Tile<TwoDim, Data> for Tile2D<Data> {
    fn into_data(self) -> Data {
        self.1
    }
}

impl<Data: TileData> From<(GridPosition2D, Data)> for Tile2D<Data> {
    fn from(tuple: (GridPosition2D, Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<TwoDim> for Tile2D<Data> {
    fn grid_position(&self) -> GridPosition2D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for Tile2D<Data> {
    fn as_ref(&self) -> &Data {
        &self.1
    }
}

impl<Data: TileData> AsMut<Data> for Tile2D<Data> {
    fn as_mut(&mut self) -> &mut Data {
        &mut self.1
    }
}

pub struct TileRef2D<'a, Data: TileData>(pub GridPosition2D, pub &'a Data);

impl<'a, Data: TileData> TileRef2D<'a, Data> {
    pub fn new(grid_position: GridPosition2D, data: &'a Data) -> Self {
        Self(grid_position, data)
    }
}

impl<'a, Data: TileData> From<(GridPosition2D, &'a Data)> for TileRef2D<'a, Data> {
    fn from(tuple: (GridPosition2D, &'a Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<TwoDim> for TileRef2D<'_, Data> {
    fn grid_position(&self) -> GridPosition2D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for TileRef2D<'_, Data> {
    fn as_ref(&self) -> &Data {
        self.1
    }
}

pub struct TileMut2D<'a, Data: TileData>(pub GridPosition2D, pub &'a mut Data);

impl<'a, Data: TileData> TileMut2D<'a, Data> {
    pub fn new(grid_position: GridPosition2D, data: &'a mut Data) -> Self {
        Self(grid_position, data)
    }
}

impl<'a, Data: TileData> From<(GridPosition2D, &'a mut Data)> for TileMut2D<'a, Data> {
    fn from(tuple: (GridPosition2D, &'a mut Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer<TwoDim> for TileMut2D<'_, Data> {
    fn grid_position(&self) -> GridPosition2D {
        self.0
    }
}

impl<Data: TileData> AsRef<Data> for TileMut2D<'_, Data> {
    fn as_ref(&self) -> &Data {
        self.1
    }
}

impl<Data: TileData> AsMut<Data> for TileMut2D<'_, Data> {
    fn as_mut(&mut self) -> &mut Data {
        &mut self.1
    }
}
