use std::{sync::mpsc::Receiver};

use glfw::{ClientApiHint, Context, SwapInterval, Window, WindowEvent, WindowHint};
use preon_core::{PreonCore, PreonRenderer, utils};

pub struct PreonRendererOpenGL {
    window: Window,
    events: Receiver<(f64, WindowEvent)>
}

impl PreonRendererOpenGL {
    pub fn init() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::OpenGl));
        glfw.window_hint(WindowHint::Samples(Some(4)));
        glfw.window_hint(WindowHint::Visible(false));

        #[cfg(target_os = "macos")]
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(640, 480, "PreonEngine Window", glfw::WindowMode::Windowed)
            .unwrap();
        window.set_size_polling(true);
        window.set_key_polling(true);
        window.set_resizable(true);
        window.make_current();

        glfw.set_swap_interval(SwapInterval::Sync(1));
        unsafe {
            gl::load_with(|s| window.get_proc_address(s));

            let (r, g, b, a) = utils::color(0x171717FF);

            gl::ClearColor(r, g, b, a);
        };

        PreonRendererOpenGL {
            window,
            events
        }
    }
}

impl PreonRenderer for PreonRendererOpenGL {
    fn start(&mut self, core: &PreonCore) {
        self.window.show();
    }

    fn update(&mut self, core: &mut PreonCore) -> bool {
        self.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        self.window.should_close()
    }

    fn render(&mut self, core: &PreonCore) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        

        self.window.swap_buffers();
    }
}
