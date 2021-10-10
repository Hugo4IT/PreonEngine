use std::sync::mpsc::Receiver;

use glfw::{ClientApiHint, Context, SwapInterval, Window, WindowEvent, WindowHint};
use preon_engine::engine::{PreonEngine, PreonRenderer, ResizedEventData, components::PreonRect, events::PreonEventListener, types::Vector2, utils};

pub struct PreonRendererOpenGL {
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl<'a> PreonRendererOpenGL {
    pub fn new() -> Self {
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

        PreonRendererOpenGL { window, events }
    }

    pub fn on_resized(self: &'a Box<&'a mut Self>, data: ResizedEventData) {
        self.window.set_size(data.new_size.x as i32, data.new_size.y as i32);
        println!("Oh no");
    }
}

impl<'a> PreonEventListener<'a, ResizedEventData> for PreonRendererOpenGL {
    fn on_emit(&mut self, data: ResizedEventData) {
        todo!()
    }
}

impl<'a> PreonRenderer<'a> for PreonRendererOpenGL {
    fn connect_to(&'a mut self, engine: &'a mut PreonEngine) {
        engine.on_resized.subscribe(Box::new(self));
    }

    fn start(&mut self) {
        self.window.show();
    }

    fn update(&mut self) -> bool {
        self.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        !self.window.should_close()
    }

    fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.window.swap_buffers();
    }
}

#[derive(Debug)]
pub struct PreonGLRenderData {
    position: Vector2<i32>,
    size: Vector2<u32>,
}

trait PreonRenderableComponent<PreonRendererOpenGL> {
    fn render(&self, data: PreonGLRenderData);
}

impl PreonRenderableComponent<PreonRendererOpenGL> for PreonRect {
    fn render(&self, data: PreonGLRenderData) {
        println!(
            "Trying to render a PreonRect with layout: {:?}, color: {:?} and data: {:?}",
            self.layout, self.color, data
        );
    }
}
