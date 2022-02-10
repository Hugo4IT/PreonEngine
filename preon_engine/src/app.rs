use std::collections::HashMap;

use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use crate::context::{PreonContext, input::MouseButton};

pub type WindowFunction<T> = fn(&mut PreonContext, &mut T);

pub struct App<T> {
    windows: Vec<(String, WindowFunction<T>)>,
    state: Option<T>,
}

impl<T: 'static> App<T> {
    pub fn new() -> App<T> {
        App {
            windows: Vec::new(),
            state: None,
        }
    }

    pub fn add_window(&mut self, title: &str, function: WindowFunction<T>) {
        self.windows.push((title.to_string(), function));
    }

    pub fn set_state(&mut self, state: T) {
        self.state = Some(state);
    }

    pub fn start(&mut self) {
        let event_loop = EventLoop::new();
        let mut windows: HashMap<WindowId, WindowHolder<T>> = HashMap::new();

        let mut state = self
            .state
            .take()
            .expect("You must supply a state with App::set_state().");

        for (title, function) in self.windows.iter() {
            let window = WindowBuilder::new()
                .with_title(title)
                .build(&event_loop)
                .unwrap();
            let context = PreonContext::new(&window);
            let window_id = window.id();
            let mut holder = WindowHolder {
                window,
                context,
                function: function.clone(),
            };

            holder.layout(&mut state);
            windows.insert(window_id, holder);
        }

        event_loop.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                for holder in windows.values_mut() {
                    holder.update(&mut state);
                }

                *control_flow = ControlFlow::Wait;
            }
            Event::RedrawRequested(id) => {
                let holder = windows.get_mut(&id).unwrap();
                holder.layout(&mut state);
                holder.render(&mut state);
            }
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    windows.remove(&window_id);

                    if windows.is_empty() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let holder = windows.get_mut(&window_id).unwrap();
                    holder.context.input.mouse.position = position;
                }
                WindowEvent::MouseInput {
                    button,
                    state,
                    ..
                } => {
                    let holder = windows.get_mut(&window_id).unwrap();
                    let index = match button {
                        winit::event::MouseButton::Left => MouseButton::Left,
                        winit::event::MouseButton::Right => MouseButton::Right,
                        winit::event::MouseButton::Middle => MouseButton::Middle,
                        winit::event::MouseButton::Other(_) => return,
                    } as usize;

                    match state {
                        winit::event::ElementState::Pressed => {
                            holder.context.input.mouse.button[index].pressed = true;
                            holder.context.input.mouse.button[index].just_pressed = true;
                        },
                        winit::event::ElementState::Released => {
                            holder.context.input.mouse.button[index].pressed = false;
                            holder.context.input.mouse.button[index].just_released = true;
                        },
                    }
                }
                WindowEvent::Resized(new_size) => {
                    let holder = windows.get_mut(&window_id).unwrap();
                    holder.context.resize(new_size);
                    holder.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    let holder = windows.get_mut(&window_id).unwrap();
                    holder.context.resize(*new_inner_size);
                    holder.request_redraw();
                }
                _ => (),
            },
            _ => (),
        });
    }
}

struct WindowHolder<T> {
    pub window: Window,
    pub context: PreonContext,
    pub function: WindowFunction<T>,
}

impl<T> WindowHolder<T> {
    #[inline]
    pub fn request_redraw(&mut self) {
        info!("Requesting redraw.");
        self.window.request_redraw();
    }

    #[inline]
    pub fn call_function(&mut self, state: &mut T) {
        self.function.clone()(&mut self.context, state);
    }

    pub fn update(&mut self, state: &mut T) {
        self.context.prepare_update();
        self.call_function(state);
        if self.context.finish_update() {
            info!("Detected changes");
            self.request_redraw();
        }
    }

    pub fn layout(&mut self, state: &mut T) {
        info!("Relayout.");

        self.context.prepare_layout();
        self.call_function(state);
        self.context.finish_layout();
    }

    pub fn render(&mut self, state: &mut T) {
        info!("Rerender.");

        self.context.prepare_render();
        self.call_function(state);
        self.context.finish_render();
    }
}