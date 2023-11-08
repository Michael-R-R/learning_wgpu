use std::time::Instant;

use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;

mod graphics;
use graphics::State;

const SCR_W: u32 = 800;
const SCR_H: u32 = 600;

fn main() {

    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Learn wgpu")
        .with_inner_size(winit::dpi::LogicalSize::new(SCR_W, SCR_H))
        .build(&event_loop)
        .expect("Failed to create a window");

    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut state = State::new(window).await;
    let mut last_time = Instant::now();

    event_loop.run(move | event, _, cf | {
        
        state.handle_event(&event);

        match event {
            Event::MainEventsCleared => state.window.request_redraw(),
            Event::RedrawEventsCleared => {
                let curr_now = Instant::now();
                let dt = (curr_now - last_time).as_secs_f32();
                last_time = curr_now;
                println!("{dt}");

                state.update(dt);
                match state.render(dt) {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => cf.set_exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::WindowEvent { event, .. } => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::Resized(size) => state.resize(size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(*new_inner_size),
                        WindowEvent::CloseRequested => cf.set_exit(),
                        WindowEvent::KeyboardInput { input, .. } => {
                            match input.virtual_keycode {
                                Some(key) => {
                                    match key {
                                        VirtualKeyCode::Escape => cf.set_exit(),
                                        _ => {}
                                    }
                                }
                                None => {},
                            }
                        },
                        _ => {},
                    }
                }
            },
            _ => {}
        }
    });
}
