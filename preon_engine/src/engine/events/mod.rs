use std::ops::AddAssign;

pub trait PreonEventReceiver<T: Copy + Clone + Sized> {
    fn received(&mut self, data: T);
}

pub struct PreonEvent<T: Copy + Clone + Sized> {
    handlers: Vec<Box<dyn PreonEventReceiver<T>>>,
}

impl<T: Copy + Clone + Sized> AddAssign<Box<dyn PreonEventReceiver<T>>> for PreonEvent<T> {
    fn add_assign(&mut self, rhs: Box<dyn PreonEventReceiver<T>>) {
        self.subscribe(rhs);
    }
}

impl<T: Copy + Clone + Sized> PreonEvent<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn fire(&mut self, data: T) {
        for handler in self.handlers.iter_mut() {
            handler.received(data);
        }
    }

    pub fn subscribe(&mut self, handler: Box<dyn PreonEventReceiver<T>>) {
        self.handlers.push(handler);
    }
}
