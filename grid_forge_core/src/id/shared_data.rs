use std::collections::{hash_map::Entry, HashMap, HashSet};

use super::{TileIdSet, TypeIdMap};

/// Marker trait for data shared between tiles of the same type.
///
/// With tiles implementing [`TypedData`](crate::id::TypedData) trait, you can use this trait
/// to store data shared between tiles of the same type. With this implemented, you can use
/// the [`TypedData`](crate::id::TypedData) to store a *state* of the tile, and the shared data
/// to store the immutable information about the tile.
pub trait SharedData {}

/// Marker trait for [`TileContainer`](crate::TileContainer) types containing
/// the [`TileData`](crate::TileData) and [`SharedData`](crate::id::SharedData) at the same time.
pub trait WithSharedData<Shared: SharedData> {
    fn shared_data(&self) -> &Shared;
}

#[doc(hidden)]
pub struct SharedDataContainer<Shared: SharedData> {
    inner: TypeIdMap<Shared>,
    pub mut_accessed: Option<TileIdSet>,
}

impl<Shared: SharedData> SharedDataContainer<Shared> {
    pub fn mut_access_tracking(&mut self, enabled: bool) {
        self.mut_accessed = if enabled {
            Some(TileIdSet::default())
        } else {
            None
        };
    }

    pub fn get_shared_data(&self, tile_type_id: &u64) -> Option<&Shared> {
        self.inner.get(tile_type_id)
    }

    pub fn get_shared_data_mut(&mut self, tile_type_id: &u64) -> Option<&mut Shared> {
        if let Some(mut_accessed) = &mut self.mut_accessed {
            mut_accessed.insert(*tile_type_id);
        }
        self.inner.get_mut(tile_type_id)
    }

    pub fn replace_shared_data(&mut self, tile_type_id: u64, data: Option<Shared>, track: bool) {
        if track {
            if let Some(mut_accessed) = &mut self.mut_accessed {
                mut_accessed.insert(tile_type_id);
            }
        }
        match data {
            Some(data) => self.inner.insert(tile_type_id, data),
            None => self.inner.remove(&tile_type_id),
        };
    }

    pub fn iter_shared_data(&self) -> impl Iterator<Item = (&u64, &Shared)> {
        self.inner.iter()
    }

    pub fn drain_shared_data(&mut self) -> TypeIdMap<Shared> {
        self.inner.drain().collect()
    }
}

impl<Shared: SharedData> Default for SharedDataContainer<Shared> {
    fn default() -> Self {
        Self {
            inner: TypeIdMap::default(),
            mut_accessed: None,
        }
    }
}
