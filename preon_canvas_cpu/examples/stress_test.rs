use std::time::Instant;

use minifb::{WindowOptions, Window};
use preon_canvas_cpu::Renderer;
use preon_engine::loaders::image::tga::TGAImageFile;

fn main() {
    let mut window = Window::new("Stress Test - PreonEngine CPU Backend Demo", 640, 480, WindowOptions::default()).unwrap();
    let mut buffer = vec![0u32; 640 * 480];

    let gradient_image = TGAImageFile::from_bytes(include_bytes!("./res/gradient.tga")).unwrap();

    let mut canvas = Renderer::new(buffer.as_mut_ptr() as *mut u8, 640, 480);
    let mono_regular = canvas.add_font(include_bytes!("./res/fonts/JetBrainsMono/JetBrains Mono Regular Nerd Font Complete Mono.ttf"));
    
    let mut x: f32 = 0.0;
    let mut last_frame_time = 0.0;
    while window.is_open() {
        let start_time = Instant::now();

        canvas.clear();

        for i in 0..1000 {
            canvas.blit_image_blend(
                (320.0 + libm::sinf(x.to_radians() + i as f32) * 200.0) as usize - 16,
                (240.0 + libm::cosf(x.to_radians() + i as f32) * 200.0) as usize - 16,
                &gradient_image,
            );
        }
        x += 1.0;

        let fps = if last_frame_time == 0.0 { 0 } else { (1.0 / last_frame_time) as u32 };
        canvas.draw_string(mono_regular, 0, 0, &format!("FPS: {}", fps), 16.0, 0xffd3d3d3);

        canvas.present();
        window.update_with_buffer(&buffer[..], 640, 480).unwrap();

        last_frame_time = (Instant::now() - start_time).as_secs_f64();
    }
}