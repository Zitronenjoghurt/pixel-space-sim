use crate::math::point::Point;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size<N> {
    pub width: N,
    pub height: N,
}

impl<N> Size<N> {
    #[inline]
    pub const fn new(width: N, height: N) -> Self {
        Self { width, height }
    }
}

impl<N: Copy> Size<N> {
    pub fn to_point(self) -> Point<N> {
        Point::new(self.width, self.height)
    }
}

impl Size<u32> {
    pub fn to_f32(self) -> Size<f32> {
        Size::new(self.width as f32, self.height as f32)
    }
}

impl Size<f32> {
    pub fn to_u32(self) -> Size<u32> {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl Display for Size<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}x{:.2}", self.width, self.height)
    }
}

impl Display for Size<u32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
