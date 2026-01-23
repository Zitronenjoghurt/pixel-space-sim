use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub tps: u16,
    pub fps: u16,
    pub visible_update_cooldown: Duration,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            tps: 60,
            fps: 60,
            visible_update_cooldown: Duration::from_millis(100),
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
