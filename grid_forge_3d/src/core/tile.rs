use super::GridPosition3D;

grid_forge_core::__impl_tile! {
    tile: Tile3D,
    tile_ref: TileRef3D,
    tile_mut: TileMut3D,
    tile_ref_shared: TileRefShared3D,
    tile_mut_shared: TileMutShared3D,
    position: GridPosition3D,
    container_trait: TileContainer3D,
}

pub trait TileContainer3D {
    fn grid_position(&self) -> GridPosition3D;
}

pub struct PosData3D<Data: TileData>(pub GridPosition3D, pub Data);

impl<Data: TileData> From<(GridPosition3D, Data)> for PosData3D<Data> {
    fn from(tuple: (GridPosition3D, Data)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<Data: TileData> TileContainer3D for PosData3D<Data> {
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

impl<Data: TileData> TileContainer3D for PosDataRef3D<'_, Data> {
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

impl<Data: TileData> TileContainer3D for PosDataMutRef3D<'_, Data> {
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