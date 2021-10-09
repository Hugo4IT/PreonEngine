pub trait PreonEventBus {
    type Data;
    type Target;
}

pub struct PreonEvent<T: PreonEventBus> {
    handlers: Vec<fn(&mut T::Target, T::Data)>,
}

impl<T: PreonEventBus> PreonEvent<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn emit(&mut self, data: T::Data) {
        for handler in self.handlers.iter_mut() {
            handler(data);
        }
    }

    pub fn subscribe(&mut self, handler: fn(&mut T::Target, T::Data)) {
        self.handlers.push(handler);
    }
}