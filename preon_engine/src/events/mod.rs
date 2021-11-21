use std::fmt::Display;

use crate::types::PreonVector;

#[derive(Debug, Clone)]
pub enum PreonButtonState {
    MouseEnter,
    MouseExit,
    MouseDown,
    MouseUp,
    Pressed,
}

impl Display for PreonButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PreonButtonState::MouseEnter => "MouseEnter",
                PreonButtonState::MouseExit => "MouseExit",
                PreonButtonState::MouseDown => "MouseDown",
                PreonButtonState::MouseUp => "MouseUp",
                PreonButtonState::Pressed => "Pressed",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum PreonEvent {
    WindowResized(PreonVector<u32>),
    WindowOpened,
    WindowClosed,
    Update,
    LayoutUpdate,
    Button(u32, PreonButtonState),
}

#[derive(Debug, Clone)]
pub enum PreonUserEvent {
    WindowResized(PreonVector<u32>),
    WindowOpened,
    WindowClosed,
    MouseMove(PreonVector<i32>),
    ForceUpdate,
}

#[derive(Debug)]
pub struct PreonEventEmitter<T: Clone> {
    events: Vec<T>,
    buffer: Vec<T>,
}

#[allow(clippy::new_without_default)]
impl<T: Clone> PreonEventEmitter<T> {
    pub fn new() -> PreonEventEmitter<T> {
        PreonEventEmitter {
            events: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, event: T) {
        self.buffer.push(event);
    }

    pub fn pull<F: FnMut(T)>(&self, mut handler: F) {
        for item in self.events.iter() {
            handler(item.clone());
        }
    }

    pub fn flip(&mut self) {
        self.events = self.buffer.drain(..).collect();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    #[inline(always)]
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
}
