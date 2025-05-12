use crate::common::Dimensionality;

#[derive(Debug, Clone)]
pub struct RangeFinderPath<D: Dimensionality> {
    path: Vec<D::Pos>,
    cost: u32,
    last: bool,
}

impl<D: Dimensionality> RangeFinderPath<D> { 
    pub (crate) fn new(path: Vec<D::Pos>, cost: u32, last: bool) -> Self {
        Self { path, cost, last }
    }

    /// Checks if the given position is in the path (start and end point included)
    pub fn in_path(&self, pos: &D::Pos) -> bool {
        self.path.contains(pos)
    }

    /// Retrieves calculated cost to pass through the path.
    pub fn cost(&self) -> u32 {
        self.cost
    }

    /// If the point is the last one in its path.
    ///
    /// Being `last`, it means that there were at least one connection that couldn't be traversed outwards from
    /// its [to()](Self::to) position. Either because cost left weren't enough to traverse it, or because the
    /// connected position were not enabled.
    ///
    /// Useful when you need to know the 'boundaries' of the path, eg. if some other range should be applied to the
    /// border positions.
    pub fn last(&self) -> bool {
        self.last
    }

    /// Returns the whole part calculated (including start and end postions).
    pub fn path(&self) -> &Vec<D::Pos> {
        &self.path
    }

    /// Returns the movement steps calculated (excluding start point).
    pub fn steps(&self) -> &[D::Pos] {
        if self.path.len() == 1 {
            &[]
        } else {
            &self.path[1..]
        }
    }

    /// Returns the start position of the path.
    pub fn start(&self) -> &D::Pos {
        &self.path[0]
    }

    /// Returns the end position of the path.
    pub fn to(&self) -> &D::Pos {
        &self.path[self.path.len() - 1]
    }
}




#[derive(Default, Debug, Clone)]
pub struct AstarPath<D: Dimensionality> {
    path: Vec<D::Pos>,
    cost: u32,
    complete: bool,
}

impl<D: Dimensionality> AstarPath<D> {
    /// Returns the cost of the path.
    pub fn cost(&self) -> u32 {
        self.cost
    }

    /// Return false if the path is not complete (i.e. the end point wasn't reached).
    pub fn complete(&self) -> bool {
        self.complete
    }

    /// Returns the whole path calculated (including start and end postions)
    pub fn path(&self) -> &[D::Pos] {
        &self.path
    }

    /// Returns the movement steps calculated (excluding star point)
    pub fn steps(&self) -> &[D::Pos] {
        if self.path.len() == 1 {
            &[]
        } else {
            &self.path[1..]
        }
    }

    /// Returns the start position of the path
    pub fn start(&self) -> &D::Pos {
        &self.path[0]
    }

    /// Returns the end position of the path
    pub fn end(&self) -> &D::Pos {
        &self.path[self.path.len() - 1]
    }
}