//Uses
use std::sync::mpsc;
use thiserror::Error;
use wgpu;
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

pub use winit;

//WINDOW MANAGER EVENT DEFINITION
#[derive(Debug)]
enum EventLoopEvent {
    CreateWindow {
        builder: WindowBuilder,
        response_tx: mpsc::Sender<Result<()>>,
    },
    CreateWgpuSurface {
        instance: wgpu::Instance,
        response_tx: mpsc::Sender<(wgpu::Instance, Result<wgpu::Surface>)>,
    },
    RegisterDeviceEventSender {
        sender: Option<mpsc::Sender<DeviceEvent>>,
    },
    RegisterWindowEventSender {
        sender: Option<mpsc::Sender<WindowEvent>>,
    },
    GetWindowInnerSize {
        response_tx: mpsc::Sender<Result<winit::dpi::PhysicalSize<u32>>>,
    },
    Exit,
}

type Result<T> = std::result::Result<T, EventLoopError>;

#[derive(Error, Debug)]
pub enum EventLoopError {
    #[error("A window was already created")]
    WindowExists,
    #[error("No valid window exists to perform operation")]
    WindowMissing,
    #[error(transparent)]
    WinitOsError(#[from] winit::error::OsError),
}

//EVENT LOOP PROXY DEFINITION
pub struct EventLoopProxy {
    el_proxy: winit::event_loop::EventLoopProxy<EventLoopEvent>,
}

impl EventLoopProxy {
    pub fn create_window(&self, wb: WindowBuilder) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let event = EventLoopEvent::CreateWindow {
            builder: wb,
            response_tx: tx,
        };
        self.el_proxy.send_event(event).unwrap();

        rx.recv().unwrap()
    }

    //Temporarily takes ownership of the passed instance in order to use it on another thread
    pub fn create_wgpu_surface(
        &self,
        instance: wgpu::Instance,
    ) -> (wgpu::Instance, Result<wgpu::Surface>) {
        let (tx, rx) = mpsc::channel();

        let event = EventLoopEvent::CreateWgpuSurface {
            instance,
            response_tx: tx,
        };
        self.el_proxy.send_event(event).unwrap();

        rx.recv().unwrap()
    }

    pub fn register_device_event_sender(&self, tx: Option<mpsc::Sender<DeviceEvent>>) {
        let event = EventLoopEvent::RegisterDeviceEventSender { sender: tx };
        self.el_proxy.send_event(event).unwrap();
    }

    pub fn create_device_event_channel(&self) -> mpsc::Receiver<DeviceEvent> {
        let (tx, rx) = mpsc::channel();
        self.register_device_event_sender(Some(tx));
        rx
    }

    pub fn register_window_event_sender(&self, tx: Option<mpsc::Sender<WindowEvent>>) {
        let event = EventLoopEvent::RegisterWindowEventSender { sender: tx };
        self.el_proxy.send_event(event).unwrap();
    }

    pub fn create_window_event_channel(&self) -> mpsc::Receiver<WindowEvent> {
        let (tx, rx) = mpsc::channel();
        self.register_window_event_sender(Some(tx));
        rx
    }

    pub fn get_window_inner_size(&self) -> Result<winit::dpi::PhysicalSize<u32>> {
        let (tx, rx) = mpsc::channel();

        let event = EventLoopEvent::GetWindowInnerSize { response_tx: tx };
        self.el_proxy.send_event(event).unwrap();

        rx.recv().unwrap()
    }

    pub fn exit(&self) {
        self.el_proxy.send_event(EventLoopEvent::Exit).unwrap();
    }
}

pub struct DeviceEvent {
    pub device_id: winit::event::DeviceId,
    pub event: winit::event::DeviceEvent,
}

pub enum WindowEvent {
    CloseRequested,
}

//EVENT LOOP DEFINITION
struct EventLoopContext {
    main_window: Option<Window>,
    device_event_sender: Option<mpsc::Sender<DeviceEvent>>,
    window_event_sender: Option<mpsc::Sender<WindowEvent>>,
}

//Hijacks the calling thread (must be the main thread)
//The main thread becomes the event loop of the application
//To interact with the event loop a proxy is sent through the channel
pub fn run_event_loop(proxy_tx: mpsc::Sender<EventLoopProxy>) -> ! {
    let event_loop = EventLoop::<EventLoopEvent>::with_user_event();
    let proxy = event_loop.create_proxy();
    proxy_tx.send(EventLoopProxy { el_proxy: proxy }).unwrap();
    let mut ctx = EventLoopContext {
        main_window: None,
        device_event_sender: None,
        window_event_sender: None,
    };

    let mut is_control_flow_initialized = false;
    event_loop.run(move |ev, target, control_flow| {
        if !is_control_flow_initialized {
            *control_flow = ControlFlow::Wait;
            is_control_flow_initialized = true;
        }

        match ev {
            Event::UserEvent(user_ev) => handle_event(&mut ctx, user_ev, target, control_flow),
            Event::DeviceEvent { device_id, event } => {
                if let Some(tx) = &ctx.device_event_sender {
                    tx.send(DeviceEvent { device_id, event }).unwrap();
                }
            }
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    if let Some(tx) = &ctx.window_event_sender {
                        tx.send(WindowEvent::CloseRequested).unwrap();
                    }
                }
                _ => (),
            },
            _ => (),
        };
    });
}

fn handle_event(
    ctx: &mut EventLoopContext,
    user_ev: EventLoopEvent,
    target: &EventLoopWindowTarget<EventLoopEvent>,
    control_flow: &mut ControlFlow,
) {
    match user_ev {
        EventLoopEvent::CreateWindow {
            builder,
            response_tx,
        } => {
            let window_result = builder.build(target);
            response_tx
                .send(match window_result {
                    Ok(window) => {
                        ctx.main_window = Some(window);
                        Ok(())
                    }
                    Err(os_error) => Err(EventLoopError::WinitOsError(os_error)),
                })
                .unwrap();
        }
        EventLoopEvent::CreateWgpuSurface {
            instance,
            response_tx,
        } => {
            response_tx
                .send(match &ctx.main_window {
                    Some(window) => unsafe {
                        let surface = instance.create_surface(&window);
                        (instance, Ok(surface))
                    },
                    None => (instance, Err(EventLoopError::WindowMissing)),
                })
                .unwrap();
        }
        EventLoopEvent::RegisterDeviceEventSender { sender } => {
            ctx.device_event_sender = sender;
        }
        EventLoopEvent::RegisterWindowEventSender { sender } => {
            ctx.window_event_sender = sender;
        }
        EventLoopEvent::GetWindowInnerSize { response_tx } => {
            response_tx
                .send(match &ctx.main_window {
                    Some(window) => Ok(window.inner_size()),
                    None => Err(EventLoopError::WindowMissing),
                })
                .unwrap();
        }
        EventLoopEvent::Exit => *control_flow = ControlFlow::Exit,
    }
}
