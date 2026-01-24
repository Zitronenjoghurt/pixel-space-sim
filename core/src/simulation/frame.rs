use crate::math::eclipse::Eclipse;
use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;
use crate::simulation::sync::snapshot::SimSnapshot;

#[derive(Clone)]
pub struct SimFrame {
    rgba: Vec<u8>,
    size: Size<u32>,
    visible_rect: Rect<f32>,
    pub snapshot: SimSnapshot,
}

impl SimFrame {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: Size::new(width, height),
            visible_rect: Rect::default(),
            rgba: vec![0u8; (width * height * 4) as usize],
            snapshot: SimSnapshot::default(),
        }
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn resize_to_visible_rect(&mut self) {
        let width = (self.visible_rect.width().ceil() as u32 + 1).max(1);
        let height = (self.visible_rect.height().ceil() as u32 + 1).max(1);

        if self.size.width != width || self.size.height != height {
            self.size = Size::new(width, height);
            self.rgba.resize((width * height * 4) as usize, 0);
        }
    }

    pub fn set_visible_rect(&mut self, rect: Rect<f32>) {
        self.visible_rect = rect;
    }

    pub fn visible_rect(&self) -> Rect<f32> {
        self.visible_rect
    }

    pub fn write_rgba(&self, dest: &mut [u8]) {
        let len = self.rgba.len().min(dest.len());
        dest[..len].copy_from_slice(&self.rgba[..len]);
    }

    pub fn fill_cell(&mut self, world_pos: Point<f32>, color: RGBA) {
        let width = self.size.width as i32;
        let height = self.size.height as i32;

        let origin_x = self.visible_rect.min.x.floor() as i32;
        let origin_y = self.visible_rect.min.y.floor() as i32;

        let buf_x = world_pos.x.floor() as i32 - origin_x;
        let buf_y = world_pos.y.floor() as i32 - origin_y;

        if buf_x >= 0 && buf_x < width && buf_y >= 0 && buf_y < height {
            let idx = ((buf_y * width + buf_x) * 4) as usize;
            self.rgba[idx..idx + 4].copy_from_slice(&color);
        }
    }

    pub fn fill_ellipse(&mut self, eclipse: Eclipse<f32>, color: RGBA) {
        let width = self.size.width as i64;
        let height = self.size.height as i64;

        let origin_x = self.visible_rect.min.x.floor() as i64;
        let origin_y = self.visible_rect.min.y.floor() as i64;

        for world_point in eclipse.to_i64().iter() {
            let buf_x = world_point.x - origin_x;
            let buf_y = world_point.y - origin_y;

            if buf_x >= 0 && buf_x < width && buf_y >= 0 && buf_y < height {
                let idx = ((buf_y * width + buf_x) * 4) as usize;
                self.rgba[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    pub fn fill_rect(&mut self, rect: Rect<f32>, color: RGBA) {
        if let Some(visible_part) = self.visible_rect.intersect(&rect) {
            let origin_x = self.visible_rect.min.x.floor() as i64;
            let origin_y = self.visible_rect.min.y.floor() as i64;
            let width = self.size.width as i64;

            let draw_rect = visible_part.to_i64();
            for world_point in draw_rect.iter() {
                let buf_x = world_point.x - origin_x;
                let buf_y = world_point.y - origin_y;

                if buf_x >= 0 && buf_x < width {
                    let idx = ((buf_y * width + buf_x) * 4) as usize;
                    if idx + 4 <= self.rgba.len() {
                        self.rgba[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.rgba.fill(0);
    }
}

impl Default for SimFrame {
    fn default() -> Self {
        Self::new(1, 1)
    }
}
