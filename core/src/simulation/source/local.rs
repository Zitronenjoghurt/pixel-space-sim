use crate::simulation::command::SimCommand;
use crate::simulation::event::SimEvent;
use crate::simulation::frame_buffer::FrameBuffer;
use crate::simulation::settings::SimulationSettings;
use crate::simulation::source::local::context::LocalSimContext;
use crate::simulation::source::SimSource;
use crate::simulation::Simulation;
use std::sync::mpsc;
use triple_buffer::TripleBuffer;

mod context;

pub struct LocalSim {
    command_tx: mpsc::Sender<SimCommand>,
    event_rx: mpsc::Receiver<SimEvent>,
    frame_reader: triple_buffer::Output<FrameBuffer>,
    _thread: std::thread::JoinHandle<()>,
}

impl LocalSim {
    pub fn spawn(settings: SimulationSettings) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let (frame_writer, frame_reader) = TripleBuffer::new(&FrameBuffer::default()).split();

        let thread = std::thread::spawn(move || {
            let context = LocalSimContext {
                simulation: Simulation::from_settings(settings),
                command_rx,
                event_tx,
                frame_writer,
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

    fn read_frame(&mut self, dest: &mut [u8]) -> Option<(u16, u16)> {
        let frame = self.frame_reader.read();
        let src = frame.pixels();
        let len = src.len().min(dest.len());
        dest[..len].copy_from_slice(&src[..len]);
        Some((frame.width, frame.height))
    }
}

impl Drop for LocalSim {
    fn drop(&mut self) {
        self.send_command(SimCommand::Shutdown);
    }
}
