use crate::simulation::command::SimCommand;
use crate::simulation::event::SimEvent;
use crate::simulation::frame_buffer::FrameBuffer;
use crate::simulation::Simulation;
use std::sync::mpsc;
use std::time::Instant;

pub struct LocalSimContext {
    pub simulation: Simulation,
    pub command_rx: mpsc::Receiver<SimCommand>,
    pub event_tx: mpsc::Sender<SimEvent>,
    pub frame_writer: triple_buffer::Input<FrameBuffer>,
}

impl LocalSimContext {
    pub fn run(mut self) {
        loop {
            let frame_start = Instant::now();

            while let Ok(cmd) = self.command_rx.try_recv() {
                self.simulation.handle_command(cmd);
            }

            if !self.simulation.is_alive() {
                break;
            }

            self.simulation.tick(false);
            self.render_frame();

            let target = self.simulation.settings.duration_per_tick();
            if let Some(remaining) = target.checked_sub(frame_start.elapsed()) {
                std::thread::sleep(remaining);
            }
        }
    }

    pub fn render_frame(&mut self) {
        let buf = self.frame_writer.input_buffer_mut();
        self.simulation.draw(buf);
        self.frame_writer.publish();
    }
}
