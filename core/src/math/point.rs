use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

impl<N> Point<N> {
    #[inline]
    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

impl<N: Copy> Point<N> {
    #[inline]
    pub fn map<M>(self, f: impl Fn(N) -> M) -> Point<M> {
        Point::new(f(self.x), f(self.y))
    }
}

impl<N: Add<Output = N>> Add for Point<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<N: Sub<Output = N>> Sub for Point<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N: Mul<Output = N> + Copy> Mul<N> for Point<N> {
    type Output = Self;
    fn mul(self, s: N) -> Self {
        Point::new(self.x * s, self.y * s)
    }
}

impl<N: Div<Output = N> + Copy> Div<N> for Point<N> {
    type Output = Self;
    fn div(self, s: N) -> Self {
        Point::new(self.x / s, self.y / s)
    }
}

impl Point<u32> {
    pub fn to_f32(self) -> Point<f32> {
        self.map(|n| n as f32)
    }
}

impl Point<f32> {
    pub fn to_u32(self) -> Point<u32> {
        self.map(|n| n as u32)
    }

    pub fn round(self) -> Self {
        self.map(|n| n.round())
    }
}

impl Display for Point<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}
