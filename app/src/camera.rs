use pss_core::math::point::Point;
use pss_core::math::rect::Rect;
use pss_core::math::screen_coords::ScreenCoords;
use pss_core::math::world_coords::WorldCoords;

pub struct Camera {
    pub pos: WorldCoords,
    pub zoom: f32,
}

impl Camera {
    const MIN_BUFFER_DIM: u32 = 32;

    pub fn new() -> Self {
        Self {
            pos: WorldCoords::default(),
            zoom: 1.0,
        }
    }

    pub fn buffer_size(&self, screen_size: ScreenCoords) -> ScreenCoords {
        let w = (screen_size.width() as f32 / self.zoom)
            .round()
            .max(Self::MIN_BUFFER_DIM as f32) as u32;
        let h = (screen_size.height() as f32 / self.zoom)
            .round()
            .max(Self::MIN_BUFFER_DIM as f32) as u32;
        ScreenCoords::new(w, h)
    }

    pub fn visible_rect(&self, screen_size: ScreenCoords) -> Rect<f32> {
        let size = self.buffer_size(screen_size);
        let half = Point::new(size.width() as f32 / 2.0, size.height() as f32 / 2.0);
        Rect::new(self.pos.point() - half, self.pos.point() + half)
    }

    pub fn screen_to_world(&self, sxy: Point<f32>, screen_size: ScreenCoords) -> WorldCoords {
        let wx = (sxy.x - screen_size.width() as f32 / 2.0) / self.zoom + self.pos.x();
        let wy = (sxy.y - screen_size.height() as f32 / 2.0) / self.zoom + self.pos.y();
        WorldCoords::new(wx, wy)
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        *self.pos.x_mut() += dx / self.zoom;
        *self.pos.y_mut() += dy / self.zoom;
    }

    pub fn zoom_at(&mut self, sxy: Point<f32>, factor: f32, screen_size: ScreenCoords) {
        let wxy = self.screen_to_world(sxy, screen_size);
        let new_zoom = (self.zoom * factor).clamp(1.0, 100.0);

        if new_zoom == self.zoom {
            return;
        }

        self.zoom = new_zoom;
        let new_wxy = self.screen_to_world(sxy, screen_size);
        *self.pos.x_mut() += wxy.x() - new_wxy.x();
        *self.pos.y_mut() += wxy.y() - new_wxy.y();
    }
}
