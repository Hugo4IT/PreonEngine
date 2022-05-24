use std::collections::HashMap;

use async_trait::async_trait;
use preon_engine::{canvas::Canvas, renderer::Renderer, transform::{Size, Position}};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowId, Window, WindowBuilder}, event::{Event, WindowEvent}, dpi::{PhysicalPosition, PhysicalSize}};

#[derive(Debug)]
pub struct WGPU {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Window>,
    titles: HashMap<WindowId, String>,
}

impl WGPU {
    pub fn new() -> WGPU {
        WGPU {
            event_loop: EventLoop::new(),
            windows: HashMap::new(),
            titles: HashMap::new(),
        }
    }
}

#[async_trait]
impl Renderer for WGPU {
    type Handle = WindowId;

    fn new_window(&mut self) -> Self::Handle {
        let window = WindowBuilder::new().build(&self.event_loop).unwrap();
        let window_id = window.id();

        self.windows.insert(window_id, window);

        window_id
    }

    fn set_window_title(&mut self, handle: Self::Handle, title: &str) {
        self.windows
            .get_mut(&handle)
            .expect("set_window_title was called on a non-existant window.")
            .set_title(title);
    }

    fn set_window_position(&mut self, handle: Self::Handle, position: Position) {
        self.windows
            .get_mut(&handle)
            .expect("set_window_position was called on a non-existant window.")
            .set_ime_position(PhysicalPosition::new(position.x, position.y));
    }

    fn set_window_size(&mut self, handle: Self::Handle, size: Size) {
        self.windows
            .get_mut(&handle)
            .expect("set_window_size was called on a non-existant window.")
            .set_inner_size(PhysicalSize::new(size.width, size.height));
    }

    fn get_window_title(&self, handle: Self::Handle) -> String {
        self.titles
            .get(&handle)
            .expect("get_window_title was called on a non-existant window.")
            .to_owned()
    }

    fn get_window_position(&self, handle: Self::Handle) -> Position {
        let position = self.windows
            .get(&handle)
            .expect("get_window_position was called on a non-existant window.")
            .inner_position()
            .unwrap();

        Position::from((position.x, position.y))
    }

    fn get_window_size(&self, handle: Self::Handle) -> Size {
        let size = self.windows
            .get(&handle)
            .expect("get_window_size was called on a non-existant window.")
            .inner_size();

        Size::from((size.width, size.height))
    }

    fn get_screen_size(&self, handle: Option<Self::Handle>) -> Size {
        if let Some(handle) = handle {
            if let Some(monitor) = self.windows
                .get(&handle)
                .expect("get_screen_size was called on a non-existant window.")
                .current_monitor()
            {
                let size = monitor.size();
                Size::from((size.width, size.height))
            } else {
                let size = self.windows
                    .get(&handle)
                    .expect("get_screen_size was called on a non-existant window.")
                    .primary_monitor()
                    .map_or(Default::default(), |m|m.size());
                Size::from((size.width, size.height))
            }
        } else if let Some(first_window_id) = self.windows.keys().next() {
            let size = self.windows
                .get(first_window_id)
                .unwrap()
                .inner_size();
            Size::from((size.width, size.height))
        } else {
            Size::default()
        }
    }
    
    async fn mainloop(&mut self) -> Option<preon_engine::Event> {
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window) => {
                *control_flow = ControlFlow::Wait
            }
            Event::MainEventsCleared => {
                *control_flow = ControlFlow::Wait
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if let Some(window) = self.windows.get_mut(&window_id) {
                    match event {
                        WindowEvent::CloseRequested => {
                            self.windows.remove(&window_id);
                            if self.windows.is_empty() {
                                *control_flow = ControlFlow::Exit
                            }
                        }
                        _ => (),
                    }
                } else {
                    eprintln!("Event for unrecognized window ({:?}), {:?}", window_id, event)
                }
            }
            _ => (),
        })
    }
}

impl Default for WGPU {
    fn default() -> Self {
        Self::new()
    }
}