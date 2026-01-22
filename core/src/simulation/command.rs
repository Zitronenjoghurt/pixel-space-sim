use crate::math::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub enum SimCommand {
    Clear,
    Pause,
    Resume,
    TogglePause,
    Shutdown,
    SetVisibleRect(Rect<f32>),
}
