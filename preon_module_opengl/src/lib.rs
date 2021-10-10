use std::sync::mpsc::Receiver;

use glfw::{ClientApiHint, Context, SwapInterval, Window, WindowEvent, WindowHint};
use preon_engine::{
    components::PreonRect,
    events::{PreonEvent, WindowEventArgs},
    pipeline::PreonRenderPipeline,
    types::Vector2,
    utils, PreonComponent, PreonEngine, PreonRenderer,
};

pub struct PreonRendererOpenGL {
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    registered_handlers: Vec<usize>,
}

impl PreonRendererOpenGL {
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

        PreonRendererOpenGL {
            window,
            events,
            registered_handlers: Vec::new(),
        }
    }
}

impl PreonRenderer for PreonRendererOpenGL {
    fn start(&mut self) {
        self.window.show();
    }

    fn update(&mut self, engine: &mut PreonEngine) -> bool {
        self.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        engine.events.pull(|event| match event {
            PreonEvent::Window(WindowEventArgs::Resized { new_size }) => {
                self.window.set_size(new_size.x, new_size.y);
            }
            _ => {}
        });

        !self.window.should_close()
    }

    fn render(&mut self, engine: &mut PreonEngine) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        recursive_render(engine.root.get_pipeline());

        self.window.swap_buffers();
    }
}

fn recursive_render(pipeline: Option<PreonRenderPipeline>) {
    if pipeline.is_some() {
        let peepline = pipeline.unwrap();

        if peepline.drawables.is_some() {
            let mut drawweebbles = peepline.drawables.unwrap();
            for _drawable in drawweebbles.iter_mut() {}
        }
    }
}

trait PreonRenderable<PreonRendererOpenGL> {
    fn render(&self, position: Vector2<i32>, size: Vector2<i32>);
}

impl PreonRenderable<PreonRendererOpenGL> for PreonRect {
    fn render(&self, _position: Vector2<i32>, _size: Vector2<i32>) {
        todo!()
    }
}
