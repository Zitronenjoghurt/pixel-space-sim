use crate::math::area::Area;
use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;
use crate::simulation::command::SimCommand;
use crate::simulation::frame::SimFrame;
use crate::simulation::procedural::hash::ProcHash;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::time::Instant;

pub mod command;
pub mod event;
mod frame;
mod procedural;
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
    discovered_asteroids: HashMap<Point<i64>, f32>,
    depleted_asteroids: HashSet<Point<i64>>,
    debounce_visible: Option<Instant>,
    visible_asteroids: HashSet<Point<i64>>,
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
            discovered_asteroids: Default::default(),
            depleted_asteroids: Default::default(),
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

        frame.fill_cells(
            self.visible_asteroids
                .iter()
                .map(|point| (point.to_f32(), RGBA::white())),
        );

        self.update_snapshot(&mut frame.snapshot);
    }

    fn debounce_update_visible(&mut self) {
        if self.debounce_visible.is_none() {
            self.debounce_visible = Some(Instant::now().add(self.settings.visible_update_cooldown));
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
        self.discovered_asteroids.keys().for_each(|point| {
            if self.visible_rect.contains(point.to_f32()) {
                self.visible_asteroids.insert(*point);
            }
        });
        self.debounce_visible = None;
    }

    fn update_snapshot(&self, snapshot: &mut snapshot::SimSnapshot) {
        snapshot.discovered_asteroids = self.discovered_asteroids.len();
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
        &self.settings
    }
}

/// World queries
impl Simulation {
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
            let normal = ProcHash::from_point_i64(self.settings.seed(), point).normalized();
            let resource_density = 0.001;
            normal < resource_density
        }
    }
}

/// World updates
impl Simulation {
    pub fn discover_asteroid(&mut self, point: Point<i64>) {
        self.discovered_asteroids.insert(point, 0.0);
        self.update_visible(true);
    }

    pub fn scout_area(&mut self, area: Area<f32>) {
        let points: Vec<_> = area.to_i64().iter().collect();

        let new_asteroids: Vec<_> = points
            .par_iter()
            .filter(|point| self.has_new_asteroid(**point))
            .copied()
            .collect();

        for point in new_asteroids {
            self.discover_asteroid(point);
        }
    }
}
