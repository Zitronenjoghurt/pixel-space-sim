use crate::icons;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use crate::ui::AppContext;
use egui::{Ui, WidgetText};

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
        let cursor_world = self
            .app_ctx
            .camera
            .screen_to_world(self.app_ctx.cursor_pos, self.app_ctx.screen_size);

        ui.label(format!("Camera: {}", self.app_ctx.camera.center));
        ui.label(format!("Zoom: {:.2}x", self.app_ctx.camera.zoom));
        ui.label(format!("Cursor (Screen): {}", self.app_ctx.cursor_pos));
        ui.label(format!("Cursor (World): {cursor_world}",));
        ui.label(format!(
            "Buffer: {}x{}",
            self.app_ctx.buffer_size.width, self.app_ctx.buffer_size.height
        ));
    }
}

impl ToggleableUiWindow for DebugWindow<'_> {
    fn toggle_label(&self) -> String {
        icons::BUG.into()
    }
}
