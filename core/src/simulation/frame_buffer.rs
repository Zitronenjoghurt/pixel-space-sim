use crate::math::rect::Rect;

#[derive(Clone)]
pub struct FrameBuffer {
    pub height: u16,
    pub width: u16,
    pub visible_rect: Rect<f32>,
    data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            visible_rect: Rect::default(),
            data: vec![0u8; width as usize * height as usize * 4],
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize(width as usize * height as usize * 4, 0);
        }
    }

    pub fn pixels(&self) -> &[u8] {
        &self.data
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn set_pixel(&mut self, wx: f32, wy: f32, color: [u8; 4]) {
        if let Some((x, y)) = self.world_to_pixel(wx, wy) {
            self.set_buffer_pixel(x, y, color);
        }
    }

    fn set_buffer_pixel(&mut self, x: u16, y: u16, color: [u8; 4]) {
        if x < self.width && y < self.height {
            let idx = (y as usize * self.width as usize + x as usize) * 4;
            self.data[idx..idx + 4].copy_from_slice(&color);
        }
    }

    pub fn world_to_pixel(&self, wx: f32, wy: f32) -> Option<(u16, u16)> {
        let rx = wx - self.visible_rect.min.x;
        let ry = wy - self.visible_rect.min.y;

        if rx < 0.0
            || ry < 0.0
            || rx >= self.visible_rect.width()
            || ry >= self.visible_rect.height()
        {
            return None;
        }

        Some((rx as u16, ry as u16))
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
