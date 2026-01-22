use std::time::Duration;

pub struct SimulationSettings {
    pub tps: u8,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self { tps: 60 }
    }
}

impl SimulationSettings {
    pub fn duration_per_tick(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.tps as f32)
    }
}
