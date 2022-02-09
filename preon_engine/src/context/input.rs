use winit::dpi::PhysicalPosition;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MouseButtonState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MouseState {
    pub button: [MouseButtonState; 3],
    pub position: PhysicalPosition<f64>,
}

impl MouseState {
    #[inline]
    pub fn get_button(&self, index: MouseButton) -> MouseButtonState {
        self.button[index as usize]
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PreonContextInput {
    pub mouse: MouseState
}

impl PreonContextInput {
    pub fn reset(&mut self) {
        for i in 0..3 {
            self.mouse.button[i].just_pressed = false;
            self.mouse.button[i].just_released = false;
        }
    }
}