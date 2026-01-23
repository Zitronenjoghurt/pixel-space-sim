use crate::math::point::Point;
use crate::simulation::procedural::hash::ProcHash;
use crate::simulation::settings;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SimState {
    seed: u64,
    pub settings: settings::SimulationSettings,
    pub discovered_asteroids: HashMap<Point<i64>, f32>,
    pub depleted_asteroids: HashSet<Point<i64>>,
}

impl SimState {
    pub fn new(settings: settings::SimulationSettings, seed: u64) -> Self {
        Self {
            seed,
            settings,
            discovered_asteroids: Default::default(),
            depleted_asteroids: Default::default(),
        }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self::new(settings::SimulationSettings::default(), seed)
    }

    pub fn new_with_random_seed() -> Self {
        Self::new(settings::SimulationSettings::default(), rand::random())
    }
}

// World queries
impl SimState {
    pub fn has_asteroid_resources(&self, point: Point<i64>) -> bool {
        self.discovered_asteroids.contains_key(&point)
    }

    pub fn has_asteroid_depleted(&self, point: Point<i64>) -> bool {
        self.depleted_asteroids.contains(&point)
    }

    pub fn has_new_asteroid(&self, point: Point<i64>) -> bool {
        if self.has_asteroid_resources(point) || self.has_asteroid_depleted(point) {
            false
        } else {
            let normal = ProcHash::from_point_i64(self.seed, point).normalized();
            let resource_density = 0.001;
            normal < resource_density
        }
    }
}
