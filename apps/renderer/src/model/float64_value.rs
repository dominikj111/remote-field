use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Float64Value {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

impl Float64Value {
    pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
        Float64Value {
            value,
            min,
            max,
            step,
        }
    }

    pub fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }

    pub fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.min);
    }

    pub fn get(&self) -> f64 {
        self.value
    }
}

impl PartialEq for Float64Value {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::fmt::Display for Float64Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
