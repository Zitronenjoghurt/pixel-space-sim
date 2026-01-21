use pss_core::math::point::Point;
use pss_core::math::rect::Rect;
use pss_core::math::screen_coords::ScreenCoords;
use pss_core::math::world_coords::WorldCoords;

pub struct Camera {
    pub pos: WorldCoords,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: WorldCoords::default(),
            zoom: 1.0,
        }
    }

    pub fn visible_rect(&self, size: ScreenCoords) -> Rect<f32> {
        let half_w = (size.width() as f32 / 2.0) / self.zoom;
        let half_h = (size.height() as f32 / 2.0) / self.zoom;
        Rect::new(
            Point::new(self.pos.x() - half_w, self.pos.y() - half_h),
            Point::new(self.pos.x() + half_w, self.pos.y() + half_h),
        )
    }

    pub fn world_to_screen(&self, wxy: WorldCoords, screen_size: ScreenCoords) -> ScreenCoords {
        let sx = ((wxy.x() - self.pos.x()) * self.zoom + screen_size.width() as f32 / 2.0) as u32;
        let sy = ((wxy.y() - self.pos.y()) * self.zoom + screen_size.height() as f32 / 2.0) as u32;
        ScreenCoords::new(sx, sy)
    }

    pub fn screen_to_world(&self, sxy: ScreenCoords, screen_size: ScreenCoords) -> WorldCoords {
        let wx = (sxy.x() as f32 - screen_size.width() as f32 / 2.0) / self.zoom + self.pos.x();
        let wy = (sxy.y() as f32 - screen_size.height() as f32 / 2.0) / self.zoom + self.pos.y();
        WorldCoords::new(wx, wy)
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        *self.pos.x_mut() += dx / self.zoom;
        *self.pos.y_mut() += dy / self.zoom;
    }

    pub fn zoom_at(&mut self, sxy: ScreenCoords, factor: f32, screen_size: ScreenCoords) {
        let wxy = self.screen_to_world(sxy, screen_size);
        self.zoom *= factor;
        self.zoom = self.zoom.clamp(0.1, 50.0);
        let new_wxy = self.screen_to_world(sxy, screen_size);
        *self.pos.x_mut() += wxy.x() - new_wxy.x();
        *self.pos.y_mut() += wxy.y() - new_wxy.y();
    }
}
