use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub tps: u16,
    pub fps: u16,
    pub visible_update_cooldown: Duration,
    pub max_asteroid_resource_amount: f32,
    pub max_asteroid_scale: f32,
    pub asteroid_density: f64,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            tps: 60,
            fps: 60,
            visible_update_cooldown: Duration::from_millis(100),
            max_asteroid_resource_amount: 1000.0,
            max_asteroid_scale: 10.0,
            asteroid_density: 0.00025,
        }
    }
}

impl SimulationSettings {
    pub fn interval_tps(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.tps as f64)
    }

    pub fn interval_fps(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.fps as f64)
    }
}
