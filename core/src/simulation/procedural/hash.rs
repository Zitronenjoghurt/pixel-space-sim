use crate::math::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct ProcHash(u64);

impl ProcHash {
    pub fn from_point_i64(seed: u64, point: Point<i64>) -> Self {
        let x = point.x as u64;
        let y = point.y as u64;

        let mut h = seed;
        h = h.wrapping_add(x.wrapping_mul(0x517cc1b727220a95));
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h = h.wrapping_add(y.wrapping_mul(0x61c8864680b583eb));
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
        h ^= h >> 33;
        Self(h)
    }

    pub fn normalized(&self) -> f64 {
        (self.0 >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}
