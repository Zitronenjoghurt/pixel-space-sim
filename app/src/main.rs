use std::sync::Arc;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod app;
mod camera;
mod gfx;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Pixel Space Sim")
            .build(&event_loop)
            .unwrap(),
    );

    let mut app = app::App::new(window.clone());

    event_loop
        .run(move |event, target| app.handle_event(event, target))
        .unwrap();
}
