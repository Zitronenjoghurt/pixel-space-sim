use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::rgba::RGBA;
use crate::math::size::Size;
use crate::simulation::snapshot::SimSnapshot;
use rayon::prelude::*;

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

    pub fn resize(&mut self, size: Size<u32>) {
        if self.size != size && size.width > 0 && size.height > 0 {
            self.size = size;
            self.rgba.resize((size.width * size.height * 4) as usize, 0);
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

    pub fn fill_cells(&mut self, points: impl IntoIterator<Item = (Point<f32>, RGBA)>) {
        let points: Vec<_> = points.into_iter().collect();

        let zoom = self.zoom();
        let width = self.size.width as usize;
        let v_rect = self.visible_rect;
        let v_min = v_rect.min;

        self.rgba
            .par_chunks_mut(width * 4)
            .enumerate()
            .for_each(|(y, row)| {
                let row_u32: &mut [u32] = bytemuck::cast_slice_mut(row);
                let y_f = y as f32;

                for &(world_pos, color) in &points {
                    let cell_y = world_pos.y.floor();
                    let screen_y0 = (cell_y - v_min.y) * zoom;
                    let screen_y1 = screen_y0 + zoom;

                    if y_f < screen_y0 || y_f >= screen_y1 {
                        continue;
                    }

                    let cell_x = world_pos.x.floor();
                    let screen_x0 = (cell_x - v_min.x) * zoom;
                    let x0 = (screen_x0.max(0.0) as usize).min(width);
                    let x1 = ((screen_x0 + zoom).ceil() as usize).min(width);

                    if x1 > x0 {
                        row_u32[x0..x1].fill(u32::from(color));
                    }
                }
            });
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
