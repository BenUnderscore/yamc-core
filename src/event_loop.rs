//Uses
use glutin::event::Event;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::{EventLoop, EventLoopWindowTarget};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, NotCurrent, RawContext};
use std::sync::mpsc;
use thiserror::Error;

//WINDOW MANAGER EVENT DEFINITION
#[derive(Debug)]
enum EventLoopEvent {
    CreateWindowedContext {
        builder: WindowBuilder,
        response_tx: mpsc::Sender<Result<RawContext<NotCurrent>>>,
    },
    RegisterDeviceEventSender {
        sender: Option<mpsc::Sender<DeviceEvent>>,
    },
    RegisterWindowEventSender {
        sender: Option<mpsc::Sender<WindowEvent>>,
    },
    Exit,
}

type Result<T> = std::result::Result<T, EventLoopError>;

#[derive(Error, Debug)]
pub enum EventLoopError {
    #[error("A window was already created")]
    WindowExists,
    #[error(transparent)]
    GlutinCreationError(#[from] glutin::CreationError),
}

//EVENT LOOP PROXY DEFINITION
pub struct EventLoopProxy {
    el_proxy: glutin::event_loop::EventLoopProxy<EventLoopEvent>,
}

impl EventLoopProxy {
    pub fn create_windowed_context(&self, wb: WindowBuilder) -> Result<RawContext<NotCurrent>> {
        let (tx, rx) = mpsc::channel();

        let event = EventLoopEvent::CreateWindowedContext {
            builder: wb,
            response_tx: tx,
        };
        self.el_proxy.send_event(event).unwrap();

        rx.recv().unwrap()
    }

    pub fn register_device_event_sender(&self, tx: Option<mpsc::Sender<DeviceEvent>>) {
        let event = EventLoopEvent::RegisterDeviceEventSender { sender: tx };
        self.el_proxy.send_event(event).unwrap();
    }

    pub fn register_window_event_sender(&self, tx: Option<mpsc::Sender<WindowEvent>>) {
        let event = EventLoopEvent::RegisterWindowEventSender { sender: tx };
        self.el_proxy.send_event(event).unwrap();
    }

    pub fn exit(&self) {
        self.el_proxy.send_event(EventLoopEvent::Exit).unwrap();
    }
}

pub struct DeviceEvent {
    pub device_id: glutin::event::DeviceId,
    pub event: glutin::event::DeviceEvent,
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
            },
            Event::WindowEvent { window_id: _, event } => {
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        if let Some(tx) = &ctx.window_event_sender {
                            tx.send(WindowEvent::CloseRequested).unwrap();
                        }
                    },
                    _ => (),
                }
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
        EventLoopEvent::CreateWindowedContext {
            builder,
            response_tx,
        } => {
            response_tx
                .send({
                    if let Some(_) = &ctx.main_window {
                        Err(EventLoopError::WindowExists)
                    } else {
                        let windowed_context_result =
                            ContextBuilder::new().build_windowed(builder, target);
                        match windowed_context_result {
                            Err(e) => Err(EventLoopError::GlutinCreationError(e)),
                            Ok(windowed_context) => unsafe {
                                let (raw_ctx, window) = windowed_context.split();
                                ctx.main_window = Some(window);
                                Ok(raw_ctx)
                            },
                        }
                    }
                })
                .unwrap();
        }
        EventLoopEvent::RegisterDeviceEventSender { sender } => {
            ctx.device_event_sender = sender;
        },
        EventLoopEvent::RegisterWindowEventSender { sender } => {
            ctx.window_event_sender = sender;
        }
        EventLoopEvent::Exit => *control_flow = ControlFlow::Exit,
    }
}
