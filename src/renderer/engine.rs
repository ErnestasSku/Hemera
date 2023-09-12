use image::{gif::GifDecoder, AnimationDecoder};
use std::time;
use wgpu::Queue;
use winit::window::Window;

use crate::renderer::{
    primitives::{image::Image, vertex::Vertex},
    scenes::scene::ImageScene,
};

use super::scenes::scene::{GifScene, SceneType, TestImageScene};

pub struct Engine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub scene: Option<SceneType>,

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
            surface,
            size,
            window,
        }
    }

    pub fn load_scene(&mut self) {
        // self.load_images()
        self.load_gif();
    }

    fn load_gif(&mut self) {
        let input =
            std::fs::File::open("C:/Users/ernes/Desktop/---/Programming/Rust/Hemera/images/1.gif")
                .unwrap();
        // let mut options = gif::DecodeOptions::new();
        // options.set_color_output(gif::ColorOutput::RGBA);

        // let mut decoder = options.read_info(input).unwrap();
        // while let Some(frame) = decoder.read_next_frame().unwrap() {

        //     println!("Frame= {frame:?}");
        // }
        
        let device = &self.device;
        let mut decoder = GifDecoder::new(input).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding frames");

        let a = frames[0].clone();
        let b = a.buffer();

        let gif_frames = frames
            .iter()
            .map(|f| {
                let mut image = Image::test_gif(
                    &self.device,
                    &self.queue,
                    1.0,
                    &f,
                );

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

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                    img.render_scene(&mut encoder, &view, &self.render_pipeline, &self.device)
                }

                SceneType::TestImages(img) => {
                    img.render_scene(&mut encoder, &view, &self.render_pipeline, &self.device)
                }

                SceneType::Gif(img) => {
                    img.render_scene(&mut encoder, &view, &self.render_pipeline, &self.device)
                }
            });
        }

        // println!("After render");

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
