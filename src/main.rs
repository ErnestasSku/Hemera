use std::time::Duration;

use bevy_ecs::prelude::*;
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget},
    platform::pump_events::EventLoopExtPumpEvents,
    window::WindowBuilder,
};

#[derive(Component)]
struct Res1(i32);

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut world = World::new();

    let mut schedule = Schedule::default();
    schedule.add_systems(hello_world);

    loop {
        let pump_status =
            event_loop.pump_events(Some(Duration::from_millis(10)), winit_event_handler);

        schedule.run(&mut world);

        match pump_status {
            winit::platform::pump_events::PumpStatus::Continue => {}
            winit::platform::pump_events::PumpStatus::Exit(_) => break,
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn hello_world() {
    println!("Hello ecs");
}

fn winit_event_handler(event: Event<()>, elwt: &EventLoopWindowTarget<()>) {
    if let Event::WindowEvent {
        window_id: _,
        event,
    } = event
    {
        match event {
            winit::event::WindowEvent::Resized(_) => {}
            winit::event::WindowEvent::Moved(_) => {}
            winit::event::WindowEvent::CloseRequested => {
                println!("exit");
                elwt.exit()
            }
            winit::event::WindowEvent::Destroyed => elwt.exit(),
            winit::event::WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }
}
