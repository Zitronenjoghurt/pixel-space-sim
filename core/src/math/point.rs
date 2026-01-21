#[derive(Debug, Default, Copy, Clone)]
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
