use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::size::Size;
use crate::simulation::command::SimCommand;
use crate::simulation::frame_buffer::FrameBuffer;
use std::collections::VecDeque;

pub mod command;
pub mod event;
mod frame_buffer;
pub mod settings;
pub mod source;

pub struct Simulation {
    event_queue: VecDeque<event::SimEvent>,
    settings: settings::SimulationSettings,
    visible_rect: Rect<f32>,
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

    pub fn draw(&self, buffer: &mut FrameBuffer) {
        let size = Size::new(
            self.visible_rect.width().max(1.0) as u16,
            self.visible_rect.height().max(1.0) as u16,
        );
        buffer.resize(size);
        buffer.visible_rect = self.visible_rect;
        buffer.clear([20, 20, 20, 255]);

        buffer.set_pixel(Point::new(0.0, 0.0), [255, 0, 0, 255]);
    }

    pub fn handle_command(&mut self, command: SimCommand) {
        match command {
            SimCommand::Clear => {}
            SimCommand::Pause => self.paused = true,
            SimCommand::Resume => self.paused = false,
            SimCommand::TogglePause => self.paused = !self.paused,
            SimCommand::Shutdown => self.alive = false,
            SimCommand::SetVisibleRect(rect) => self.visible_rect = rect,
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
