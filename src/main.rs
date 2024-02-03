use std::time::Duration;

use bevy_ecs::prelude::*;
use event_loop::{AfterRenderSchedule, StartupSchedule, UpdateSchedule};
use winit::{platform::pump_events::EventLoopExtPumpEvents, window::WindowBuilder};

mod event_loop;

fn main() {
    let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut world = World::new();

    // Schedules
    let mut startup = Schedule::new(StartupSchedule);
    let mut update = Schedule::new(UpdateSchedule);
    let mut after_render = Schedule::new(AfterRenderSchedule);

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
