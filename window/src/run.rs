use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen},
};

use crate::api::*;

pub fn run<T: API + 'static>() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Title")
        .with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0))
        .build(&event_loop).unwrap();

    let mut monitor_index = 0;
    let mut monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitor found!");
    println!("Monitor: {:?}", monitor.name());

    let mut mode_index = 0;
    let mut mode = monitor.video_modes().next().expect("no mode found");
    println!("Mode: {mode}");

    let fullscreen = Some(Fullscreen::Borderless(Some(monitor.clone())));
    println!("Setting mode: {fullscreen:?}");
    window.set_fullscreen(fullscreen);

    let mut state: T = API::new(window);
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, window_id: _ }
        if !state.input(event) => {
            match event {
                WindowEvent::CloseRequested |
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                },
                _ => {}
            }
        },
        Event::RedrawRequested(_window_id) => {
            state.update();
            match state.render() {
                Ok(_) =>
                    {}
                /*
                Err(wgpu::SurfaceError::Lost) => {
                    state.resize(state.size());
                },  
                Err(wgpu::SurfaceError::OutOfMemory) =>
                    *control_flow = ControlFlow::Exit,
                */
                Err(e) =>
                    eprintln!("{:?}", e),
            }
        },
        Event::MainEventsCleared => {
            state.window().request_redraw();
        },
        _ => {}
    });
}