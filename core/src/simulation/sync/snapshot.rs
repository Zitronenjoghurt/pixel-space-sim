use crate::simulation::state::settings::SimulationSettings;
use std::time::Duration;

#[derive(Default, Clone)]
pub struct SimSnapshot {
    pub discovered_asteroids: usize,
    pub settings: SimulationSettings,
    pub avg_frame: Duration,
    pub avg_tick: Duration,
}

impl SimSnapshot {
    pub fn frame_time_per_second(&self) -> Duration {
        Duration::from_secs_f64(self.avg_frame.as_secs_f64() * self.settings.fps as f64)
    }
}
