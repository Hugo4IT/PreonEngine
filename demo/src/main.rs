use std::any::Any;

use preon_engine::{PreonEngine, components::{PreonComponent, PreonComponentRenderStage, PreonComponentStack, PreonCustomComponentStack}, events::PreonEvent, rendering::PreonRenderPass, size, types::{PreonBorder, PreonBox, PreonColor, PreonVector}};
use preon_module_wgpu::preon;

#[derive(Debug, Copy, Clone)]
pub enum MyComponentStack {
    Boi
}

impl PreonCustomComponentStack for MyComponentStack {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(component: &mut PreonComponent<T>) {
        match component.data {
            PreonComponentStack::Custom(ref d) => {
                let comp: &MyComponentStack = (d as &dyn Any).downcast_ref::<MyComponentStack>().unwrap();
                match comp {
                    MyComponentStack::Boi => {
                        
                    },
                }
            },
            _ => {}
        }
    }

    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(stage: PreonComponentRenderStage, component: &mut PreonComponent<T>, pass: &mut PreonRenderPass) {
        match stage {
            _ => {}
        }
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
                min_size: PreonVector::new(640, 480),
            },
            children: Some(vec![
                PreonComponent {
                    children: None,
                    data: PreonComponentStack::RectComponent {
                        color: PreonColor::from_hex("#87CA3C"),
                    },
                    model: PreonBox {
                        margin: PreonBorder::zero(),
                        padding: PreonBorder::from_xy(160, 8),
                        border: PreonBorder::zero(),
                        size_flags: size::FIT,
                        min_size: PreonVector::new(120, 60)
                    },
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