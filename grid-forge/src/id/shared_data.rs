use std::{collections::{HashMap, HashSet}};

/// Marker trait for data shared between tiles of the same type.
/// 
/// 
pub trait SharedData {}

pub trait WithSharedData<Shared: SharedData> {
    fn shared_data(&self) -> &Shared;
}

pub (crate) struct SharedDataContainer<Shared: SharedData> {
    inner: HashMap<u64, Shared>,
    pub(crate) mut_accessed: Option<HashSet<u64>>,
}

impl <Shared: SharedData> SharedDataContainer<Shared> {

    pub (crate) fn mut_access_tracking(&mut self, enabled: bool) {
        self.mut_accessed = if enabled {
            Some(HashSet::new())
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

    pub fn iter_shared_data(&self) -> impl Iterator<Item = (&u64, &Shared)> {
        self.inner.iter()
    }

    pub fn drain_shared_data(&mut self) -> Vec<(u64, Shared)> {
        self.inner.drain().collect()
    }
}

impl <Shared: SharedData> Default for SharedDataContainer<Shared> {
    fn default() -> Self {
        Self { inner: HashMap::new(), mut_accessed: None }
    }
}

