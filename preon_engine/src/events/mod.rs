use std::fmt::Display;

use crate::types::PreonVector;

#[derive(Debug, Copy, Clone)]
pub enum PreonButtonState {
    MouseEnter,
    MouseExit,
    MouseDown,
    MouseUp,
    Pressed,
}

impl Display for PreonButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PreonButtonState::MouseEnter => "MouseEnter",
            PreonButtonState::MouseExit => "MouseExit",
            PreonButtonState::MouseDown => "MouseDown",
            PreonButtonState::MouseUp => "MouseUp",
            PreonButtonState::Pressed => "Pressed",
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PreonEvent {
    WindowOpened,
    WindowResized {
        new_size: PreonVector<i32>
    },
    WindowClosed,
    Button(u32, PreonButtonState),
}

#[derive(Debug, Copy, Clone)]
pub enum PreonUserEvent {
    MouseMove(i32, i32)
}

#[derive(Debug)]
pub struct PreonEventEmitter<T: Copy + Clone> {
    events: Vec<T>,
    buffer: Vec<T>,
}

impl<T: Copy + Clone> PreonEventEmitter<T> {
    pub fn new() -> PreonEventEmitter<T> {
        PreonEventEmitter {
            events: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn new_with_initial(initial: T) -> PreonEventEmitter<T> {
        PreonEventEmitter {
            events: vec![initial],
            buffer: Vec::new()
        }
    }

    pub fn push(&mut self, event: T) {
        self.buffer.push(event);
    }

    pub fn pull<F: FnMut(T)>(&mut self, mut handler: F) {
        let mut events = self.events.iter();
        while let Some(item) = events.next() {
            handler(*item);
        }
    }

    pub fn flip(&mut self) {
        self.events = self.buffer.drain(..).collect();
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.events.len() }
}
