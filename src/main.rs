use std::time::Duration;

use bevy_ecs::prelude::*;
use color_eyre::Report;
use event_loop::{AfterRenderSchedule, StartupSchedule, UpdateSchedule};
use renderer::render_context::RenderContext;
use winit::{platform::pump_events::EventLoopExtPumpEvents, window::WindowBuilder};

mod event_loop;
mod renderer;

#[derive(Component)]
struct Scene;

#[derive(Component)]
struct Entity;

#[derive(Component)]
struct Material {
    bind_group: wgpu::BindGroup,
}

#[derive(Component)]
struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_elements: u32,
}

fn main() -> Result<(), Report> {
    setup()?;
    let (mut event_loop, window) = setup_winit();
    let mut world = World::new();

    //
    let render_context = pollster::block_on(RenderContext::new_with_window(window));
    world.insert_resource(render_context);

    // Schedules
    let mut startup = Schedule::new(StartupSchedule);
    let mut update = Schedule::new(UpdateSchedule);
    let mut after_render = Schedule::new(AfterRenderSchedule);

    update.add_systems(render_final);

    startup.run(&mut world);

    loop {
        update.run(&mut world);
        after_render.run(&mut world);

        let pump_status =
            event_loop.pump_events(Some(Duration::from_millis(10)), winit_event_handler);

        match pump_status {
            winit::platform::pump_events::PumpStatus::Continue => {}
            winit::platform::pump_events::PumpStatus::Exit(_) => break,
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn render_final(render_context: ResMut<RenderContext>) {
    let output = &render_context.target;
    match output {
        renderer::render_context::RenderTarget::WindowSurface(surface) => {
            let output = surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder =
                render_context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            render_context
                .queue
                .submit(std::iter::once(encoder.finish()));
            output.present();
        }
        renderer::render_context::RenderTarget::Texture(_texture) => {
            // Todo
        }
    };
}

fn setup() -> Result<(), Report> {
    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn setup_winit() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let monitor_handle = event_loop.available_monitors().next().unwrap();
    let _video_mode = monitor_handle
        .video_modes()
        .find(|p| p.size().width == 1920 && p.size().height == 1080)
        .unwrap();

    let window = WindowBuilder::new()
        // .with_fullscreen(Some(winit::window::Fullscreen::Exclusive(video_mode)))
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(
            monitor_handle,
        ))))
        .build(&event_loop)
        .unwrap();
    (event_loop, window)
}

fn winit_event_handler(
    event: winit::event::Event<()>,
    elwt: &winit::event_loop::EventLoopWindowTarget<()>,
) {
    if let winit::event::Event::WindowEvent {
        window_id: _,
        event,
    } = event
    {
        match event {
            winit::event::WindowEvent::Resized(_) => {}
            winit::event::WindowEvent::Moved(_) => {}
            winit::event::WindowEvent::CloseRequested => elwt.exit(),
            winit::event::WindowEvent::Destroyed => elwt.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                println!("Redraw");
            }
            _ => {}
        }
    }
}
