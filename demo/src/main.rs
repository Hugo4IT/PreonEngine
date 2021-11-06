use std::any::Any;

use preon_engine::{
    components::{
        AddHBox, AddPanel, AddVBox, PreonComponent, PreonComponentBuilder,
        PreonComponentRenderStage, PreonComponentStack, PreonCustomComponentStack,
    },
    events::{PreonEvent, PreonUserEvent},
    rendering::PreonRenderPass,
    types::{PreonBorder, PreonColor},
    PreonEngine,
};
use preon_module_wgpu::preon;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum MyComponentStack {}

impl PreonCustomComponentStack for MyComponentStack {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(_: &mut PreonComponent<T>) {}

    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        _: PreonComponentRenderStage,
        _: &mut PreonComponent<T>,
        _: &mut PreonRenderPass,
    ) {
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut first_panel: Vec<usize> = Vec::new();
    let mut panel_list: Vec<usize> = Vec::new();

    #[rustfmt::skip]
    preon::run(PreonEngine::<MyComponentStack>::new(
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
                            .panel_color("#c4c4c4")
                            .with_min_size(0, 48)
                            .expand_horizontally()
                            .store_path(&mut first_panel)
                        .end()
                        .store_path(&mut panel_list)
                    .end()
                .end()
                .start_panel()
                    .panel_color("#d3d3d3")
                    .expand()
                .end()
            .end()
        .build()
    ), move |tree, event, user_events| match event {
        PreonEvent::WindowOpened => println!("Over the hills far away, Ferris came to play!"),
        PreonEvent::WindowResized( _new_size ) => {
            let mut panel = tree.get_child_recursive(&first_panel);
            let list = tree.get_child_ref_mut_recursive(&panel_list);

            let new_component = PreonComponentBuilder::new_from(PreonComponentStack::Panel {
                color: PreonColor::from_hex("#da0037")
            })
                .with_min_size(0, 48)
                .expand_horizontally()
                .build();

            list.insert_child(0, new_component);

            if let PreonComponentStack::Panel { ref mut color } = panel.data {
                *color = PreonColor::from_rgba(rng.gen(), rng.gen(), rng.gen(), 1.0);
            }

            tree.validate(&mut first_panel);
            tree.return_child_recursive(panel, &first_panel);

            user_events.push(PreonUserEvent::ForceUpdate);
        }
        PreonEvent::WindowClosed => println!("Then he died..."),
        _ => {},
    });
}
