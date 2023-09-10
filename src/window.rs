use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) => {
                
                // state.update();
                // match state.render() {
                //     Ok(_) => {}
                //     // Reconfigure the surface if lost
                //     Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                //     // The system is out of memory, we should probably quit
                //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                //     // All other errors (Outdated, Timeout) should be resolved by the next frame
                //     Err(e) => eprintln!("{:?}", e),
                // }
            }
            Event::MainEventsCleared => {
                // state.window().request_redraw();
            }
            _ => {}
        }
    });
}