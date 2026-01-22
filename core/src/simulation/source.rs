use crate::simulation::command::SimCommand;
use crate::simulation::event::SimEvent;
use crate::simulation::frame::SimFrame;

pub mod local;

pub trait SimSource: Send {
    fn is_alive(&self) -> bool;
    fn send_command(&self, command: SimCommand);
    fn poll_event(&self) -> Option<SimEvent>;
    fn read_frame(&mut self) -> &SimFrame;
}
