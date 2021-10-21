use std::{
    ffi::{c_void, CString},
    mem::size_of,
    os::raw,
    ptr,
    sync::mpsc::Receiver,
};

use gl::types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr};
use glfw::{
    ClientApiHint, Context, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint,
};
use preon_engine::{
    events::{PreonEvent, PreonEventEmitter},
    rendering::{PreonRenderPass, PreonShape},
    types::PreonColor,
    PreonRenderer,
};

const RECT_VERTEX_SOURCE: &str = r#"#version 330
layout (location = 0) in vec3 pos;
uniform mat4 transform;
void main()
{
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}"#;

const RECT_FRAGMENT_SOURCE: &str = r#"#version 330
out vec4 FragColor;
uniform vec4 color;
void main()
{
    FragColor = color;
}"#;

fn make_cstr(input: &'static str) -> *const raw::c_char {
    CString::new(input.as_bytes())
        .unwrap()
        .as_bytes_with_nul()
        .as_ptr() as *const raw::c_char
}

pub struct PreonRendererOpenGL {
    window: Window,
    events: Receiver<(f64, WindowEvent)>,

    rect_vbo: u32,
    rect_vao: u32,
    rect_ebo: u32,
    rect_shader: u32,
    rect_uniform_color: i32,
    rect_uniform_transform: i32,
}

impl PreonRendererOpenGL {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
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

            let PreonColor { r, g, b, a, .. } = PreonColor::from_hex("#da0037ff");

            gl::ClearColor(r, g, b, a);
            gl::Viewport(0, 0, 640, 480);

            let (vbo, vao, ebo) = {
                let vertices: [f32; 12] =
                    [1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];

                let indices = [0, 1, 3, 1, 2, 3];

                let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
                gl::GenVertexArrays(1, &mut vao);
                gl::GenBuffers(1, &mut vbo);
                gl::GenBuffers(1, &mut ebo);

                gl::BindVertexArray(vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                    &vertices[0] as *const f32 as *const c_void,
                    gl::STATIC_DRAW,
                );
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                    &indices[0] as *const i32 as *const c_void,
                    gl::STATIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * size_of::<GLfloat>()) as GLsizei,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);

                (vbo, vao, ebo)
            };

            let (shader, uniform_color, uniform_transform) = {
                let vertex = gl::CreateShader(gl::VERTEX_SHADER);
                let c_vert_src = CString::new(RECT_VERTEX_SOURCE.as_bytes()).unwrap();
                gl::ShaderSource(vertex, 1, &c_vert_src.as_ptr(), std::ptr::null());
                gl::CompileShader(vertex);
                let mut success = gl::FALSE as GLint;
                let mut log = Vec::with_capacity(512);
                log.set_len(512 - 1);
                gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        vertex,
                        512,
                        ptr::null_mut(),
                        log.as_mut_ptr() as *mut GLchar,
                    );
                    println!("{}", std::str::from_utf8(&log).unwrap());
                }

                let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
                let c_frag_src = CString::new(RECT_FRAGMENT_SOURCE.as_bytes()).unwrap();
                gl::ShaderSource(fragment, 1, &c_frag_src.as_ptr(), std::ptr::null());
                gl::CompileShader(fragment);
                gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        fragment,
                        512,
                        ptr::null_mut(),
                        log.as_mut_ptr() as *mut GLchar,
                    );
                    println!("{}", std::str::from_utf8(&log).unwrap());
                }

                let program = gl::CreateProgram();
                gl::AttachShader(program, vertex);
                gl::AttachShader(program, fragment);
                gl::LinkProgram(program);
                gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(
                        program,
                        512,
                        ptr::null_mut(),
                        log.as_mut_ptr() as *mut GLchar,
                    );
                    println!("{}", std::str::from_utf8(&log).unwrap());
                }

                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);

                let transform = gl::GetUniformLocation(program, make_cstr("transform"));
                let color = gl::GetUniformLocation(program, make_cstr("color"));

                (program, color, transform)
            };

            PreonRendererOpenGL {
                window,
                events,
                rect_vbo: vbo,
                rect_vao: vao,
                rect_ebo: ebo,
                rect_shader: shader,
                rect_uniform_color: uniform_color,
                rect_uniform_transform: uniform_transform,
            }
        }
    }
}

impl PreonRenderer for PreonRendererOpenGL {
    fn start(&mut self) {
        self.window.show();
    }

    fn update(&mut self, events: &mut PreonEventEmitter) -> bool {
        self.window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        events.pull(|event| match event {
            PreonEvent::WindowResized { new_size } => {
                self.window.set_size(new_size.x, new_size.y);
            }
            _ => {}
        });

        !self.window.should_close()
    }

    fn render(&mut self, render_pass: &mut PreonRenderPass) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        render_pass.pull(|shape| match shape {
            PreonShape::Rect {
                position,
                size,
                color,
            } => unsafe {
                gl::UseProgram(self.rect_shader);
                gl::BindVertexArray(self.rect_vao);

                // gl::UniformMatrix4fv(
                //     self.rect_uniform_transform,
                //     1,
                //     gl::TRUE,
                //     [
                //         1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32,
                //         0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
                //     ]
                //     .as_ptr(),
                // );
                gl::Uniform4f(self.rect_uniform_color, color.r, color.g, color.b, color.a);

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                // println!("{}", color);
            },
            _ => {}
        });

        self.window.swap_buffers();
    }
}

impl Drop for PreonRendererOpenGL {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.rect_shader);
            gl::DeleteBuffers(2, [self.rect_vbo, self.rect_ebo].as_ptr());
            gl::DeleteVertexArrays(1, [self.rect_vao].as_ptr());
        }
    }
}
