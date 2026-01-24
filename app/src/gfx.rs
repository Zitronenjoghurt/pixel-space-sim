use egui::FontDefinitions;
use egui_wgpu::{wgpu, ScreenDescriptor};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Gfx {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    cell_texture: wgpu::Texture,
    cell_texture_view: wgpu::TextureView,
    cell_buffer: Vec<u8>,
    cell_size: [u32; 2],
    render_pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    screen: ScreenDescriptor,
    textures: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    surface_size: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    uv_offset: [f32; 2],
    uv_scale: [f32; 2],
}

impl Gfx {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to find adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let initial_size = [256u32, 256];
        let (cell_texture, cell_texture_view) = create_cell_texture(&device, initial_size);
        let cell_buffer = vec![0u8; (initial_size[0] * initial_size[1] * 4) as usize];

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Cell Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[CameraUniform {
                uv_offset: [0.0, 0.0],
                uv_scale: [0.0, 0.0],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cell Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = create_bind_group(
            &device,
            &bind_group_layout,
            &cell_texture_view,
            &sampler,
            &camera_buffer,
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cell Shader"),
            source: wgpu::ShaderSource::Wgsl(CELL_SHADER.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cell Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cell Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let egui_ctx = egui::Context::default();
        egui_ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgba_unmultiplied(30, 30, 30, 240),
            ..egui::Visuals::dark()
        });

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "phosphor".into(),
            egui::FontData::from_static(egui_phosphor::Variant::Regular.font_bytes()),
        );
        if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            font_keys.insert(1, "phosphor".into());
        }
        egui_ctx.set_fonts(fonts);

        let max_tex = device.limits().max_texture_dimension_2d as usize;
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            Some(max_tex),
        );

        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1);

        Self {
            window,
            device,
            queue,
            surface,
            surface_config,
            cell_texture,
            cell_texture_view,
            cell_buffer,
            cell_size: initial_size,
            render_pipeline,
            sampler,
            bind_group_layout,
            bind_group,
            camera_buffer,
            egui_ctx,
            egui_state,
            egui_renderer,
            screen: ScreenDescriptor {
                size_in_pixels: [size.width, size.height],
                pixels_per_point: scale_factor,
            },
            textures: Default::default(),
            paint_jobs: Vec::new(),
            surface_size: [size.width, size.height],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            self.surface_size = [width, height];
            self.screen.size_in_pixels = [width, height];
        }
    }

    pub fn resize_cell_buffer(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);

        if self.cell_size != [width, height] {
            self.cell_size = [width, height];
            self.cell_buffer.resize((width * height * 4) as usize, 0);

            let (texture, view) = create_cell_texture(&self.device, [width, height]);
            self.cell_texture = texture;
            self.cell_texture_view = view;

            self.bind_group = create_bind_group(
                &self.device,
                &self.bind_group_layout,
                &self.cell_texture_view,
                &self.sampler,
                &self.camera_buffer,
            );
        }
    }

    pub fn cell_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.cell_buffer
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.screen.pixels_per_point
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.screen.pixels_per_point = scale_factor;
        self.egui_ctx.set_pixels_per_point(scale_factor);
    }

    pub fn set_camera(&mut self, uv_offset: [f32; 2], uv_scale: [f32; 2]) {
        let uniform = CameraUniform {
            uv_offset,
            uv_scale,
        };
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn prepare_ui(&mut self, ui_fn: impl FnOnce(&egui::Context)) {
        let input = self.egui_state.take_egui_input(&self.window);
        let output = self.egui_ctx.run(input, ui_fn);

        self.screen.pixels_per_point = self.egui_ctx.pixels_per_point();

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(&self.window, output.platform_output);
        self.paint_jobs = self
            .egui_ctx
            .tessellate(output.shapes, self.screen.pixels_per_point);
    }

    pub fn render(&mut self) {
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.cell_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.cell_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.cell_size[0]),
                rows_per_image: Some(self.cell_size[1]),
            },
            wgpu::Extent3d {
                width: self.cell_size[0],
                height: self.cell_size[1],
                depth_or_array_layers: 1,
            },
        );

        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
        };

        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cell Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..4, 0..1); // Fullscreen quad
        }

        for (id, delta) in &self.textures.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, delta);
        }
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.paint_jobs,
            &self.screen,
        );

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            self.egui_renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        for id in &std::mem::take(&mut self.textures).free {
            self.egui_renderer.free_texture(id);
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn on_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.egui_state
            .on_window_event(&self.window, event)
            .consumed
    }
}

fn create_cell_texture(
    device: &wgpu::Device,
    size: [u32; 2],
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Cell Texture"),
        size: wgpu::Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    (texture, view)
}

fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
    camera_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Cell Bind Group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: camera_buffer.as_entire_binding(),
            },
        ],
    })
}

const CELL_SHADER: &str = r#"
struct CameraUniform {
    uv_offset: vec2<f32>,
    uv_scale: vec2<f32>,
}

@group(0) @binding(0) var cell_texture: texture_2d<f32>;
@group(0) @binding(1) var cell_sampler: sampler;
@group(0) @binding(2) var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 4>(
        vec2(-1.0, -1.0), // 0: bottom-left
        vec2( 1.0, -1.0), // 1: bottom-right
        vec2(-1.0,  1.0), // 2: top-left
        vec2( 1.0,  1.0), // 3: top-right
    );

    var uvs = array<vec2<f32>, 4>(
        vec2(0.0, 0.0), // bottom-left screen -> UV bottom-left
        vec2(1.0, 0.0), // bottom-right screen -> UV bottom-right
        vec2(0.0, 1.0), // top-left screen -> UV top-left
        vec2(1.0, 1.0), // top-right screen -> UV top-right
    );

    var out: VertexOutput;
    out.position = vec4(positions[vertex_index], 0.0, 1.0);
    out.uv = camera.uv_offset + uvs[vertex_index] * camera.uv_scale;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(cell_texture, cell_sampler, in.uv);
}
"#;
