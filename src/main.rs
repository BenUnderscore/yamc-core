//Uses
use glutin;
use std::sync::mpsc;
use std::thread;
use std::time;

//Modules
mod event_loop;

fn main() {
    let (proxy_tx, proxy_rx) = mpsc::channel();

    thread::spawn(move || {
        let proxy = proxy_rx.recv().unwrap();
        run(proxy);
    });

    event_loop::run_event_loop(proxy_tx);
}

fn run(event_loop_proxy: event_loop::EventLoopProxy) {
    let device_event_rx = {
        let (tx, rx) = mpsc::channel();
        event_loop_proxy.register_device_event_sender(Some(tx));
        rx
    };

    {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Yet Another (Crappy) Minecraft Clone")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

        let ctx = event_loop_proxy
            .create_windowed_context(window_builder)
            .unwrap();

        //The game loop
        let mut duration_behind: time::Duration = Default::default();
        let mut last_instant = time::Instant::now();
        let mut should_end = false;

        while !should_end {
            for ev in device_event_rx.try_iter() {
                match ev.event {
                    glutin::event::DeviceEvent::Key(input) => {
                        if let Some(key_code) = input.virtual_keycode {
                            match key_code {
                                glutin::event::VirtualKeyCode::Escape => should_end = true,
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }

            let new_instant = time::Instant::now();
            duration_behind += new_instant - last_instant;
            last_instant = new_instant;
        }
    }

    event_loop_proxy.exit();
}
