use std::ops::{Add, Div, Mul, Sub};

/// Helper type used to compare floating point numbers.
///
/// Uses [OrderedFloat::EPSILON] as the epsilon for comparisons.
#[derive(Debug, Copy, Clone)]
pub struct OrderedFloat(f32);

impl OrderedFloat {
    pub const EPSILON: f32 = 0.00001;

    /// Create a new OrderedFloat, panicking on NaN
    pub fn new(value: f32) -> Self {
        assert!(!value.is_nan(), "Cannot create OrderedFloat from NaN");
        Self(value)
    }

    /// Get the inner float value
    pub fn get(self) -> f32 {
        self.0
    }
}

impl Eq for OrderedFloat {}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < Self::EPSILON
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get().eq(&other.get()) {
            return std::cmp::Ordering::Equal;
        }
        self.0
            .partial_cmp(&other.0)
            .expect("OrderedFloat should never contain NaN")
    }
}

impl Add for OrderedFloat {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.get() + other.get())
    }
}

impl Sub for OrderedFloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.get() - other.get())
    }
}

impl Mul for OrderedFloat {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.get() * other.get())
    }
}

impl Div for OrderedFloat {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.get() / other.get())
    }
}

impl From<f32> for OrderedFloat {
  fn from(value: f32) -> Self {
      Self::new(value)
  }
}

impl From<OrderedFloat> for f32 {
  fn from(value: OrderedFloat) -> f32 {
      value.get()
  }
}