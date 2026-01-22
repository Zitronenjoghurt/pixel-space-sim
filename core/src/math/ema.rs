use std::ops::Deref;

const ALPHA: f64 = 0.1;

/// Exponential Moving Average
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct EMA(f64);

impl EMA {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn update(&mut self, value: f64) {
        if self.0 == 0.0 {
            self.0 = value;
        } else {
            self.0 = ALPHA * value + (1.0 - ALPHA) * self.0;
        }
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Deref for EMA {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
