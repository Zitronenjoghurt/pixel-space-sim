use crate::simulation::command::SimCommand;
use crate::simulation::event::SimEvent;

pub mod local;

pub trait SimSource: Send {
    fn is_alive(&self) -> bool;
    fn send_command(&self, command: SimCommand);
    fn poll_event(&self) -> Option<SimEvent>;
    fn read_frame(&mut self, dest: &mut [u8]) -> Option<(u16, u16)>;
}
