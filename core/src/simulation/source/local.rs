use crate::math::ema::EMA;
use crate::math::point::Point;
use crate::simulation::frame::SimFrame;
use crate::simulation::source::local::context::LocalSimContext;
use crate::simulation::source::SimSource;
use crate::simulation::state::colony::Colony;
use crate::simulation::state::SimState;
use crate::simulation::sync::command::SimCommand;
use crate::simulation::sync::event::SimEvent;
use crate::simulation::Simulation;
use std::sync::mpsc;
use triple_buffer::TripleBuffer;

mod context;

pub struct LocalSim {
    command_tx: mpsc::Sender<SimCommand>,
    event_rx: mpsc::Receiver<SimEvent>,
    frame_reader: triple_buffer::Output<SimFrame>,
    _thread: std::thread::JoinHandle<()>,
}

impl LocalSim {
    pub fn spawn(state: SimState) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let (frame_writer, frame_reader) = TripleBuffer::new(&SimFrame::default()).split();

        let thread = std::thread::spawn(move || {
            let mut sim = Simulation::new(state);
            sim.state
                .colonies
                .insert(Point::new(1000, 700), Colony::default());
            let context = LocalSimContext {
                simulation: sim,
                command_rx,
                event_tx,
                frame_writer,
                avg_frame_secs: EMA::default(),
                avg_tick_secs: EMA::default(),
            };
            context.run();
        });

        Self {
            command_tx,
            event_rx,
            frame_reader,
            _thread: thread,
        }
    }
}

impl SimSource for LocalSim {
    fn is_alive(&self) -> bool {
        !self._thread.is_finished()
    }

    fn send_command(&self, command: SimCommand) {
        let _ = self.command_tx.send(command);
    }

    fn poll_event(&self) -> Option<SimEvent> {
        self.event_rx.try_recv().ok()
    }

    fn read_frame(&mut self) -> &SimFrame {
        self.frame_reader.read()
    }
}

impl Drop for LocalSim {
    fn drop(&mut self) {
        self.send_command(SimCommand::Shutdown);
    }
}
