use preon_canvas_wgpu::WGPU;
use preon_engine::{App, widget::defaults::color_rect::ColorRect, canvas::color::Color, renderer::Renderer};

pub fn main() {
    let mut renderer = WGPU::new();

    let window = renderer.new_window();
    renderer.set_window_title(window, "Todo App");
    renderer.center_window(window);

    let mut app = App::new(renderer);
    app.add_widget(ColorRect::new(Color::from_rgb8(0xda, 0x00, 0x37)));
    app.start();
}
