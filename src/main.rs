
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin;

//Module definitions
mod render;

fn main() {
    let mut events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let ctx = glutin::ContextBuilder::new().build_windowed(wb, &events_loop).unwrap();
    
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