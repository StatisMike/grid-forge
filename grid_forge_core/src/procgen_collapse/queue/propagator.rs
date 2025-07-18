use std::fmt::Debug;

#[derive(Debug)]
pub struct PropagateItem<T: Debug + Clone + Copy> {
    pub position: T,
    pub to_remove: usize,
}

impl<T: Debug + Clone + Copy> PropagateItem<T> {
    pub fn new(position: T, to_remove: usize) -> Self {
        Self {
            position,
            to_remove,
        }
    }
}