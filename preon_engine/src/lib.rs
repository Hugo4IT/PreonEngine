use std::any::Any;

use canvas::Canvas;
use futures_util::task::AtomicWaker;
use renderer::Renderer;
use widget::{Widget, WidgetHolder, WidgetImplementation};

pub mod transform;
pub mod renderer;
pub mod widget;
pub mod canvas;

pub struct App<R: Renderer + Canvas> {
    renderer: R,
    widgets: Vec<WidgetHolder>,
    widgets_data: Vec<Box<dyn Send + Any>>,
    widgets_impl: Vec<WidgetImplementation<R>>,
}

impl<R: Renderer + Canvas> App<R> {
    pub fn new(canvas: R) -> App<R> {
        App {
            renderer: canvas,
            widgets: Vec::new(),
            widgets_data: Vec::new(),
            widgets_impl: Vec::new(),
        }
    }

    pub fn add_widget<W: Widget<R>>(&mut self, widget: W) {
        
    }

    pub fn add_widget_impl<W: Widget<R>>(&mut self) {
        self.widgets_impl.push(WidgetImplementation::from_type::<W>())
    }

    pub fn start(mut self) {
        let waker = AtomicWaker::new();
        // TODO: Implement a render instruction stream, for async/await functionality
    }
}