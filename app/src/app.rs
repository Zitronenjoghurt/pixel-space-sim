use crate::camera::Camera;
use crate::gfx::Gfx;
use crate::ui::{AppContext, Ui};
use pss_core::math::point::Point;
use pss_core::math::size::Size;
use pss_core::simulation::command::SimCommand;
use pss_core::simulation::settings::SimulationSettings;
use pss_core::simulation::snapshot::SimSnapshot;
use pss_core::simulation::source::local::LocalSim;
use pss_core::simulation::source::SimSource;
use std::sync::Arc;
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub struct App {
    camera: Camera,
    gfx: Gfx,
    ui: Ui,
    simulation: Option<Box<dyn SimSource>>,
    sim_snapshot: Option<SimSnapshot>,
    cursor_pos: Point<f32>,
    drag_start: Option<Point<f32>>,
}

impl App {
    pub fn new(window: Arc<Window>) -> Self {
        let settings = SimulationSettings::default_with_seed(2);
        let sim = LocalSim::spawn(settings);

        Self {
            camera: Camera::new(),
            gfx: Gfx::new(window),
            ui: Ui::default(),
            simulation: Some(Box::new(sim)),
            sim_snapshot: None,
            cursor_pos: Point::default(),
            drag_start: None,
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, target: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { event, .. } => {
                let egui_consumed = self.gfx.on_window_event(&event);

                match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::RedrawRequested => self.render(),
                    WindowEvent::Resized(size) => {
                        self.gfx.resize(size.width, size.height);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let new_pos = Point::new(position.x as f32, position.y as f32);

                        if let Some(start) = self.drag_start {
                            let delta = start - new_pos;
                            self.camera.pan(delta);
                            self.drag_start = Some(new_pos);
                        }

                        self.cursor_pos = new_pos;
                    }
                    WindowEvent::MouseInput { state, button, .. } if !egui_consumed => {
                        self.ui.on_mouse_input(state, button);
                        match (button, state) {
                            (MouseButton::Middle, ElementState::Pressed) => {
                                self.drag_start = Some(self.cursor_pos);
                            }
                            (MouseButton::Middle, ElementState::Released) => {
                                self.drag_start = None;
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } if !egui_consumed => {
                        let scroll = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y,
                            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 120.0,
                        };
                        let factor = 1.1_f32.powf(scroll);
                        self.camera
                            .zoom_at(self.cursor_pos, factor, self.screen_size());
                    }
                    WindowEvent::KeyboardInput { event, .. } if !egui_consumed => {
                        self.ui.on_keyboard_input(&event);
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                self.update();
                self.gfx.request_redraw();
            }
            _ => {}
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        let screen_size = self.screen_size();

        if let Some(sim) = &self.simulation {
            let rect = self.camera.visible_rect(screen_size);
            sim.send_command(SimCommand::SetVisibleRect(rect));
        }

        self.gfx.prepare_ui(|ctx| {
            let app_ctx = AppContext {
                simulation: self.simulation.as_deref(),
                sim_snapshot: self.sim_snapshot.as_ref(),
                camera: &self.camera,
                cursor_screen_pos: self.cursor_pos,
                screen_size,
            };
            self.ui.draw(ctx, &app_ctx);
        });

        if let Some(sim) = &mut self.simulation {
            let frame = sim.read_frame();

            let frame_rect = frame.visible_rect();
            let frame_size = frame.size();

            self.gfx
                .resize_cell_buffer(frame_size.width, frame_size.height);

            let cells_wide = frame_size.width as f32;
            let cells_high = frame_size.height as f32;

            let fract_x = frame_rect.min.x.rem_euclid(1.0);
            let fract_y = frame_rect.min.y.rem_euclid(1.0);

            let uv_offset = [fract_x / cells_wide, fract_y / cells_high];

            let uv_scale = [
                frame_rect.width() / cells_wide,
                frame_rect.height() / cells_high,
            ];

            self.gfx.set_camera(uv_offset, uv_scale);

            let dest = self.gfx.cell_buffer_mut();
            frame.write_rgba(dest);

            self.sim_snapshot = Some(frame.snapshot.clone());
        } else {
            self.gfx.cell_buffer_mut().fill(0);
        }

        self.gfx.render();
    }

    fn screen_size(&self) -> Size<u32> {
        let s = self.gfx.window().inner_size();
        Size::new(s.width, s.height)
    }
}
