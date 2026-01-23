use crate::math::point::Point;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Circle<N> {
    center: Point<N>,
    radius: N,
}

impl<N> Circle<N> {
    pub fn new(center: Point<N>, radius: N) -> Self {
        Self { center, radius }
    }
}

impl Circle<f32> {
    pub fn to_i64(&self) -> Circle<i64> {
        Circle::new(self.center.to_i64(), self.radius.floor() as i64)
    }
}

impl Circle<i64> {
    pub fn iter(&self) -> impl Iterator<Item = Point<i64>> {
        let center = self.center;
        let radius = self.radius;
        let r_squared = radius * radius;
        (center.x - radius..=center.x + radius).flat_map(move |x| {
            (center.y - radius..=center.y + radius)
                .map(move |y| Point::new(x, y))
                .filter(move |p| {
                    let dx = p.x - center.x;
                    let dy = p.y - center.y;
                    dx * dx + dy * dy <= r_squared
                })
        })
    }
}
