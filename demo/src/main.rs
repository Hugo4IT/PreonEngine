use std::any::Any;

use preon_engine::{
    components::{
        PreonComponent, PreonComponentBuilder, PreonComponentRenderStage, PreonComponentStack,
        PreonCustomComponentStack,
    },
    events::PreonEvent,
    rendering::PreonRenderPass,
    types::{PreonBorder, PreonColor, PreonVector},
    PreonEngine,
};
use preon_module_wgpu::preon;

#[derive(Debug, Copy, Clone)]
pub enum MyComponentStack {
    Boi,
}

impl PreonCustomComponentStack for MyComponentStack {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(
        component: &mut PreonComponent<T>,
    ) {
        match component.data {
            PreonComponentStack::Custom(ref d) => {
                let comp: &MyComponentStack =
                    (d as &dyn Any).downcast_ref::<MyComponentStack>().unwrap();
                match comp {
                    MyComponentStack::Boi => {
                        println!("Handle 'Boi' layout")
                    }
                }
            }
            _ => {}
        }
    }

    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        stage: PreonComponentRenderStage,
        _component: &mut PreonComponent<T>,
        _pass: &mut PreonRenderPass,
    ) {
        match stage {
            PreonComponentRenderStage::Background { .. } => { /* Render here */ }
            PreonComponentRenderStage::Border { .. } => { /* Render here */ }
            PreonComponentRenderStage::Foreground { .. } => { /* Render here */ }
        }
    }
}

fn main() {
    preon::run(
        PreonEngine::<MyComponentStack>::new(
            PreonComponentBuilder::new(PreonComponentStack::vbox_default())
                .with_padding(PreonBorder::from_single(16))
                .with_child(
                    PreonComponentBuilder::new(PreonComponentStack::hbox_default())
                        .expand_horizontally()
                        .fit_children_vertically()
                        .with_child(
                            PreonComponentBuilder::new(PreonComponentStack::RectComponent {
                                color: PreonColor::from_hex("#444"),
                            })
                            .with_min_size(PreonVector::new(300, 32))
                            .with_margin(PreonBorder::new(0, 16, 0, 0))
                            .build(),
                        )
                        .with_child(
                            PreonComponentBuilder::new(PreonComponentStack::RectComponent {
                                color: PreonColor::from_hex("#333"),
                            })
                            .with_min_size(PreonVector::new(0, 32))
                            .expand_horizontally()
                            .build(),
                        )
                        .build(),
                )
                .with_child(
                    PreonComponentBuilder::new(PreonComponentStack::hbox_default())
                        .with_margin(PreonBorder::new(16, 0, 0, 0))
                        .expand()
                        .with_child(
                            PreonComponentBuilder::new(PreonComponentStack::RectComponent {
                                color: PreonColor::from_hex("#333"),
                            })
                            .with_min_size(PreonVector::new(300, 0))
                            .with_margin(PreonBorder::new(0, 16, 0, 0))
                            .expand_vertically()
                            .build(),
                        )
                        .with_child(
                            PreonComponentBuilder::new(PreonComponentStack::RectComponent {
                                color: PreonColor::from_hex("#222"),
                            })
                            .expand()
                            .build(),
                        )
                        .build(),
                )
                .build(),
        ),
        |e| match e {
            PreonEvent::WindowClosed => {
                println!("Press F to pay respect for a lost fellow");
            }
            PreonEvent::Button(id, state) => {
                println!("Button {} fired event {}", id, state);
            }
            _ => {}
        },
    );
}
