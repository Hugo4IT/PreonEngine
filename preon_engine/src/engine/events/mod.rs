pub struct PreonEvent<T: Clone + Copy + Sized> {
    handlers: Vec<Box<dyn Fn(&mut dyn Sync, T)>>,
}

impl<T: Clone + Copy + Sized> PreonEvent<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn emit(&mut self, data: T) {
        for handler in self.handlers.iter() {
            handler(handler., data);
        }
    }

    pub fn subscribe(&mut self, target: &Box<dyn Clone>>, handler: Box<dyn Fn(&mut dyn Sized, T)>) {
        self.handlers.push(handler);
    }
}