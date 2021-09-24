use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, platform::run_return::EventLoopExtRunReturn, window::{Window, WindowBuilder}};
use preon_core::{PreonCore, PreonRenderer};

pub struct PreonRendererWGPU {
    event_loop: EventLoop<()>,
    window: Window
}

impl PreonRenderer for PreonRendererWGPU {
    fn init(core: &PreonCore) -> Self {
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("PreonEngine Window")
            .build(&event_loop)
            .unwrap();

        PreonRendererWGPU { event_loop, window }
    }

    fn start(&mut self, core: &PreonCore) {
        let window = &self.window;

        self.event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if (window_id == window.id()) => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }

    fn update(&mut self, core: &PreonCore) {
        
    }

    fn render(&mut self, core: &PreonCore) {
        
    }
}