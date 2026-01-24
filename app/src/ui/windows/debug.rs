use crate::icons;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use crate::ui::AppContext;
use egui::{Grid, Ui, WidgetText};

#[derive(Default)]
pub struct DebugWindowState {
    pub is_open: bool,
}

pub struct DebugWindow<'a> {
    state: &'a mut DebugWindowState,
    app_ctx: &'a AppContext<'a>,
}

impl<'a> DebugWindow<'a> {
    pub fn new(state: &'a mut DebugWindowState, app_ctx: &'a AppContext<'a>) -> Self {
        Self { state, app_ctx }
    }
}

impl UiWindow for DebugWindow<'_> {
    fn id() -> egui::Id {
        egui::Id::new("debug_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("debug_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Main Thread GFX Time");
                ui.label(format!("{:.2}ms", self.app_ctx.avg_gfx_secs.get() * 1000.0));
                ui.end_row();

                ui.label("Main Thread UI Time");
                ui.label(format!("{:.2}ms", self.app_ctx.avg_ui_secs.get() * 1000.0));
                ui.end_row();

                if let Some(snapshot) = self.app_ctx.sim_snapshot {
                    ui.label("Sim Frame Time");
                    ui.label(format!(
                        "{:.2}ms",
                        snapshot.avg_frame.as_secs_f32() * 1000.0
                    ));
                    ui.end_row();

                    ui.label("Sim Frame Time Budget");
                    ui.label(format!(
                        "{:.2}%",
                        snapshot.frame_time_per_second().as_secs_f32() * 100.0
                    ));
                    ui.end_row();

                    ui.label("Sim Tick Time");
                    ui.label(format!("{:.2}ms", snapshot.avg_tick.as_secs_f32() * 1000.0));
                    ui.end_row();

                    ui.label("Discovered Asteroids");
                    ui.label(format!("{}", snapshot.discovered_asteroids));
                    ui.end_row();
                }

                ui.label("Camera Center");
                ui.label(format!("{}", self.app_ctx.camera.center));
                ui.end_row();

                ui.label("Camera Zoom");
                ui.label(format!("{:.2}x", self.app_ctx.camera.zoom));
                ui.end_row();

                ui.label("Cursor (Screen)");
                ui.label(format!("{}", self.app_ctx.cursor_screen_pos));
                ui.end_row();

                ui.label("Cursor (World)");
                ui.label(format!("{}", self.app_ctx.cursor_world_pos()));
                ui.end_row();

                ui.label("Screen Size");
                ui.label(format!("{}", self.app_ctx.screen_size));
                ui.end_row();
            });
    }
}

impl ToggleableUiWindow for DebugWindow<'_> {
    fn toggle_label(&self) -> String {
        icons::BUG.into()
    }
}
