use crate::types::PreonVector;

pub enum PreonEvent {
    WindowResized {
        new_size: PreonVector<i32>
    },
    WindowClosed,
    Button {},
}

pub struct PreonEventEmitter {
    events: Vec<PreonEvent>,
    buffer: Vec<PreonEvent>,
}

impl PreonEventEmitter {
    pub fn new() -> PreonEventEmitter {
        PreonEventEmitter {
            events: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, event: PreonEvent) {
        self.buffer.push(event);
    }

    pub fn pull<F: FnMut(&PreonEvent)>(&mut self, mut handler: F) {
        let mut events = self.events.iter();
        while let Some(item) = events.next() {
            handler(item);
        }
    }

    pub fn flip(&mut self) {
        self.events.clear();
        for item in self.buffer.drain(..) {
            self.events.push(item);
        }
    }
}
