use crate::camera::Camera;
use crate::ui::windows::main::{MainWindow, MainWindowState};
use crate::ui::windows::UiWindow;
use pss_core::math::point::Point;
use pss_core::math::screen_coords::ScreenCoords;
use pss_core::simulation::source::SimSource;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

mod widgets;
mod windows;

pub struct AppContext<'a> {
    pub simulation: Option<&'a Box<dyn SimSource>>,
    pub camera: &'a Camera,
    pub cursor_pos: Point<f32>,
    pub buffer_size: ScreenCoords,
    pub screen_size: ScreenCoords,
}

#[derive(Default)]
pub struct Ui {
    main_window: MainWindowState,
}

impl Ui {
    pub fn draw(&mut self, ctx: &egui::Context, app_ctx: &AppContext<'_>) {
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
}
