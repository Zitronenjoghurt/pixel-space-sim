use pss_core::math::point::Point;
use pss_core::math::rect::Rect;
use pss_core::math::size::Size;

pub struct Camera {
    pub center: Point<f32>,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: Point::default(),
            zoom: 1.0,
        }
    }

    pub fn visible_rect(&self, screen: Size<u32>) -> Rect<f32> {
        let size = Size::new(
            screen.width as f32 / self.zoom,
            screen.height as f32 / self.zoom,
        );
        Rect::from_center_size(self.center, size)
    }

    pub fn screen_to_world(&self, screen_pos: Point<f32>, screen: Size<u32>) -> Point<f32> {
        let visible = self.visible_rect(screen);
        Point::new(
            visible.min.x + screen_pos.x / self.zoom,
            visible.min.y + screen_pos.y / self.zoom,
        )
    }

    pub fn pan(&mut self, screen_delta: Point<f32>) {
        self.center.x += screen_delta.x / self.zoom;
        self.center.y += screen_delta.y / self.zoom;
    }

    pub fn zoom_at(&mut self, screen_pos: Point<f32>, factor: f32, screen: Size<u32>) {
        let world_before = self.screen_to_world(screen_pos, screen);
        self.zoom = (self.zoom * factor).clamp(0.5, 10000.0);
        let world_after = self.screen_to_world(screen_pos, screen);
        self.center = self.center + (world_before - world_after);
    }
}
