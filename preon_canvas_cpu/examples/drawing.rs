use std::time::Duration;

use minifb::{WindowOptions, Window};
use preon_canvas_cpu::Renderer;
use preon_engine::loaders::image::tga::TGAImageFile;

fn main() {
    let image = TGAImageFile::from_bytes(include_bytes!("./res/gradient.tga")).unwrap();

    let mut window = Window::new("Drawing - PreonEngine CPU Backend Demo", 640, 480, WindowOptions::default()).unwrap();
    let mut buffer = vec![0u32; 640 * 480];
    window.limit_update_rate(Some(Duration::from_secs_f32(0.01666667)));

    let mut canvas = Renderer::new(buffer.as_mut_ptr() as *mut u8, 640, 480);
    canvas.clear_color = 0xff171717;

    let mono_regular = canvas.add_font(include_bytes!("./res/fonts/JetBrainsMono/JetBrains Mono Regular Nerd Font Complete Mono.ttf"));
    
    while window.is_open() {
        canvas.clear();
        
        canvas.fill_rect(10, 10, 32, 32, 0xffda0037);
        canvas.draw_string(mono_regular, 52, 11, "fill_rect(32x32, #da0037)", 24.0, 0xffd3d3d3);
        
        canvas.blit_image(10, 52, &image);
        canvas.draw_string(mono_regular, 52, 53, "blit_image(gradient.tga)", 24.0, 0xffd3d3d3);
        
        canvas.blit_image_blend(10, 94, &image);
        canvas.draw_string(mono_regular, 52, 95, "blit_image_blend(gradient.tga)", 24.0, 0xffd3d3d3);

        canvas.present();
        window.update_with_buffer(&buffer[..], 640, 480).unwrap();
    }
}