use wgpu::{util::DeviceExt, Device};

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
            plane: Plane::new(0.5),
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

    pub fn create_vertex_buffer(&mut self, device: Device) -> &mut Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image vertex buffer"),
            contents: bytemuck::cast_slice(&self.plane.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        self.vertex_buffer = Some(vertex_buffer);
        self
    }

    pub fn create_index_buffer(&mut self, device: Device) -> &mut Self {
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image 9ndex buffer"),
            contents: bytemuck::cast_slice(&self.plane.get_indices()),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.index_buffer = Some(index_buffer);
        self
    }
}
