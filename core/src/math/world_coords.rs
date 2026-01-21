use crate::math::point::Point;

#[derive(Debug, Default, Copy, Clone)]
#[repr(transparent)]
pub struct WorldCoords(Point<f32>);

impl WorldCoords {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self {
        Self(Point::new(x, y))
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0.x
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0.y
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.x()
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.y()
    }
}
