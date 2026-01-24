use crate::math::area::Area;
use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::size::Size;
use crate::simulation::frame::SimFrame;
use crate::simulation::procedural::asteroid_shape::asteroid_shape_eclipse_radii;
use crate::simulation::state::SimState;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use state::settings;
use std::collections::{HashSet, VecDeque};
use std::ops::Add;
use std::time::Instant;
use sync::command::SimCommand;
use sync::{event, snapshot};

mod frame;
mod procedural;
pub mod source;
pub mod state;
pub mod sync;

pub struct Simulation {
    state: SimState,
    event_queue: VecDeque<event::SimEvent>,
    visible_rect: Rect<f32>,
    screen_size: Size<u32>,
    ticks: u64,
    alive: bool,
    paused: bool,
    debounce_visible: Option<Instant>,
    visible_asteroids: HashSet<Point<i64>>,
}

impl Simulation {
    pub fn new(state: SimState) -> Self {
        Self {
            state,
            event_queue: VecDeque::new(),
            visible_rect: Rect::default(),
            screen_size: Size::new(1, 1),
            ticks: 0,
            alive: true,
            paused: false,
            debounce_visible: None,
            visible_asteroids: Default::default(),
        }
    }

    pub fn tick(&mut self, force: bool) {
        if !force && self.paused {
            return;
        }
        self.ticks = self.ticks.wrapping_add(1);
    }

    pub fn update_frame(&mut self, frame: &mut SimFrame) {
        if frame.visible_rect() != self.visible_rect {
            frame.set_visible_rect(self.visible_rect);
            frame.resize_to_visible_rect();
            self.debounce_update_visible();
        }
        frame.clear();

        self.update_visible(false);

        for point in self.visible_asteroids.iter() {
            let Some(resource_type) = self.state.resource_type_at(*point) else {
                continue;
            };
            let Some(scale) = self.state.asteroid_scale_at(*point) else {
                continue;
            };
            let shape_seed = self.state.asteroid_shape_seed(*point);
            let (rx, ry) = asteroid_shape_eclipse_radii(shape_seed, scale);
            frame.fill_ellipse(point.to_f32(), rx, ry, resource_type.into());
        }

        self.update_snapshot(&mut frame.snapshot);
    }

    fn debounce_update_visible(&mut self) {
        if self.debounce_visible.is_none() {
            self.debounce_visible =
                Some(Instant::now().add(self.state.settings.visible_update_cooldown));
        }
    }

    fn update_visible(&mut self, force: bool) {
        if !force {
            let Some(debounce) = &self.debounce_visible else {
                return;
            };

            if Instant::now() < *debounce {
                return;
            }
        }

        self.visible_asteroids.clear();
        self.state.discovered_asteroids.keys().for_each(|point| {
            if self.visible_rect.contains(point.to_f32()) {
                self.visible_asteroids.insert(*point);
            }
        });
        self.debounce_visible = None;
    }

    fn update_snapshot(&self, snapshot: &mut snapshot::SimSnapshot) {
        snapshot.discovered_asteroids = self.state.discovered_asteroids.len();
        snapshot.settings = self.state.settings.clone();
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
            SimCommand::ScoutArea(area) => self.scout_area(area),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn settings(&self) -> &settings::SimulationSettings {
        &self.state.settings
    }
}

// World updates
impl Simulation {
    pub fn discover_asteroid(&mut self, point: Point<i64>) {
        self.state.discovered_asteroids.insert(point, 0.0);
        self.update_visible(true);
    }

    pub fn scout_area(&mut self, area: Area<f32>) {
        let points: Vec<_> = area.to_i64().iter().collect();

        let new_asteroids: Vec<_> = points
            .par_iter()
            .filter(|point| self.state.has_new_asteroid(**point))
            .copied()
            .collect();

        for point in new_asteroids {
            self.discover_asteroid(point);
        }
    }
}
