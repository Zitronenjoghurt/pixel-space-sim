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

    pub fn fill_ellipse(&mut self, center: Point<f32>, rx: f32, ry: f32, color: RGBA) {
        let width = self.size.width as i32;
        let height = self.size.height as i32;

        let origin_x = self.visible_rect.min.x.floor() as i32;
        let origin_y = self.visible_rect.min.y.floor() as i32;

        let min_x = ((center.x - rx).floor() as i32 - origin_x).max(0);
        let max_x = ((center.x + rx).ceil() as i32 - origin_x).min(width - 1);
        let min_y = ((center.y - ry).floor() as i32 - origin_y).max(0);
        let max_y = ((center.y + ry).ceil() as i32 - origin_y).min(height - 1);

        if min_x > max_x || min_y > max_y {
            return;
        }

        let cx = center.x - origin_x as f32;
        let cy = center.y - origin_y as f32;
        let inv_rx_sq = 1.0 / (rx * rx);
        let inv_ry_sq = 1.0 / (ry * ry);

        for y in min_y..=max_y {
            let dy = y as f32 + 0.5 - cy;
            let dy_term = dy * dy * inv_ry_sq;

            if dy_term > 1.0 {
                continue;
            }

            let row_start = (y * width) as usize * 4;

            for x in min_x..=max_x {
                let dx = x as f32 + 0.5 - cx;

                if dx * dx * inv_rx_sq + dy_term <= 1.0 {
                    let idx = row_start + (x as usize * 4);
                    self.rgba[idx..idx + 4].copy_from_slice(&color);
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
