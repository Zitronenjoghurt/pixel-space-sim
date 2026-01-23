use crate::math::circle::Circle;
use crate::math::point::Point;
use crate::math::rect::Rect;
use either::Either;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Area<N> {
    Circle(Circle<N>),
    Rect(Rect<N>),
}

impl Area<f32> {
    pub fn to_i64(&self) -> Area<i64> {
        match self {
            Area::Circle(circle) => Area::Circle(circle.to_i64()),
            Area::Rect(rect) => Area::Rect(rect.to_i64()),
        }
    }
}

impl Area<i64> {
    pub fn iter(&self) -> impl Iterator<Item = Point<i64>> {
        match self {
            Area::Circle(circle) => Either::Left(circle.iter()),
            Area::Rect(rect) => Either::Right(rect.iter()),
        }
    }
}
