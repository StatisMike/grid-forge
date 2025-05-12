use core::f32;

use crate::common::TileData;

use crate::common::Dimensionality;
use crate::utils::OrderedFloat;

use super::cost::ConstCost;
use super::cost::CostType;

pub struct PathfinderTile<D: Dimensionality, Cost: CostType> {
    cost: Vec<u32>,
    enabled: bool,
    previous: Option<D::Pos>,
    is_last: bool,
    cost_remaining: u32,
    // Used in range finder
    computed_generation: usize,
    // used in A*
    open_pass: usize,
    closed_pass: usize,
    //
    g_score: OrderedFloat,
    _cost_type: std::marker::PhantomData<Cost>,
}

impl <C: CostType, D: Dimensionality> TileData for PathfinderTile<D, C> {}


impl <C: CostType, D: Dimensionality> PathfinderTile<D, C> {

    pub const DEFAULT_COST: u32 = 1;

    const MAX_COST: OrderedFloat = OrderedFloat::new(f32::MAX);

    pub (crate) fn new() -> Self {
        let mut cost = Vec::new();
        cost.resize(C::LEN, Self::DEFAULT_COST);
        Self {
            cost,
            enabled: true,
            previous: None,
            is_last: false,
            cost_remaining: 0,
            computed_generation: 0,
            open_pass: 0,
            closed_pass: 0,
            g_score: Self::MAX_COST,
            _cost_type: std::marker::PhantomData,
        }
    }

    pub (crate) fn cost(&self, cost_type: C) -> u32 {
        self.cost[cost_type.into()] 
    }

    pub (crate) fn cost_mut(&mut self, cost_type: C) -> &mut u32 {
        &mut self.cost[cost_type.into()]
    }

    pub fn set_cost(&mut self, cost_type: C, cost: u32) {
        self.cost[cost_type.into()] = cost;
    }

    pub (crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub (crate) fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }

    pub (crate) fn is_closed(&self, generation: usize) -> bool {
        self.closed_pass == generation
    }

    pub (crate) fn close(&mut self, generation: usize) {
        self.closed_pass = generation;
    }

    pub (crate) fn get_g_score(&self, generation: usize) -> OrderedFloat {
        if generation != self.open_pass {
            return Self::MAX_COST;
        }
        self.g_score
    }

    pub (crate) fn set_g_score(&mut self, generation: usize, score: OrderedFloat) {
        self.g_score = score;
        self.open_pass = generation;
    }

    pub (crate) fn previous(&self, generation: usize) -> Option<D::Pos> {
        if self.computed_generation == generation {
            return self.previous;
        }
        None
    }

    pub (crate) fn last(&self, generation: usize) -> bool {
        if self.computed_generation == generation {
            return self.is_last;
        }
        false
    }

    pub(crate) fn cost_remaining(&self, generation: usize) -> u32 {
        if self.computed_generation == generation {
            return self.cost_remaining;
        }
        0
    }

    pub (crate) fn range_update(&mut self, generation: usize, previous: Option<D::Pos>, is_last: bool) {
        self.computed_generation = generation;
        self.previous = previous;
        self.is_last = is_last;
    }
}
