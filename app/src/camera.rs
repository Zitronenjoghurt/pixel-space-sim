// camera.rs
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
        let min = Self::MIN_BUFFER_DIM as f32;
        let aspect = screen.width as f32 / screen.height as f32;

        let smaller_screen = screen.width.min(screen.height) as f32;
        let smaller_buffer = (smaller_screen / self.zoom).round().max(min);

        let (w, h) = if screen.width <= screen.height {
            let w = smaller_buffer;
            let h = (w / aspect).round();
            (w, h)
        } else {
            let h = smaller_buffer;
            let w = (h * aspect).round();
            (w, h)
        };

        Size::new(w as u32, h as u32)
    }

    pub fn visible_rect(&self, screen: Size<u32>) -> Rect<f32> {
        let size = self.buffer_size(screen).to_f32();
        Rect::from_center_size(self.center, size)
    }

    fn buffer_screen_transform(&self, screen: Size<u32>) -> (Point<f32>, f32) {
        let buffer = self.buffer_size(screen);

        let scale_x = screen.width as f32 / buffer.width as f32;
        let scale_y = screen.height as f32 / buffer.height as f32;
        let scale = scale_x.min(scale_y);

        let rendered_w = buffer.width as f32 * scale;
        let rendered_h = buffer.height as f32 * scale;

        let offset = Point::new(
            (screen.width as f32 - rendered_w) / 2.0,
            (screen.height as f32 - rendered_h) / 2.0,
        );

        (offset, scale)
    }

    pub fn screen_to_world(&self, screen_pos: Point<f32>, screen: Size<u32>) -> Point<f32> {
        let buffer = self.buffer_size(screen);
        let (offset, scale) = self.buffer_screen_transform(screen);

        let buffer_pos = (screen_pos - offset) / scale;
        let buffer_center = Point::new(buffer.width as f32 / 2.0, buffer.height as f32 / 2.0);

        self.center + (buffer_pos - buffer_center)
    }

    pub fn pan(&mut self, delta: Point<f32>) {
        self.center = self.center + delta / self.zoom;
    }

    pub fn max_zoom(&self, screen: Size<u32>) -> f32 {
        screen.width.min(screen.height) as f32 / Self::MIN_BUFFER_DIM as f32
    }

    pub fn zoom_at(&mut self, screen_pos: Point<f32>, factor: f32, screen: Size<u32>) {
        let world_before = self.screen_to_world(screen_pos, screen);
        self.zoom = (self.zoom * factor).clamp(1.0, self.max_zoom(screen));
        let world_after = self.screen_to_world(screen_pos, screen);
        self.center = self.center + (world_before - world_after);
    }
}
