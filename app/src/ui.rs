use crate::camera::Camera;
use crate::ui::windows::main::{MainWindow, MainWindowState};
use crate::ui::windows::UiWindow;
use pss_core::math::ema::EMA;
use pss_core::math::point::Point;
use pss_core::math::size::Size;
use pss_core::simulation::source::SimSource;
use pss_core::simulation::sync::snapshot::SimSnapshot;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

mod widgets;
mod windows;

pub struct AppContext<'a> {
    pub simulation: Option<&'a dyn SimSource>,
    pub sim_snapshot: Option<&'a SimSnapshot>,
    pub camera: &'a Camera,
    pub cursor_screen_pos: Point<f32>,
    pub screen_size: Size<u32>,
    pub avg_gfx_secs: EMA,
    pub avg_ui_secs: EMA,
}

impl AppContext<'_> {
    pub fn cursor_world_pos(&self) -> Point<f32> {
        self.camera
            .screen_to_world(self.cursor_screen_pos, self.screen_size)
    }
}

#[derive(Default)]
pub struct Ui {
    main_window: MainWindowState,
    drawing: bool,
}

impl Ui {
    pub fn draw(&mut self, ctx: &egui::Context, app_ctx: &AppContext<'_>) {
        if self.drawing {
            self.main_window.draw.on_draw(app_ctx)
        }
        MainWindow::new(&mut self.main_window, app_ctx).show(ctx);
    }

    pub fn on_keyboard_input(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };

        match code {
            KeyCode::Escape => {
                if event.state == ElementState::Pressed {
                    self.main_window.is_open = !self.main_window.is_open
                }
            }
            _ => {}
        }
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if state == ElementState::Pressed && button == MouseButton::Left {
            self.drawing = true;
        } else if state == ElementState::Released && button == MouseButton::Left {
            self.drawing = false;
        }
    }
}
