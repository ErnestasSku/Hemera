use std::time::{Duration, Instant};

use wgpu::{CommandEncoder, Device, RenderPipeline, TextureView};

use crate::renderer::primitives::image::Image;

trait Scene {
    fn render_scene();
}

#[allow(dead_code)]
pub enum SceneType {
    Image(ImageScene),
    TestImages(TestImageScene),
    Gif(GifScene),
}

pub struct ImageScene {
    pub image: Image,
}

pub struct TestImageScene {
    pub images: Vec<Image>,
}

pub struct GifScene {
    pub first_load: bool,
    pub time_loaded: Option<Instant>,
    pub time_till_next_frame: u128,
    pub current_frame: u32,
    pub frames: Vec<(Image, Duration)>,
}

impl ImageScene {
    pub fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
        _device: &Device,
    ) {
        // println!("Render");
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

impl TestImageScene {
    pub fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
        _device: &Device,
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

impl GifScene {
    pub fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
        _device: &Device,
    ) {
        let current_time = std::time::Instant::now();

        if self.first_load {
            self.first_load = false;
            self.time_loaded = Some(std::time::Instant::now());
            self.time_till_next_frame = self.frames.get(0).unwrap().1.as_millis();
            println!("loaded");
        }

        // println!("{:?}", self.time_loaded);

        if self.time_loaded.is_some() {
            let time = self
                .time_loaded
                .unwrap()
                .checked_add(Duration::from_millis(
                    (self.time_till_next_frame).try_into().unwrap(),
                ))
                .unwrap();
            if time < current_time {
                self.current_frame += 1;
                if self.current_frame > (self.frames.len() - 1) as u32 {
                    self.current_frame = 0;
                }

                self.time_loaded = Some(current_time);
                self.time_till_next_frame = self
                    .frames
                    .get(self.current_frame as usize)
                    .unwrap()
                    .1
                    .as_millis();
            }
        }

        // println!("{:?}", self.current_frame);

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

        let image = &self.frames.get(self.current_frame as usize).unwrap().0;

        render_pass.set_bind_group(0, image.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, image.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(image.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..image.plane.get_indices().len() as u32, 0, 0..1);
    }
}
