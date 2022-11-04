use self::window_system::WindowSystem;

pub mod window_system;

pub trait Renderer {
    type WindowSystem: WindowSystem;
    
}