//Uses
use glutin;
use std::sync::mpsc;
use std::thread;

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
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Yet Another (Crappy) Minecraft Clone")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    event_loop_proxy
        .create_windowed_context(window_builder)
        .unwrap();
}
