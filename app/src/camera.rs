use pss_core::math::point::Point;
use pss_core::math::rect::Rect;
use pss_core::math::size::Size;

pub struct Camera {
    pub center: Point<f32>,
    pub zoom: f32,
}

impl Camera {
    const MIN_BUFFER_DIM: u32 = 32;

    pub fn new() -> Self {
        Self {
            center: Point::default(),
            zoom: 1.0,
        }
    }

    pub fn buffer_size(&self, screen: Size<u32>) -> Size<u32> {
        let scaled = screen.to_f32().to_point() / self.zoom;
        Size::new(
            (scaled.x.round() as u32).max(Self::MIN_BUFFER_DIM),
            (scaled.y.round() as u32).max(Self::MIN_BUFFER_DIM),
        )
    }

    pub fn visible_rect(&self, screen: Size<u32>) -> Rect<f32> {
        let size = self.buffer_size(screen).to_f32();
        Rect::from_center_size(self.center, size)
    }

    pub fn screen_to_world(&self, screen_pos: Point<f32>, screen: Size<u32>) -> Point<f32> {
        let screen_center = screen.to_f32().to_point() / 2.0;
        let offset = (screen_pos - screen_center) / self.zoom;
        self.center + offset
    }

    pub fn pan(&mut self, delta: Point<f32>) {
        self.center = self.center + delta / self.zoom;
    }

    pub fn zoom_at(&mut self, screen_pos: Point<f32>, factor: f32, screen: Size<u32>) {
        let world_before = self.screen_to_world(screen_pos, screen);
        self.zoom = (self.zoom * factor).clamp(1.0, 100.0);
        let world_after = self.screen_to_world(screen_pos, screen);
        self.center = self.center + (world_before - world_after);
    }
}
