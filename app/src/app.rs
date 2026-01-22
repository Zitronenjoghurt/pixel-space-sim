use crate::camera::Camera;
use crate::gfx::Gfx;
use pss_core::math::screen_coords::ScreenCoords;
use std::sync::Arc;
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub struct App {
    camera: Camera,
    gfx: Gfx,
    cursor_pos: ScreenCoords,
    drag_start: Option<ScreenCoords>,
}

impl App {
    pub fn new(window: Arc<Window>) -> Self {
        Self {
            camera: Camera::new(),
            gfx: Gfx::new(window),
            cursor_pos: ScreenCoords::default(),
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
                        self.sync_buffer_size();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let new_pos = ScreenCoords::new(position.x as u32, position.y as u32);

                        if let Some(start) = self.drag_start {
                            let dx = start.x() as f32 - new_pos.x() as f32;
                            let dy = start.y() as f32 - new_pos.y() as f32;
                            self.camera.pan(dx, dy);
                            self.drag_start = Some(new_pos);
                        }

                        self.cursor_pos = new_pos;
                    }
                    WindowEvent::MouseInput { state, button, .. } if !egui_consumed => {
                        match (button, state) {
                            (MouseButton::Middle, ElementState::Pressed) => {
                                self.drag_start = Some(self.cursor_pos);
                            }
                            (MouseButton::Middle, ElementState::Released) => {
                                self.drag_start = None;
                            }
                            (MouseButton::Left, ElementState::Pressed) => {
                                self.on_click();
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } if !egui_consumed => {
                        let scroll = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y,
                            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                        };
                        let factor = 1.0 + scroll * 0.1;
                        self.camera
                            .zoom_at(self.cursor_pos, factor, self.screen_size());
                        self.sync_buffer_size();
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

    fn sync_buffer_size(&mut self) {
        let buffer = self.camera.buffer_size(self.screen_size());
        self.gfx.set_buffer_size(buffer.width(), buffer.height());
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        self.sync_buffer_size();

        let screen_size = self.screen_size();
        let buffer_size = self.buffer_size();

        {
            let frame = self.gfx.frame();
            frame.fill(0);
            Self::draw_world(frame, buffer_size, &self.camera);
        }

        let camera = &self.camera;
        let cursor_world = camera.screen_to_world(self.cursor_pos, screen_size);

        self.gfx.prepare_ui(|ctx| {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.label(format!(
                    "Camera: ({:.1}, {:.1})",
                    camera.pos.x(),
                    camera.pos.y()
                ));
                ui.label(format!("Zoom: {:.2}x", camera.zoom));
                ui.label(format!(
                    "Cursor: ({:.1}, {:.1})",
                    cursor_world.x(),
                    cursor_world.y()
                ));
                ui.label(format!(
                    "Buffer: {}x{}",
                    buffer_size.width(),
                    buffer_size.height()
                ));
            });
        });

        self.gfx.render();
    }

    fn draw_world(frame: &mut [u8], buffer_size: ScreenCoords, camera: &Camera) {
        if let Some(origin_buffer) = camera.world_to_buffer(
            pss_core::math::world_coords::WorldCoords::new(0.0, 0.0),
            buffer_size,
        ) {
            Self::draw_pixel(frame, origin_buffer, buffer_size, [255, 0, 0, 255]);
        }
    }

    fn draw_pixel(frame: &mut [u8], pos: ScreenCoords, size: ScreenCoords, color: [u8; 4]) {
        let idx = ((pos.y() * size.width() + pos.x()) * 4) as usize;
        frame[idx..idx + 4].copy_from_slice(&color);
    }

    fn on_click(&mut self) {
        let world_pos = self
            .camera
            .screen_to_world(self.cursor_pos, self.screen_size());
        println!("Clicked at world: ({}, {})", world_pos.x(), world_pos.y());
    }

    fn screen_size(&self) -> ScreenCoords {
        let size = self.gfx.window().inner_size();
        ScreenCoords::new(size.width, size.height)
    }

    fn buffer_size(&self) -> ScreenCoords {
        let [w, h] = self.gfx.buffer_size();
        ScreenCoords::new(w, h)
    }
}
