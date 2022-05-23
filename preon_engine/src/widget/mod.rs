pub mod defaults;

use std::any::Any;

use crate::canvas::{SubCanvas, Canvas};

pub trait Widget<C: Canvas>: Sized + 'static {
    fn view(&mut self, canvas: SubCanvas<C>);
    fn _view(mut data: Box<dyn Any>, canvas: SubCanvas<C>) {
        #[cfg(debug_assertions)]
        {
            Self::view(data.downcast_mut().unwrap(), canvas)
        }
        
        #[cfg(not(debug_assertions))]
        Self::view(unsafe { data.downcast_mut_unchecked() }, canvas)
    }
}

pub struct WidgetHolder {
    id: usize,
    impl_id: usize,
}

pub struct WidgetImplementation<C: Canvas> {
    view_fn: fn(Box<dyn Any>, SubCanvas<C>),
}

impl<C: Canvas> WidgetImplementation<C> {
    pub fn from_type<W: Widget<C>>() -> WidgetImplementation<C> {
        WidgetImplementation {
            view_fn: W::_view,
        }
    }
}