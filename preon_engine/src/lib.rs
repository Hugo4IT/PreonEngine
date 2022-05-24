use std::any::Any;

use canvas::Canvas;
use futures_util::task::AtomicWaker;
use renderer::Renderer;
use widget::{Widget, WidgetHolder, WidgetImplementation};

pub mod transform;
pub mod renderer;
pub mod widget;
pub mod canvas;

pub enum Event {
    WindowClose,
}

pub struct App<R: Renderer> {
    renderer: R,
    widgets: Vec<WidgetHolder>,
    widgets_data: Vec<Box<dyn Send + Any>>,
    widgets_impl: Vec<WidgetImplementation>,
}

impl<R: Renderer> App<R> {
    pub fn new(renderer: R) -> App<R> {
        App {
            renderer,
            widgets: Vec::new(),
            widgets_data: Vec::new(),
            widgets_impl: Vec::new(),
        }
    }

    pub fn add_widget<W: Widget>(&mut self, widget: W) {
        
    }

    pub fn add_widget_impl<W: Widget>(&mut self) {
        self.widgets_impl.push(WidgetImplementation::from_type::<W>())
    }

    pub fn start(mut self) {
        let mut canvas = Canvas::new();
        
    }
}