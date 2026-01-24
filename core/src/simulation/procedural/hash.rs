use crate::math::point::Point;
use rapidhash::fast::RapidHasher;
use std::hash::Hasher;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct ProcHash(u64);

impl ProcHash {
    pub fn from_point_i64(seed: u64, point: Point<i64>, domain: ProcHashDomain) -> Self {
        let mut hasher = RapidHasher::new(seed);
        hasher.write_i64(point.x);
        hasher.write_i64(point.y);
        hasher.write_u64(domain as u64);
        Self(hasher.finish())
    }

    #[inline]
    /// Uniform distribution in [0, n]
    pub fn uniform_n(&self, n: u64) -> u64 {
        ((self.0 as u128 * n as u128) >> 64) as u64
    }

    #[inline]
    pub fn normalized(&self) -> f64 {
        (self.0 >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ProcHashDomain {
    AsteroidExists = 1,
    AsteroidResourceType = 2,
    AsteroidResourceAmount = 3,
    AsteroidShape = 4,
}
