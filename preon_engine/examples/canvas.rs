use preon_engine::{canvas::{BufferedCanvas, Canvas, color::Color}, transform::{Rect, Position, Size}};

fn main() {
    let mut canvas = BufferedCanvas::new();

    let mut sub_canvas = canvas.derive(((30, 20), (100, 100)).into());
    sub_canvas.fill_rect(Rect((70, 80).into(), (320, 240).into()), Color::from_rgb8(0xda, 0x00, 0x37));

    let mut sub_canvas = canvas.derive(((30, 20), (200, 200)).into());
    sub_canvas.fill_rounded_rect(((10, 10), (170, 170)).into(), Color::from_rgb8(0xda, 0x00, 0x37), 16.0);

    let instructions = canvas.end();
    println!("{:#?}", instructions);
}