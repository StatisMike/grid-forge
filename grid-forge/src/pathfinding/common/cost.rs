pub trait CostType: std::fmt::Debug + Into<usize> + Copy {
    const LEN: usize;
}

#[derive(Debug, Clone, Copy)]
pub struct ConstCost {}

impl ConstCost {
    pub const DEFAULT_COST: u32 = 1;
}

impl CostType for ConstCost {
    const LEN: usize = 1;
}

#[allow(clippy::from_over_into)]
impl Into<usize> for ConstCost {
    fn into(self) -> usize {
        0
    }
}