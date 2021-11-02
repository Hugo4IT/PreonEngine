use std::any::Any;

use preon_engine::{PreonEngine, components::{AddHBox, AddPanel, AddVBox, PreonComponent, PreonComponentBuilder, PreonComponentRenderStage, PreonComponentStack, PreonCustomComponentStack}, events::PreonEvent, rendering::PreonRenderPass, types::PreonBorder};
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
            PreonComponentStack::Custom(ref _d) => { /* Handle layout here */ }
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
    #[rustfmt::skip]
    preon::run(
        PreonEngine::<MyComponentStack>::new(
            PreonComponentBuilder::new()
                .start_panel()
                    .panel_color("#da0037")
                    .with_min_size(0, 60)
                    .expand_horizontally()
                .end()
                .start_hbox()
                    .expand()
                    .start_panel()
                        .panel_color("#ffffff")
                        .with_min_size(300, 0)
                        .expand_vertically()
                        .with_padding(PreonBorder::from_single(16))
                        .start_vbox()
                            .fit_children_vertically()
                            .expand_horizontally()
                            .start_panel()
                                .panel_color("#da0037")
                                .with_min_size(0, 48)
                                .expand_horizontally()
                            .end()
                        .end()
                    .end()
                    .start_panel()
                        .panel_color("#d3d3d3")
                        .expand()
                    .end()
                .end()
            .build()
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
