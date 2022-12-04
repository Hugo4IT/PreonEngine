use alloc::{vec::Vec, string::String};

use core::fmt::Display;

use crate::types::PreonVector;

impl Display for PreonButtonState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PreonButtonState::Pressed => "Pressed",
                PreonButtonState::Released => "Released",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PreonButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub enum PreonMouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}

#[derive(Debug, Clone)]
pub enum PreonEvent {
    WindowOpened,
    WindowResized(PreonVector<u32>),
    WindowClosed,
    Update,
    LayoutUpdate,
    ComponentPressed(String, PreonButtonState),
    MouseInput(PreonMouseButton, PreonButtonState),
    KeyboardInput(PreonKeyCode, PreonButtonState),
    ReceivedCharacter(char),
}

#[derive(Debug, Clone, Copy)]
pub enum PreonUserEvent {
    WindowOpened,
    WindowResized(PreonVector<u32>),
    WindowClosed,
    ForceUpdate,
    MouseMove(PreonVector<i32>),
    MouseInput(PreonMouseButton, PreonButtonState),
    KeyboardInput(PreonKeyCode, PreonButtonState),
    ReceivedCharacter(char),
}

/// Contains a front- and backbuffer> Events get pushed onto
/// the backbuffer and pulled from the frontbuffer. Use [`PreonEventEmitter::flip`]
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
    fn backbuffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffers[1 - self.current_buffer]
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum PreonKeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    PrintScreen,
    ScrollLock,
    PauseBreak,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Backspace,
    Return,
    Space,

    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}