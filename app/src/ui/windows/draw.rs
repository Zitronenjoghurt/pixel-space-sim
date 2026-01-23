use crate::icons;
use crate::ui::widgets::enum_select::EnumSelect;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use crate::ui::AppContext;
use egui::{Id, Ui, Widget, WidgetText};
use pss_core::math::area::Area;
use pss_core::math::circle::Circle;
use pss_core::math::point::Point;
use pss_core::math::rect::Rect;
use pss_core::simulation::command::SimCommand;
use std::fmt::Display;
use strum_macros::EnumIter;

pub struct DrawWindow<'a> {
    pub state: &'a mut DrawWindowState,
    pub app_ctx: &'a AppContext<'a>,
}

impl<'a> DrawWindow<'a> {
    pub fn new(state: &'a mut DrawWindowState, app_ctx: &'a AppContext<'a>) -> Self {
        Self { state, app_ctx }
    }
}

impl UiWindow for DrawWindow<'_> {
    fn id() -> Id {
        Id::new("draw_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Drawing"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Draw mode");
            EnumSelect::new(&mut self.state.mode, "draw_window_draw_mode_select").ui(ui);
        });

        ui.separator();

        if matches!(self.state.mode, DrawMode::Scout) {
            ui.horizontal(|ui| {
                ui.label("Area Type");
                EnumSelect::new(&mut self.state.area_type, "draw_window_area_type_select").ui(ui);
            });
        }

        if matches!(self.state.mode, DrawMode::Scout) {
            match self.state.area_type {
                DrawAreaType::Circle => {
                    ui.horizontal(|ui| {
                        ui.label("Diameter");
                    });
                }
                DrawAreaType::Square => {
                    ui.horizontal(|ui| {
                        ui.label("Size");
                    });
                }
            }
            egui::Slider::new(&mut self.state.size, 1..=100).ui(ui);
        }
    }
}

impl ToggleableUiWindow for DrawWindow<'_> {
    fn toggle_label(&self) -> String {
        icons::PALETTE.into()
    }
}

#[derive(Debug, Default)]
pub struct DrawWindowState {
    pub is_open: bool,
    pub mode: DrawMode,
    pub area_type: DrawAreaType,
    pub size: u8,
}

impl DrawWindowState {
    pub fn on_draw(&self, app_ctx: &AppContext<'_>) {
        let Some(sim) = app_ctx.simulation else {
            return;
        };

        match self.mode {
            DrawMode::Scout => sim.send_command(SimCommand::ScoutArea(
                self.get_area(app_ctx.cursor_world_pos()),
            )),
        }
    }

    fn get_circle(&self, point: Point<f32>) -> Circle<f32> {
        Circle::new(point, self.size as f32 * 2.0)
    }

    fn get_square(&self, point: Point<f32>) -> Rect<f32> {
        Rect::new_square(point, self.size as f32)
    }

    fn get_area(&self, point: Point<f32>) -> Area<f32> {
        match self.area_type {
            DrawAreaType::Circle => Area::Circle(self.get_circle(point)),
            DrawAreaType::Square => Area::Rect(self.get_square(point)),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, EnumIter)]
pub enum DrawMode {
    #[default]
    Scout,
}

impl Display for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawMode::Scout => write!(f, "Scout"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, EnumIter)]
pub enum DrawAreaType {
    #[default]
    Circle,
    Square,
}

impl Display for DrawAreaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawAreaType::Circle => write!(f, "Circle"),
            DrawAreaType::Square => write!(f, "Square"),
        }
    }
}
