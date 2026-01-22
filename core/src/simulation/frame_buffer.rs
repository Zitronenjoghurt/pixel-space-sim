use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;

#[derive(Clone)]
pub struct FrameBuffer {
    size: Size<u32>,
    visible_rect: Rect<f32>,
    data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: Size::new(width, height),
            visible_rect: Rect::default(),
            data: vec![0u8; (width * height * 4) as usize],
        }
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn resize(&mut self, size: Size<u32>) {
        if self.size != size && size.width > 0 && size.height > 0 {
            self.size = size;
            self.data.resize((size.width * size.height * 4) as usize, 0);
        }
    }

    pub fn set_visible_rect(&mut self, rect: Rect<f32>) {
        self.visible_rect = rect;
    }

    pub fn visible_rect(&self) -> Rect<f32> {
        self.visible_rect
    }

    pub fn pixels(&self) -> &[u8] {
        &self.data
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn zoom(&self) -> f32 {
        let w = self.visible_rect.width();
        if w > 0.0 {
            self.size.width as f32 / w
        } else {
            1.0
        }
    }

    pub fn world_to_screen(&self, world_pos: Point<f32>) -> Point<f32> {
        let relative = world_pos - self.visible_rect.min;
        let zoom = self.zoom();
        Point::new(relative.x * zoom, relative.y * zoom)
    }

    pub fn set_screen_pixel(&mut self, x: u32, y: u32, color: RGBA) {
        if x < self.size.width && y < self.size.height {
            let idx = ((y * self.size.width + x) * 4) as usize;
            self.data[idx..idx + 4].copy_from_slice(&color);
        }
    }

    pub fn fill_world_rect(&mut self, world_rect: Rect<f32>, color: RGBA) {
        let clamped = match world_rect.intersect(&self.visible_rect) {
            Some(r) => r,
            None => return,
        };

        let min_screen = self.world_to_screen(clamped.min);
        let max_screen = self.world_to_screen(clamped.max);

        let x0 = (min_screen.x.floor() as u32).min(self.size.width);
        let y0 = (min_screen.y.floor() as u32).min(self.size.height);
        let x1 = (max_screen.x.ceil() as u32).min(self.size.width);
        let y1 = (max_screen.y.ceil() as u32).min(self.size.height);

        if x1 <= x0 {
            return;
        }

        for y in y0..y1 {
            for x in x0..x1 {
                self.set_screen_pixel(x, y, color);
            }
        }
    }

    pub fn fill_cell(&mut self, world_pos: Point<f32>, color: RGBA) {
        let cell = Rect::new(
            Point::new(world_pos.x.floor(), world_pos.y.floor()),
            Point::new(world_pos.x.floor() + 1.0, world_pos.y.floor() + 1.0),
        );
        self.fill_world_rect(cell, color);
    }

    pub fn clear(&mut self, color: RGBA) {
        for pixel in self.data.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new(1, 1)
    }
}
