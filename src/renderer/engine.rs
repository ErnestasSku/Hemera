use image::{gif::GifDecoder, AnimationDecoder};
use std::time;
use wgpu::Queue;
use winit::window::Window;

use crate::renderer::{
    primitives::{image::Image, vertex::Vertex},
    scenes::scene::ImageScene,
};

use super::{
    scenes::scene::{GifScene, SceneType, TestImageScene},
    transitions::Transition, texture,
};

pub struct Engine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub scene: Option<SceneType>,
    pub transition: Option<Transition>,

    //Winit
    pub window: Window,
}

impl Engine {
    pub async fn new(window: Window) -> Engine {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../renderer/shaders/shader.wgsl").into(),
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
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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

        // println!("Created");
        Self {
            config,
            device,
            queue,
            render_pipeline,
            scene: None,
            transition: None,
            surface,
            size,
            window,
        }
    }

    pub fn load_scene(&mut self) {
        // self.load_images()
        self.load_gif();
    }

    pub fn load_transition(&mut self) {
        let mut transition = Transition::test(&self.device, &self.config);
        transition.create_index_buffer(&self.device);
        transition.create_vertex_buffer(&self.device);

        self.transition = Some(transition);
    }

    fn load_gif(&mut self) {
        let input =
            std::fs::File::open("C:/Users/ernes/Desktop/---/Programming/Rust/Hemera/images/2.gif")
                .unwrap();

        let device = &self.device;
        let mut decoder = GifDecoder::new(input).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding frames");

        let a = frames[0].clone();
        let b = a.buffer();

        let gif_frames = frames
            .iter()
            .map(|f| {
                let mut image = Image::test_gif(&self.device, &self.queue, 1.0, &f);

                image.create_bind_group(&device);
                image.create_index_buffer(&device);
                image.create_vertex_buffer(&device);

                let (num, denum) = f.delay().numer_denom_ms();
                let delay = time::Duration::from_millis((num / denum) as u64);
                (image, delay)
            })
            .collect::<Vec<(Image, time::Duration)>>();

        let gif_scene = GifScene {
            first_load: true,
            time_loaded: None,
            time_till_next_frame: 0,
            current_frame: 0,
            frames: gif_frames,
        };

        let scene = Some(SceneType::Gif(gif_scene));
        self.scene = scene;
    }

    fn load_images(&mut self) {
        let device = &self.device;
        let queue = &self.queue;

        let image1 = Image::test(&device, &queue, 0.1, 0.7, 0.7);
        let image2 = Image::test(&device, &queue, 0.2, 0.0, 0.0);
        let image3 = Image::test(&device, &queue, 0.3, -0.7, 0.2);

        let mut images = vec![image1, image2, image3];

        for image in images.iter_mut() {
            image.create_bind_group(&device);
            image.create_index_buffer(&device);
            image.create_vertex_buffer(&device);
        }

        let image_scene = TestImageScene { images };

        let scene = Some(SceneType::TestImages(image_scene));
        self.scene = scene;
    }

    pub fn update(&mut self) {
        if let Some(transition) = &mut self.transition {
            let now = std::time::Instant::now();
            let diff = now.duration_since(transition.time_started);
            transition
                .transition_uniform
                .update_time_offset(diff.as_secs_f32());

            self.queue.write_buffer(
                &transition.transition_buffer,
                0,
                bytemuck::cast_slice(&[transition.transition_uniform]),
            );
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let render_texture = &self.device.create_texture(&wgpu::TextureDescriptor {
            // Set the dimensions and format of the texture
            size: wgpu::Extent3d {
                width: 500,   // Replace with the actual width of your surface
                height: 500, // Replace with the actual height of your surface
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format, 
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Render Texture"),
            view_formats: &[],
        });

        
        let output = self.surface.get_current_texture()?;
        
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // println!("Prepare render");
        {
            self.scene.as_mut().map(|v| match v {
                SceneType::Image(img) => {
                    img.render_scene(&mut encoder, &render_texture.create_view(&wgpu::TextureViewDescriptor::default()), &self.render_pipeline, &self.device)
                }

                SceneType::TestImages(img) => {
                    img.render_scene(&mut encoder, &render_texture.create_view(&wgpu::TextureViewDescriptor::default()), &self.render_pipeline, &self.device)
                }

                SceneType::Gif(img) => {
                    img.render_scene(&mut encoder, &render_texture.create_view(&wgpu::TextureViewDescriptor::default()), &self.render_pipeline, &self.device)
                }
            });
        }

        ////


            // .create_view(&wgpu::TextureViewDescriptor::default());

        //Create texture bind group
        if let Some(transition) = &mut self.transition {
            let a = &render_texture.create_view(&wgpu::TextureViewDescriptor::default());
            transition.create_bind_group(&self.device, &a);

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Transition pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, //TODO
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.1,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&transition.transition_pipeline);

            render_pass.set_bind_group(0, transition.bind_group.as_ref().unwrap(), &[]);
            render_pass.set_bind_group(1, &transition.transition_bind_group, &[]);
            render_pass.set_vertex_buffer(0, transition.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(transition.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..transition.plane.get_indices().len() as u32, 0, 0..1)

        }

        ////

        // println!("{:?}", view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
