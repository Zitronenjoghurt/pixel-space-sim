use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod camera;
mod gfx;
mod ui;

pub use egui_phosphor::regular as icons;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new();

    event_loop.run_app(&mut app)?;
    Ok(())
}
