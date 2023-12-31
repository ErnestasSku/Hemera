use wgpu::{CommandEncoder, RenderPipeline, TextureView};

use crate::renderer::primitives::image::Image;

use super::scene::Scene;

pub struct TestImageScene {
    pub images: Vec<Image>,
}

impl Scene for TestImageScene {
    fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.5,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(pipeline);

        for i in self.images.iter() {
            let bind_group = i.bind_group.as_ref().unwrap();
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_vertex_buffer(0, i.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(
                i.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );

            render_pass.draw_indexed(0..i.plane.get_indices().len() as u32, 0, 0..1);
        }
    }
}

impl TestImageScene {}
