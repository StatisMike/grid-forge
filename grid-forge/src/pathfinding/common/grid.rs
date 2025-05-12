use std::{cmp::Ordering, collections::{BTreeMap, BinaryHeap, HashMap}, error::Error, fmt::Display};

use crate::{common::{Dimensionality, GridMap, TileContainer, TileData, TileMut as _, TileRef as _}, utils::OrderedFloat};

use super::{cost::{self, CostType}, paths::{AstarPath, RangeFinderPath}, tile::PathfinderTile};

pub trait Pathfinder<D: Dimensionality, Cost: CostType, G: GridMap<D, PathfinderTile<D, Cost>>> { 
    fn _map(&self) -> &G;
    fn _map_mut(&mut self) -> &mut G;
    fn _size(&self) -> D::Size;
    fn _size_mut(&self) -> &mut D::Size;


    fn _add(&mut self, pos: D::Pos) -> Result<(), PathfinderError<D>> {
        let data = PathfinderTile::new();
        if let Some(slot) = self
            ._map_mut()
            .get_mut_slot(&pos) {
                return match slot {
                    Some(_) => {
                        Err(PathfinderError::PosConflict(pos))
                    },
                    None => {
                        slot.replace(data);
                        Ok(())
                    }
                }    
            };
        Err(PathfinderError::MissingPos(pos))
    }

    fn _set_cost(&mut self, pos: D::Pos, cost_type: Cost, cost: u32) -> Result<(), PathfinderError<D>> {
        if let Some(data) = self._map_mut().get_mut_data_at_position(&pos) {
            data.set_cost(cost_type, cost);
            return Ok(());
        };
        Err(PathfinderError::MissingPos(pos))
    }

    fn _get_cost(&self, cost_type: Cost, pos: D::Pos) -> Result<u32, PathfinderError<D>> {
        if let Some(data) = self._map().get_data_at_position(&pos) {
            return Ok(data.cost(cost_type));
        }
        Err(PathfinderError::MissingPos(pos))
    } 
    fn _add_with_costs(&mut self, pos: D::Pos, costs_setter: impl FnOnce(&mut PathfinderTile<D, Cost>)) -> Result<(), PathfinderError<D>> {
        if let Some(data) = self._map_mut().get_mut_data_at_position(&pos) {
            costs_setter(data);
            return Ok(());
        };
        Err(PathfinderError::MissingPos(pos))
    }
    fn _remove(&mut self, pos: D::Pos) -> Result<(), PathfinderError<D>> {
        if self._map_mut().remove_tile_at_position(&pos).is_none() {
            return Err(PathfinderError::MissingPos(pos));
        } 
        Ok(())
    }
    fn _enabled(&self, pos: D::Pos) -> Result<bool, PathfinderError<D>> {
        if let Some(data) = self._map().get_data_at_position(&pos) {
            return Ok(data.enabled());
        }
        Err(PathfinderError::MissingPos(pos))
    }
    fn _set_enabled(&mut self, pos: D::Pos, enabled: bool) -> Result<(), PathfinderError<D>> {
        if let Some(data) = self._map_mut().get_mut_data_at_position(&pos) {
            *data.enabled_mut() = enabled; 
            return Ok(());
        }
        Err(PathfinderError::MissingPos(pos))
    }
    fn _set_enabled_all(&mut self, enabled: bool) {
        for pos in self._map().get_all_positions() {
            if let Some(data) = self._map_mut().get_mut_data_at_position(&pos) {
                *data.enabled_mut() = enabled;
            }
        }
    }

    // Astar
    fn _astar_find_path(&self, start: D::Pos, end: D::Pos, cost_type: Cost) -> Result<AstarPath<D>, PathfinderError<D>>;
    fn _astar_find_path_limited(&self, start: D::Pos, end: D::Pos, cost_type: Cost, limit: u32) -> Result<AstarPath<D>, PathfinderError<D>>;

    // Range finder
    fn _range(&self, start: D::Pos, end: D::Pos, cost_type: Cost) -> Result<HashMap<D::Pos, RangeFinderPath<D>>, PathfinderError<D>>;

    // From GridMap
    fn _add_from_gridmap<M: GridMap<D, T>, T: TileData>(&mut self, grid_map: &M) -> Result<(), PathfinderError<D>> {
        for (pos, _) in grid_map.indexed_iter() {
            self._add(pos)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct AstarNode<D: Dimensionality> {
    pos: D::Pos,
    f: OrderedFloat,
    cost_remaining: Option<u32>,
}

impl <D: Dimensionality> AstarNode<D> {
    pub fn new(pos: D::Pos, f: f32, cost_remaining: Option<u32>) -> Self {
        Self {
            pos,
            f: OrderedFloat::new(f),
            cost_remaining,
        }
    }
}

impl <D: Dimensionality> Eq for AstarNode<D> {}

impl <D: Dimensionality> PartialEq for AstarNode<D> {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl <D: Dimensionality> PartialOrd for AstarNode<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <D: Dimensionality> Ord for AstarNode<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap, pos-based if equal f-scores for tie-breaker
        if self.f == other.f {
            other.pos.cmp(&self.pos)
        } else {
            other.f.cmp(&self.f)
        }
    }
}

pub (crate) struct RangeFinderNode<D: Dimensionality> {
    pos: D::Pos,
    cost_left: u32,
    from: Option<D::Pos>,
}

impl <D: Dimensionality> RangeFinderNode<D> {
    fn new(id: D::Pos, cost_left: u32, from: Option<D::Pos>) -> Self {
        Self {
            pos: id,
            cost_left,
            from,
        }
    }
}

impl <D: Dimensionality> Eq for RangeFinderNode<D> {}

impl <D: Dimensionality> PartialEq for RangeFinderNode<D> {
    fn eq(&self, other: &Self) -> bool {
        self.cost_left == other.cost_left
    }
}

impl <D: Dimensionality> PartialOrd for RangeFinderNode<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <D: Dimensionality> Ord for RangeFinderNode<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = other.cost_left.cmp(&self.cost_left);
        if order == Ordering::Equal {
            // If costs are equal, prefer the node with the smaller ID
            return other.pos.cmp(&self.pos);
        }
        order
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PathfinderError<D: Dimensionality> {
    MissingPos(D::Pos),
    PosConflict(D::Pos),
}

impl <D:Dimensionality> PathfinderError<D> {
    pub fn position(&self) -> D::Pos {
        match self {
            PathfinderError::MissingPos(pos) => *pos,
            PathfinderError::PosConflict(pos) => *pos,
        }
    }
}

impl <D:Dimensionality> Display for PathfinderError<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathfinderError::MissingPos(pos) => write!(f, "missing position: {pos:?}"),
            PathfinderError::PosConflict(pos) => write!(f, "position conflict, already added: {pos:?}"),
        }
    }
}

impl <D:Dimensionality> Error for PathfinderError<D> {}

pub (crate) fn rangefinder_process_node<D, Cost, G>(
    grid: &mut G, 
    node: RangeFinderNode<D>, 
    cost_type: Cost,
    generation: usize,
    open_set: &mut BinaryHeap<RangeFinderNode<D>>,
    closed_set: &mut BinaryHeap<D::Pos>,
)
where
    D: Dimensionality,
    Cost: CostType,
    G: GridMap<D, PathfinderTile<D, Cost>>,
{
    let neighbours = grid.get_neighbours(&node.pos);

    let mut last = neighbours.is_empty();

    for neigbour in neighbours {
        let cost = neigbour.data().cost(cost_type);

        // Point is not enabled or cannot moe to the point
        if neigbour.data().enabled() || cost > node.cost_left {
            last = true;
            continue;
        }

        // no better path, the one we have can remain
        if neigbour.data().cost_remaining(generation) <= node.cost_left {
            continue;
        }

        open_set.push(
            RangeFinderNode::new(neigbour.grid_position(), node.cost_left - cost, Some(node.pos))
        );
    }

    grid
        .get_mut_data_at_position(&node.pos)
        .expect(&format!("cannot get tile at position {:?}", node.pos))
        .range_update(generation, node.from, last);

    closed_set.push(node.pos);
}


pub (crate) fn rangefinder_push_path<D, Cost, G>(
    grid: &G,
    map: &mut BTreeMap<D::Pos, RangeFinderPath<D>>,
    generation: usize, 
    to: D::Pos,
    cost_type: Cost
) 
where
    D: Dimensionality,
    Cost: CostType,
    G: GridMap<D, PathfinderTile<D, Cost>>,
{
    let mut path = Vec::new();
    let mut current = to;
    let mut cost = 0;
    let mut end_pos = None;
    let mut last = None;
    loop {
        let point = grid.get_data_at_position(&current)
            .expect(&format!("couldn't get point at position {current:?} during path reconstruction")); 

        if last.is_none() {
            last = Some(point.last(generation));
        }

        if end_pos.is_none() {
            end_pos = Some(current);
        }

        if let Some(position) = point.previous(generation) {
            path.push(position);
            cost += point.cost(cost_type);
            current = position;
            continue;
        }
        path.push(current);
        break;
    }

    path.reverse();
    map.insert(
        end_pos.unwrap(),
        RangeFinderPath::new(path, cost, last.unwrap_or(false)),
    );
}



pub mod adv_traits {
    use crate::common::TileRef;

    use super::*;

    trait RangeFinder<D:Dimensionality, Cost: CostType, G: GridMap<D, PathfinderTile<D, Cost>>> {
        fn astar_process_node(&self, grid: &mut G, node: RangeFinderNode<D>, cost_type: Cost) {
            let tile = grid
                .get_data_at_position(&node.pos)
                .expect(&format!("cannot get tile at position {:?}", node.pos));
            let mut last = false;

            for neigbour in grid.get_neighbours(&node.pos) {
                let cost = neigbour.data().cost(cost_type);

            }
        }
    }
}
