use crate::math::point::Point;

#[derive(Debug, Default, Copy, Clone)]
#[repr(transparent)]
pub struct ScreenCoords(Point<u32>);

impl ScreenCoords {
    #[inline(always)]
    pub fn new(x: u32, y: u32) -> Self {
        Self(Point::new(x, y))
    }

    #[inline(always)]
    pub fn x(&self) -> u32 {
        self.0.x
    }

    #[inline(always)]
    pub fn y(&self) -> u32 {
        self.0.y
    }

    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.x()
    }

    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.y()
    }
}
