use crate::math::point::Point;
use std::ops::Sub;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Rect<N> {
    pub min: Point<N>,
    pub max: Point<N>,
}

impl<N> Rect<N> {
    #[inline(always)]
    pub fn new(min: Point<N>, max: Point<N>) -> Self {
        Self { min, max }
    }
}

impl<N> Rect<N>
where
    N: Copy + Sub<Output = N>,
{
    pub fn width(&self) -> N {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> N {
        self.max.y - self.min.y
    }
}
