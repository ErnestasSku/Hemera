use wgpu::{
    util::DeviceExt, CommandEncoder, Device, RenderPipeline, SurfaceConfiguration, TextureFormat,
    TextureView,
};

use super::{
    primitives::{plane::Plane, vertex::Vertex},
    scenes::scene::{Scene, SceneType},
};

pub struct Transition {
    pub transition_uniform: TransitionUniform,
    pub transition_buffer: wgpu::Buffer,
    pub transition_bind_group: wgpu::BindGroup,
    pub transition_pipeline: wgpu::RenderPipeline,
    pub sampler: wgpu::Sampler,
    pub plane: Plane,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub time_started: std::time::Instant,

    pub scene: SceneType,
    pub scene_texture: wgpu::Texture,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransitionUniform {
    pub time_offset: f32,
    pub dissolve_speed: f32,
}

impl TransitionUniform {
    pub fn update_time_offset(&mut self, time: f32) {
        self.time_offset = time;
    }
}

impl Transition {
    pub fn test(
        device: &Device,
        config: &SurfaceConfiguration,
        scene: SceneType,
        format: TextureFormat,
    ) -> Self {
        let transition_uniform = TransitionUniform {
            dissolve_speed: 0.5,
            time_offset: 0.0,
        };

        let transition_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transition buffer"),
            contents: bytemuck::cast_slice(&[transition_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transition_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("transition uniform bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let transition_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("a"),
            layout: &transition_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transition_buffer.as_entire_binding(),
            }],
        });

        let plane = Plane::new(1.0);

        let time_started = std::time::Instant::now();

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Transition shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../renderer/shaders/transition2.wgsl").into(),
            ),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Transition render layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &transition_bind_group_layout],
                push_constant_ranges: &[],
            });

        let transition_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Transition pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main_vertex",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main_fragment",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let scene_texture = device.create_texture(&wgpu::TextureDescriptor {
            // Set the dimensions and format of the texture
            size: wgpu::Extent3d {
                width: 500,  // Replace with the actual width of your surface
                height: 500, // Replace with the actual height of your surface
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Render Texture"),
            view_formats: &[],
        });

        Self {
            transition_uniform,
            transition_buffer,
            transition_bind_group,
            plane,
            time_started,
            bind_group: None,
            index_buffer: None,
            vertex_buffer: None,
            sampler,
            transition_pipeline,
            scene,
            scene_texture,
        }
    }

    pub fn create_vertex_buffer(&mut self, device: &Device) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image vertex buffer"),
            contents: bytemuck::cast_slice(&self.plane.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn create_index_buffer(&mut self, device: &Device) {
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image 9ndex buffer"),
            contents: bytemuck::cast_slice(&self.plane.get_indices()),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.index_buffer = Some(index_buffer);
    }

    pub fn create_bind_group(&mut self, device: &Device, texture: &wgpu::TextureView) {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
            label: Some("transition bind group"),
        });

        self.bind_group = Some(diffuse_bind_group);
    }

    pub fn transition(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        device: &Device,
        render_pipeline: &RenderPipeline,
    ) {
        let texture_view = self
            .scene_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.scene
            .render_scene(encoder, &texture_view, render_pipeline);

        self.create_bind_group(device, &texture_view);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Transition pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.transition_pipeline);

        render_pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(1, &self.transition_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.draw_indexed(0..self.plane.get_indices().len() as u32, 0, 0..1)
    }
}
