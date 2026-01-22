use crate::math::point::Point;
use crate::math::size::Size;
use std::ops::{Add, Div, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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

impl<N: Copy + Add<Output = N> + Sub<Output = N> + Div<Output = N>> Rect<N>
where
    N: From<u8>,
{
    pub fn size(&self) -> Size<N> {
        Size::new(self.width(), self.height())
    }

    pub fn center(&self) -> Point<N> {
        let two = N::from(2u8);
        Point::new(
            (self.min.x + self.max.x) / two,
            (self.min.y + self.max.y) / two,
        )
    }
}

impl Rect<f32> {
    pub fn from_center_size(center: Point<f32>, size: Size<f32>) -> Self {
        let half = Point::new(size.width / 2.0, size.height / 2.0);
        Self {
            min: center - half,
            max: center + half,
        }
    }

    pub fn contains(&self, p: Point<f32>) -> bool {
        p.x >= self.min.x && p.x < self.max.x && p.y >= self.min.y && p.y < self.max.y
    }

    pub fn intersect(&self, other: &Rect<f32>) -> Option<Rect<f32>> {
        let min_x = self.min.x.max(other.min.x);
        let min_y = self.min.y.max(other.min.y);
        let max_x = self.max.x.min(other.max.x);
        let max_y = self.max.y.min(other.max.y);

        if min_x < max_x && min_y < max_y {
            Some(Rect {
                min: Point::new(min_x, min_y),
                max: Point::new(max_x, max_y),
            })
        } else {
            None
        }
    }
}
