use bevy_ecs::system::Resource;

use crate::{pipelines::common::texture_bind_group_layout, primitives::vertex::Vertex};

#[derive(Resource)]
pub struct TexturePipeline(pub wgpu::RenderPipeline);

impl TexturePipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertex_shader = Self::vertex_shader(device);
        let fragment_shader = Self::fragment_shader(device);
        let texture_bind_group_layout = texture_bind_group_layout(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
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

        Self(render_pipeline)
    }

    fn vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("texture vertex shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/texture.vertex.wgsl").into()),
        })
    }

    fn fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("texture fragment shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("./shaders/texture.fragment.wgsl").into(),
            ),
        })
    }
}
