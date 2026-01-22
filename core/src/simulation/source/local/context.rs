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
        let mut last_tick = Instant::now();
        let mut last_render = Instant::now();

        loop {
            let now = Instant::now();

            while let Ok(cmd) = self.command_rx.try_recv() {
                self.simulation.handle_command(cmd);
            }

            if !self.simulation.is_alive() {
                break;
            }

            let tick_interval = self.simulation.settings.interval_tps();
            while now.duration_since(last_tick) >= tick_interval {
                self.simulation.tick(false);
                last_tick += tick_interval;
            }

            let render_interval = self.simulation.settings.interval_fps();
            if now.duration_since(last_render) >= render_interval {
                self.render_frame();
                last_render = now;
            }

            let next_tick = last_tick + tick_interval;
            let next_render = last_render + render_interval;
            let next_event = next_tick.min(next_render);

            if let Some(wait) = next_event.checked_duration_since(Instant::now()) {
                spin_sleep::sleep(wait);
            }
        }
    }

    pub fn render_frame(&mut self) {
        let buf = self.frame_writer.input_buffer_mut();
        self.simulation.draw(buf);
        self.frame_writer.publish();
    }
}
