use preon::Preon;
use preon_core::{PreonRect, margin, padding_xy};

fn main () {
    let mut preon = Preon::new();

    let mut rect = PreonRect::new();
    rect.layout.padding = padding_xy(16, 8);
    rect.layout.margin = margin(8);
    preon.add_child(rect);
    
    preon.start();
}