use log;
use thiserror::Error;
use winit::dpi::{LogicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to create application")]
    FailedToCreateApplication,
}

pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub struct WindowDescriptor {
    pub size: WindowSize,
    pub title: String,
}

pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let window_descriptor = WindowDescriptor {
            size: WindowSize {
                width: 800,
                height: 600,
            },
            title: "Some Basic Application".to_string(),
        };
        let event_loop = EventLoop::new();
        let window = Self::create_window(&event_loop, window_descriptor);
        let application = Self { event_loop, window };
        Ok(application)
    }

    fn create_window(event_loop: &EventLoop<()>, window_descriptor: WindowDescriptor) -> Window {
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(
                window_descriptor.size.width,
                window_descriptor.size.height,
            ))
            .with_title(window_descriptor.title)
            .build(&event_loop)
            .unwrap();
        window
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            log::info!("{:?}", event);
            *control_flow = winit::event_loop::ControlFlow::Wait;
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
