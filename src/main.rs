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
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Yet Another (Crappy) Minecraft Clone")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    let ctx = event_loop_proxy
        .create_windowed_context(window_builder)
        .unwrap();

    //The game loop
    let mut duration_behind: time::Duration = Default::default();
    let mut last_instant = time::Instant::now();
    loop {
        let new_instant = time::Instant::now();
        duration_behind += new_instant - last_instant;
        last_instant = new_instant;
    }
}
