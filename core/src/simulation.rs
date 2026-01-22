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
        frame.set_visible_rect(self.visible_rect);
        frame.resize_to_visible_rect();
        frame.clear();

        let rect = self.visible_rect;
        let min_x = (rect.min.x / 10.0).floor() as i32 * 10;
        let max_x = (rect.max.x / 10.0).ceil() as i32 * 10;
        let min_y = (rect.min.y / 10.0).floor() as i32 * 10;
        let max_y = (rect.max.y / 10.0).ceil() as i32 * 10;

        let cells = (min_y..=max_y).step_by(10).flat_map(|y| {
            (min_x..=max_x).step_by(10).map(move |x| {
                let color = match (x.rem_euclid(20) == 0, y.rem_euclid(20) == 0) {
                    (true, true) => RGBA::white(),
                    (true, false) => RGBA::red(),
                    (false, true) => RGBA::blue(),
                    (false, false) => RGBA::green(),
                };
                (Point::new(x as f32, y as f32), color)
            })
        });

        frame.fill_cells(cells);
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
