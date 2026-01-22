use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;
use crate::simulation::command::SimCommand;
use crate::simulation::frame::SimFrame;
use std::collections::VecDeque;

pub mod command;
pub mod event;
mod frame;
pub mod settings;
pub mod snapshot;
pub mod source;

pub struct Simulation {
    event_queue: VecDeque<event::SimEvent>,
    settings: settings::SimulationSettings,
    visible_rect: Rect<f32>,
    screen_size: Size<u32>,
    ticks: u64,
    alive: bool,
    paused: bool,
}

impl Simulation {
    pub fn from_settings(settings: settings::SimulationSettings) -> Self {
        Self {
            event_queue: VecDeque::new(),
            settings,
            visible_rect: Rect::default(),
            screen_size: Size::new(1, 1),
            ticks: 0,
            alive: true,
            paused: false,
        }
    }

    pub fn tick(&mut self, force: bool) {
        if !force && self.paused {
            return;
        }
        self.ticks = self.ticks.wrapping_add(1);
    }

    pub fn update_frame(&self, frame: &mut SimFrame) {
        frame.resize(self.screen_size);
        frame.set_visible_rect(self.visible_rect);
        frame.clear();

        frame.fill_cells([
            (Point::new(0.0, 0.0), RGBA::red()),
            (Point::new(1.0, 0.0), RGBA::blue()),
            (Point::new(-1.0, -1.0), RGBA::yellow()),
            (Point::new(0.0, 1.0), RGBA::green()),
        ]);

        self.update_snapshot(&mut frame.snapshot);
    }

    fn update_snapshot(&self, snapshot: &mut snapshot::SimSnapshot) {
        snapshot.settings = self.settings.clone();
    }

    pub fn handle_command(&mut self, command: SimCommand) {
        match command {
            SimCommand::Clear => {}
            SimCommand::Pause => self.paused = true,
            SimCommand::Resume => self.paused = false,
            SimCommand::TogglePause => self.paused = !self.paused,
            SimCommand::Shutdown => self.alive = false,
            SimCommand::SetVisibleRect(rect) => self.visible_rect = rect,
            SimCommand::SetScreenSize(size) => self.screen_size = size,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn settings(&self) -> &settings::SimulationSettings {
        &self.settings
    }
}
