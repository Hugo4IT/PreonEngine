pub mod defaults;

use std::any::Any;

use crate::canvas::Canvas;

pub trait Widget: Sized + 'static {
    fn view(&mut self, canvas: &mut Canvas);
    fn _view(mut data: Box<dyn Any>, canvas: &mut Canvas) {
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

pub struct WidgetImplementation {
    view_fn: fn(Box<dyn Any>, &mut Canvas),
}

impl WidgetImplementation {
    pub fn from_type<W: Widget>() -> WidgetImplementation {
        WidgetImplementation {
            view_fn: W::_view,
        }
    }
}