use crate::math::area::Area;
use crate::math::rect::Rect;
use crate::math::size::Size;

pub enum SimCommand {
    Clear,
    Pause,
    Resume,
    TogglePause,
    Shutdown,
    SetVisibleRect(Rect<f32>),
    SetScreenSize(Size<u32>),
    ScoutArea(Area<f32>),
}
