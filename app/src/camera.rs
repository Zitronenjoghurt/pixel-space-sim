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

    pub fn world_to_buffer(
        &self,
        wxy: WorldCoords,
        buffer_size: ScreenCoords,
    ) -> Option<ScreenCoords> {
        let bx = wxy.x() - self.pos.x() + buffer_size.width() as f32 / 2.0;
        let by = wxy.y() - self.pos.y() + buffer_size.height() as f32 / 2.0;

        if bx >= 0.0
            && bx < buffer_size.width() as f32
            && by >= 0.0
            && by < buffer_size.height() as f32
        {
            Some(ScreenCoords::new(bx as u32, by as u32))
        } else {
            None
        }
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
        self.zoom = self.zoom.clamp(1.0, 100.0); // Min 1.0: no sub-pixel rendering
        let new_wxy = self.screen_to_world(sxy, screen_size);
        *self.pos.x_mut() += wxy.x() - new_wxy.x();
        *self.pos.y_mut() += wxy.y() - new_wxy.y();
    }
}
