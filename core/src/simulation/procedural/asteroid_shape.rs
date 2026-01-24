use crate::math::eclipse::Eclipse;
use crate::math::point::Point;

pub fn asteroid_shape_eclipse(seed: u64, center: Point<f32>, scale: f32) -> Eclipse<f32> {
    let rx = scale * (0.7 + (seed >> 56) as f32 * (0.3 / 255.0));
    let ry = scale * (0.7 + ((seed >> 48) & 0xFF) as f32 * (0.3 / 255.0));
    Eclipse::new(center, rx, ry)
}
