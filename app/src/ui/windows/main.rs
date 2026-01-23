use crate::ui::windows::debug::{DebugWindow, DebugWindowState};
use crate::ui::windows::draw::{DrawWindow, DrawWindowState};
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use crate::ui::AppContext;
use egui::{Id, Ui, WidgetText};

pub struct MainWindowState {
    pub is_open: bool,
    pub draw: DrawWindowState,
    debug: DebugWindowState,
}

impl Default for MainWindowState {
    fn default() -> Self {
        Self {
            is_open: true,
            draw: Default::default(),
            debug: Default::default(),
        }
    }
}

pub struct MainWindow<'a> {
    state: &'a mut MainWindowState,
    app_ctx: &'a AppContext<'a>,
}

impl<'a> MainWindow<'a> {
    pub fn new(state: &'a mut MainWindowState, app_ctx: &'a AppContext<'a>) -> Self {
        Self { state, app_ctx }
    }
}

impl UiWindow for MainWindow<'_> {
    fn id() -> Id {
        Id::new("main_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Main Window"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            DebugWindow::new(&mut self.state.debug, self.app_ctx)
                .toggle_button(ui)
                .show(ui.ctx());
            DrawWindow::new(&mut self.state.draw, self.app_ctx)
                .toggle_button(ui)
                .show(ui.ctx());
        });
    }
}
