use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::engine::Engine;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let mut state = State::new(window).await;
    let mut state = Engine::new(window).await;

    // event_loop.run(move |event, _, control_flow| {
    //     match event {
    //         Event::RedrawRequested(window_id) => {
    //             println!("Redraw");
    //             engine.update();
    //             let _ = engine.render();
    //             // state.update();
    //             // match state.render() {
    //             //     Ok(_) => {}
    //             //     // Reconfigure the surface if lost
    //             //     Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
    //             //     // The system is out of memory, we should probably quit
    //             //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
    //             //     // All other errors (Outdated, Timeout) should be resolved by the next frame
    //             //     Err(e) => eprintln!("{:?}", e),
    //             // }
    //         }
    //         Event::MainEventsCleared => {
    //             // state.window().request_redraw();
    //         }
    //         _ => {}
    //     }
    // });

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        // state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {},
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // println!("main event cleared");
                state.window.request_redraw();
            }
            _ => {}
        }
    });
}
