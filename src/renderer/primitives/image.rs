use wgpu::{util::DeviceExt, Device, BindGroupLayout};

use crate::renderer::texture::Texture;

use super::plane::Plane;

pub struct Image {
    pub plane: Plane,
    pub texture: Texture,
    pub bind_group: Option<wgpu::BindGroup>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Image {
    pub fn test(device: &Device, queue: &wgpu::Queue) -> Self {
        Image {
            plane: Plane::new(1.0),
            texture: Texture::from_bytes(
                device,
                queue,
                include_bytes!("../../../images/1.png"),
                "label",
            )
            .unwrap(),
            bind_group: None,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn create_vertex_buffer(&mut self, device: &Device)  {
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

    pub fn create_bind_group(&mut self, device: &Device, texture_bind_group_layout: &BindGroupLayout) {
        // let texture_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 0,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Texture {
        //                     multisampled: false,
        //                     view_dimension: wgpu::TextureViewDimension::D2,
        //                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //                 },
        //                 count: None,
        //             },
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 1,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 // This should match the filterable field of the
        //                 // corresponding Texture entry above.
        //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        //                 count: None,
        //             },
        //         ],
        //         label: Some("texture_bind_group_layout"),
        //     });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        self.bind_group = Some(diffuse_bind_group);
    }
}
