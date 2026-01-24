use crate::math::point::Point;
use crate::simulation::procedural::hash::{ProcHash, ProcHashDomain};
use crate::simulation::state::colony::Colony;
use std::collections::{HashMap, HashSet};

pub mod colony;
pub mod resource;
mod resource_bag;
pub mod settings;

#[derive(Debug, Clone)]
pub struct SimState {
    seed: u64,
    pub settings: settings::SimulationSettings,
    pub discovered_asteroids: HashMap<Point<i64>, f32>,
    pub depleted_asteroids: HashSet<Point<i64>>,
    pub colonies: HashMap<Point<i64>, Colony>,
}

impl SimState {
    pub fn new(settings: settings::SimulationSettings, seed: u64) -> Self {
        Self {
            seed,
            settings,
            discovered_asteroids: Default::default(),
            depleted_asteroids: Default::default(),
            colonies: Default::default(),
        }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self::new(settings::SimulationSettings::default(), seed)
    }

    pub fn new_with_random_seed() -> Self {
        Self::new(settings::SimulationSettings::default(), rand::random())
    }
}

// Procedural generation
impl SimState {
    fn asteroid_exists(&self, point: Point<i64>) -> bool {
        let normal =
            ProcHash::from_point_i64(self.seed, point, ProcHashDomain::AsteroidExists).normalized();
        normal < self.settings.asteroid_density
    }

    fn asteroid_resource_type(&self, point: Point<i64>) -> resource::ResourceType {
        let uniform =
            ProcHash::from_point_i64(self.seed, point, ProcHashDomain::AsteroidResourceType)
                .uniform_n(100);
        match uniform {
            0..=60 => resource::ResourceType::Ice,
            61..=90 => resource::ResourceType::Iron,
            _ => resource::ResourceType::Gold,
        }
    }

    fn asteroid_initial_amount(&self, point: Point<i64>) -> f32 {
        let n = ProcHash::from_point_i64(self.seed, point, ProcHashDomain::AsteroidResourceAmount)
            .normalized() as f32;
        n * n * self.settings.max_asteroid_resource_amount
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
            self.asteroid_exists(point)
        }
    }

    pub fn resource_type_at(&self, point: Point<i64>) -> Option<resource::ResourceType> {
        if !self.has_asteroid_resources(point) {
            return None;
        }
        Some(self.asteroid_resource_type(point))
    }

    pub fn resource_amount_at(&self, point: Point<i64>) -> Option<f32> {
        let mined_amount = self.discovered_asteroids.get(&point)?;
        Some((self.asteroid_initial_amount(point) - mined_amount).max(0.0))
    }

    pub fn asteroid_scale_at(&self, point: Point<i64>) -> Option<f32> {
        let resource_amount = self.resource_amount_at(point)?;
        Some(
            resource_amount / self.settings.max_asteroid_resource_amount
                * self.settings.max_asteroid_scale,
        )
    }

    pub fn asteroid_shape_seed(&self, point: Point<i64>) -> u64 {
        ProcHash::from_point_i64(self.seed, point, ProcHashDomain::AsteroidShape).raw()
    }

    pub fn colony_at(&self, point: Point<i64>) -> Option<&Colony> {
        self.colonies.get(&point)
    }
}
