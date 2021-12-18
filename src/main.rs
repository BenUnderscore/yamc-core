
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;

fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    events_loop.run(
        |ev, _target, control_flow| {
            match ev
            {
                Event::WindowEvent { window_id, event } =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                _ => (),
            };
        }
    );
}