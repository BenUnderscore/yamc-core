//Uses
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::{EventLoop, EventLoopWindowTarget};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, NotCurrent, RawContext};
use std::sync::mpsc;
use thiserror::Error;

//WINDOW MANAGER EVENT DEFINITION
#[derive(Debug)]
enum WindowManagerEvent {
    CreateWindowedContext {
        builder: WindowBuilder,
        response_tx: mpsc::Sender<Result<RawContext<NotCurrent>>>,
    },
}

type Result<T> = std::result::Result<T, EventLoopError>;

#[derive(Error, Debug)]
pub enum EventLoopError {
    #[error("A window was already created")]
    WindowExists,
    #[error(transparent)]
    GlutinCreationError(#[from] glutin::CreationError),
}

//WINDOW MANAGER PROXY DEFINITION
pub struct EventLoopProxy {
    el_proxy: glutin::event_loop::EventLoopProxy<WindowManagerEvent>,
}

impl EventLoopProxy {
    pub fn create_windowed_context(&self, wb: WindowBuilder) -> Result<RawContext<NotCurrent>> {
        let (tx, rx) = mpsc::channel();

        let event = WindowManagerEvent::CreateWindowedContext {
            builder: wb,
            response_tx: tx,
        };
        self.el_proxy.send_event(event).unwrap();

        rx.recv().unwrap()
    }
}

//EVENT LOOP DEFINITION
struct EventLoopContext {
    main_window: Option<Window>,
}

//Hijacks the calling thread (must be the main thread)
//The main thread becomes the event loop of the application
//To interact with the event loop a proxy is sent through the channel
pub fn run_event_loop(proxy_tx: mpsc::Sender<EventLoopProxy>) -> ! {
    let event_loop = EventLoop::<WindowManagerEvent>::with_user_event();
    let proxy = event_loop.create_proxy();
    proxy_tx.send(EventLoopProxy { el_proxy: proxy }).unwrap();
    let mut ctx = EventLoopContext { main_window: None };

    event_loop.run(move |ev, target, control_flow| {
        match ev {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::UserEvent(user_ev) => {
                handle_window_manager_event(&mut ctx, user_ev, target, control_flow)
            }
            _ => (),
        };
    });
}

fn handle_window_manager_event(
    ctx: &mut EventLoopContext,
    user_ev: WindowManagerEvent,
    target: &EventLoopWindowTarget<WindowManagerEvent>,
    _control_flow: &mut ControlFlow,
) {
    match user_ev {
        WindowManagerEvent::CreateWindowedContext {
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
    }
}
