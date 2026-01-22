use std::ops::{Add, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

impl<N> Point<N> {
    #[inline(always)]
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

impl Sub for Point<f32> {
    type Output = Point<f32>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point<f32> {
    type Output = Point<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
