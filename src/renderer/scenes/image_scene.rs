use wgpu::{CommandEncoder, TextureView, RenderPipeline};

use crate::renderer::primitives::image::Image;

use super::scene::Scene;



pub struct ImageScene {
    pub image: Image,
}

impl Scene for ImageScene {
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

        let bind_group = self.image.bind_group.as_ref().unwrap();
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.image.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.image.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.draw_indexed(0..self.image.plane.get_indices().len() as u32, 0, 0..1);
    }
}

impl ImageScene {
  
}