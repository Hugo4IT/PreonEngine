pub struct PreonEvent<T: Copy + Clone + Sized> {
    handlers: Vec<fn(T)>,
}

impl<T: Copy + Clone + Sized> PreonEvent<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn fire(&self, args: T) {
        for handler in self.handlers.iter() {
            handler(args);
        }
    }

    pub fn subscribe(&mut self, handler: fn(T)) {
        self.handlers.push(handler);
    }
}
