use egui::FontDefinitions;
use egui_wgpu::{wgpu, ScreenDescriptor};
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::window::Window;

pub struct Gfx {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    screen: ScreenDescriptor,
    textures: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    surface_size: [u32; 2],
    buffer_size: [u32; 2],
}

impl Gfx {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());

        let pixels = Pixels::new(size.width, size.height, surface).unwrap();

        let egui_ctx = egui::Context::default();

        egui_ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgba_unmultiplied(30, 30, 30, 200),
            ..egui::Visuals::dark()
        });

        egui_ctx.set_pixels_per_point(1.5);

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "phosphor".into(),
            egui::FontData::from_static(egui_phosphor::Variant::Regular.font_bytes()),
        );
        if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            font_keys.insert(1, "phosphor".into());
        }
        egui_ctx.set_fonts(fonts);

        let max_tex = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            Some(max_tex),
        );

        let egui_renderer =
            egui_wgpu::Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);

        Self {
            window,
            pixels,
            egui_ctx,
            egui_state,
            egui_renderer,
            screen: ScreenDescriptor {
                size_in_pixels: [size.width, size.height],
                pixels_per_point: 1.5,
            },
            textures: Default::default(),
            paint_jobs: Vec::new(),
            surface_size: [size.width, size.height],
            buffer_size: [size.width, size.height],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.pixels.resize_surface(width, height).unwrap();
            self.surface_size = [width, height];
            self.screen.size_in_pixels = [width, height];
        }
    }

    pub fn set_buffer_size(&mut self, width: u32, height: u32) {
        let width = width.clamp(1, self.surface_size[0]);
        let height = height.clamp(1, self.surface_size[1]);

        if self.buffer_size != [width, height] {
            self.pixels.resize_buffer(width, height).unwrap();
            self.buffer_size = [width, height];
        }
    }

    pub fn buffer_size(&self) -> [u32; 2] {
        self.buffer_size
    }

    pub fn prepare_ui(&mut self, ui_fn: impl FnOnce(&egui::Context)) {
        let input = self.egui_state.take_egui_input(&self.window);
        let output = self.egui_ctx.run(input, ui_fn);

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(&self.window, output.platform_output);
        self.paint_jobs = self
            .egui_ctx
            .tessellate(output.shapes, self.screen.pixels_per_point);
    }

    pub fn render(&mut self) {
        let (jobs, screen, textures, renderer) = (
            &self.paint_jobs,
            &self.screen,
            &self.textures,
            &mut self.egui_renderer,
        );

        self.pixels
            .render_with(|encoder, target, ctx| {
                ctx.scaling_renderer.render(encoder, target);

                for (id, delta) in &textures.set {
                    renderer.update_texture(&ctx.device, &ctx.queue, *id, delta);
                }
                renderer.update_buffers(&ctx.device, &ctx.queue, encoder, jobs, screen);

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                renderer.render(&mut rpass, jobs, screen);
                Ok(())
            })
            .unwrap();

        for id in &std::mem::take(&mut self.textures).free {
            self.egui_renderer.free_texture(id);
        }
    }

    pub fn frame(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }

    pub fn set_pixels_per_point(&mut self, pixels_per_point: f32) {
        self.screen.pixels_per_point = pixels_per_point;
        self.egui_ctx.set_pixels_per_point(pixels_per_point);
    }

    pub fn on_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.egui_state
            .on_window_event(&self.window, event)
            .consumed
    }
}
