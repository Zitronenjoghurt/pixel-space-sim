use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::size::Size;

#[derive(Clone)]
pub struct FrameBuffer {
    pub visible_rect: Rect<f32>,
    size: Size<u16>,
    data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            size: Size::new(width, height),
            visible_rect: Rect::default(),
            data: vec![0u8; width as usize * height as usize * 4],
        }
    }

    pub fn size(&self) -> Size<u16> {
        self.size
    }

    pub fn resize(&mut self, size: Size<u16>) {
        if self.size != size {
            self.size = size;
            self.data
                .resize(size.width as usize * size.height as usize * 4, 0);
        }
    }

    pub fn pixels(&self) -> &[u8] {
        &self.data
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn set_pixel(&mut self, world_pos: Point<f32>, color: [u8; 4]) {
        if let Some(px) = self.world_to_pixel(world_pos) {
            self.set_buffer_pixel(px, color);
        }
    }

    fn set_buffer_pixel(&mut self, pos: Point<u16>, color: [u8; 4]) {
        if pos.x < self.size.width && pos.y < self.size.height {
            let idx = (pos.y as usize * self.size.width as usize + pos.x as usize) * 4;
            self.data[idx..idx + 4].copy_from_slice(&color);
        }
    }

    pub fn world_to_pixel(&self, world_pos: Point<f32>) -> Option<Point<u16>> {
        let relative = world_pos - self.visible_rect.min;

        if !self.visible_rect.contains(world_pos) {
            return None;
        }

        Some(Point::new(relative.x as u16, relative.y as u16))
    }

    pub fn clear(&mut self, color: [u8; 4]) {
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
