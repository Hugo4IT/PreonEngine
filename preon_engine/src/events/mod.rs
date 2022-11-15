use alloc::vec::Vec;

use core::fmt::Display;

use crate::{types::PreonVector, components::PreonComponent};

#[derive(Debug, Clone)]
pub enum PreonButtonState {
    MouseEnter,
    MouseExit,
    MouseDown,
    MouseUp,
    Pressed,
}

impl Display for PreonButtonState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// Contains a front- and backbuffer> Events get pushed onto
/// the backbuffer and pulled from the frontbuffer. Use [`PreonEventEmitter::pull`]
/// to swap the front- and backbuffer.
#[derive(Debug)]
pub struct PreonEventEmitter<T: Clone> {
    buffers: [Vec<T>; 2],
    current_buffer: usize,
}

#[allow(clippy::new_without_default)]
impl<T: Clone> PreonEventEmitter<T> {
    pub fn new() -> PreonEventEmitter<T> {
        PreonEventEmitter {
            buffers: [Vec::new(), Vec::new()],
            current_buffer: 0,
        }
    }

    pub fn push(&mut self, event: T) {
        self.backbuffer_mut().push(event);
    }

    pub fn take(&self) -> Vec<T> {
        self.buffer().clone()
    }

    pub fn flip(&mut self) {
        self.current_buffer = 1 - self.current_buffer;
        self.backbuffer_mut().clear();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.buffer().len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.buffer().is_empty()
    }

    #[inline(always)]
    pub fn buffer_len(&self) -> usize {
        self.backbuffer().len()
    }

    #[inline(always)]
    fn buffer(&self) -> &Vec<T> {
        &self.buffers[self.current_buffer]
    }

    #[inline(always)]
    fn backbuffer(&self) -> &Vec<T> {
        &self.buffers[1 - self.current_buffer]
    }

    #[inline(always)]
    fn buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffers[self.current_buffer]
    }

    #[inline(always)]
    fn backbuffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffers[1 - self.current_buffer]
    }
}
