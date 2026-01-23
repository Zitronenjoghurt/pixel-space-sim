use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;
use crate::simulation::snapshot::SimSnapshot;

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

    pub fn fill_cells(&mut self, points: impl IntoIterator<Item = (Point<f32>, RGBA)>) {
        let width = self.size.width as i32;
        let height = self.size.height as i32;
        let v_min = self.visible_rect.min;

        let origin_x = v_min.x.floor() as i32;
        let origin_y = v_min.y.floor() as i32;

        for (world_pos, color) in points {
            let cell_x = world_pos.x.floor() as i32;
            let cell_y = world_pos.y.floor() as i32;

            let buf_x = cell_x - origin_x;
            let buf_y = cell_y - origin_y;

            if buf_x >= 0 && buf_x < width && buf_y >= 0 && buf_y < height {
                let idx = ((buf_y * width + buf_x) * 4) as usize;
                self.rgba[idx..idx + 4].copy_from_slice(&color);
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
