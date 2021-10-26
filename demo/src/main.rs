use preon_engine::{PreonEngine, components::{PreonComponent, PreonComponentStack, PreonCustomComponentStack}, events::PreonEvent, size, types::{PreonBorder, PreonBox, PreonColor, PreonVector}};
use preon_module_wgpu::preon;

#[derive(Debug, Copy, Clone)]
pub enum MyComponentStack {
}

impl PreonCustomComponentStack for MyComponentStack {
    fn custom_layout(component: &mut Self) {
        
    }
}

fn main() {
    preon::run(PreonEngine::<MyComponentStack>::new(
        PreonComponent {
            data: PreonComponentStack::VBoxComponent,
            model: PreonBox {
                margin: PreonBorder::from_single(8),
                padding: PreonBorder::from_xy(16, 8),
                border: PreonBorder::zero(),
                size_flags: size::FIT,
                min_size: PreonVector::new(320, 240),
            },
            children: Some(vec![
                PreonComponent {
                    children: None,
                    data: PreonComponentStack::RectComponent {
                        color: PreonColor::from_hex("#da0037"),
                    },
                    model: PreonBox::initial(),
                    ..Default::default()
                }
            ]),
            ..Default::default()
        },
    ), |e| match e {
        PreonEvent::WindowClosed => {
            println!("Press F to pay respect for a lost fellow");
        },
        PreonEvent::Button(id, state) => {
            println!("Button {} fired event {}", id, state);
        },
        _ => {}
    });
}