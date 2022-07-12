use alloc::string::String;

use crate::transform::{Size, Position};

pub trait WindowSystem {
    type Handle: Copy;
    fn new_window(&mut self) -> Self::Handle;
    fn set_window_title(&mut self, handle: Self::Handle, title: &str);
    fn set_window_position(&mut self, handle: Self::Handle, position: Position);
    fn set_window_size(&mut self, handle: Self::Handle, size: Size);
    fn get_window_title(&self, handle: Self::Handle) -> String;
    fn get_window_position(&self, handle: Self::Handle) -> Position;
    fn get_window_size(&self, handle: Self::Handle) -> Size;
    fn get_screen_size(&self, handle: Option<Self::Handle>) -> Size;

    fn center_window(&mut self, handle: Self::Handle) {
        let screen_size = self.get_screen_size(Some(handle));
        let window_size = self.get_window_size(handle);
        self.set_window_position(
            handle,
            (
                screen_size.width * 0.5 - window_size.width * 0.5,
                screen_size.height * 0.5 - window_size.height * 0.5,
            ).into(),
        );
    }
}