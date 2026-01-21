use crate::math::point::Point;

#[derive(Debug, Default, Copy, Clone)]
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
