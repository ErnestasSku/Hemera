use bevy_ecs::system::Resource;

pub enum RenderTarget {
    WindowSurface(wgpu::Surface<'static>),
    Texture(wgpu::Texture),
}

pub enum RenderConfig {
    SurfaceConfiguration(wgpu::SurfaceConfiguration),
    TextureDescriptor(wgpu::TextureDescriptor<'static>),
}

#[derive(Resource)]
pub struct RenderContext {
    pub surface: RenderTarget,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: RenderConfig,
}

impl RenderContext {
    #[allow(dead_code)]
    pub async fn new(size: (u32, u32)) -> Self {
        let instance = wgpu::Instance::new(RenderContext::instance_desc());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&RenderContext::devices_desc(), None)
            .await
            .unwrap();

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Main texture plane"),
            view_formats: &[],
        };

        let texture = device.create_texture(&texture_desc);

        Self {
            surface: RenderTarget::Texture(texture),
            device,
            queue,
            config: RenderConfig::TextureDescriptor(texture_desc),
        }
    }

    #[allow(dead_code)]
    pub async fn new_with_window(window: winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(RenderContext::instance_desc());

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&RenderContext::devices_desc(), None)
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_formats = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_formats,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            // Default is 2
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        Self {
            surface: RenderTarget::WindowSurface(surface),
            device,
            queue,
            config: RenderConfig::SurfaceConfiguration(config),
        }
    }

    fn instance_desc() -> wgpu::InstanceDescriptor {
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: Default::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        }
    }

    fn devices_desc() -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            label: None,
        }
    }
}
