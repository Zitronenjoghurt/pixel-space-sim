use crate::math::point::Point;

pub struct Eclipse<N> {
    pub center: Point<N>,
    pub rx: N,
    pub ry: N,
}

impl<N> Eclipse<N> {
    pub fn new(center: Point<N>, rx: N, ry: N) -> Self {
        Self { center, rx, ry }
    }
}

impl Eclipse<f32> {
    pub fn to_i64(&self) -> Eclipse<i64> {
        Eclipse::new(
            self.center.to_i64(),
            self.rx.floor() as i64,
            self.ry.floor() as i64,
        )
    }
}

impl Eclipse<i64> {
    pub fn iter(&self) -> impl Iterator<Item = Point<i64>> + '_ {
        let min_x = self.center.x - self.rx;
        let max_x = self.center.x + self.rx;
        let min_y = self.center.y - self.ry;
        let max_y = self.center.y + self.ry;

        let rx_sq = (self.rx * self.rx) as f64;
        let ry_sq = (self.ry * self.ry) as f64;
        let cx = self.center.x as f64;
        let cy = self.center.y as f64;

        (min_x..=max_x).flat_map(move |x| {
            (min_y..=max_y).filter_map(move |y| {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;

                if (dx * dx) / rx_sq + (dy * dy) / ry_sq <= 1.0 {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
    }
}
