use preon_engine::{
    components::{
        AddHBox, AddLabel, AddPanel, AddStaticTexture, AddVBox, NoCustomComponents, PreonComponent,
        PreonComponentBuilder, PreonComponentStack,
    },
    events::{PreonEvent, PreonUserEvent},
    preon_font,
    rendering::PreonStaticRenderData,
    types::{PreonBorder, PreonColor},
    PreonEngine,
};
use preon_module_wgpu::preon;
use rand::Rng;

pub fn app() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let mut rng = rand::thread_rng();
    let mut first_panel: Vec<usize> = Vec::new();
    let mut panel_list: Vec<usize> = Vec::new();

    #[rustfmt::skip]
    let engine: PreonEngine<NoCustomComponents> = PreonEngine::new(
        PreonStaticRenderData {
            textures: &[
                include_bytes!("../../res/mm2wood.png"),
                include_bytes!("../../res/juan.png"),
            ],
            fonts: &[
                preon_font!("../../res/Montserrat", "otf")
            ]
        },
        PreonComponentBuilder::new()
            .start_panel_hex("#da0037")
                .with_min_size(0, 60)
                .expand_horizontally()
                .start_static_texture(0)
                    .with_margin(PreonBorder::new(0, 0, -50, 0))
                    .with_min_size(200, 200)
                .end()
            .end()
            .start_hbox()
                .expand()
                .start_panel_hex("#ffffff")
                    .with_min_size(300, 0)
                    .expand_vertically()
                    .with_padding(PreonBorder::from_single(16))
                    .start_vbox()
                        .fit_children_vertically()
                        .expand_horizontally()
                        .start_panel_hex("#c4c4c4")
                            .with_min_size(0, 48)
                            .expand_horizontally()
                            .store_path(&mut first_panel)
                        .end()
                        .start_static_texture(0)
                            .with_min_size(0, 200)
                            .expand_horizontally()
                        .end()
                        .start_static_texture(1)
                            .with_min_size(0, 200)
                            .expand_horizontally()
                        .end()
                        .start_label_str("Such art.")
                            .with_min_size(0, 200)
                            .expand_horizontally()
                        .end()
                        .store_path(&mut panel_list)
                    .end()
                .end()
                .start_panel_hex("#d3d3d3")
                    .expand()
                    .start_vbox()
                        .expand_horizontally()
                        .start_label(format!("Size of PreonComponent: {}", std::mem::size_of::<PreonComponent<NoCustomComponents>>()))
                            .expand_horizontally()
                            .with_min_size(0, 200)
                            .bold()
                        .end()
                    .end()
                .end()
            .end()
        .build(),
    );

    preon::run(engine, move |tree, event, user_events| match event {
        PreonEvent::WindowOpened => {
            println!("Over the hills far away, Ferris came to play!");

            let list = tree.get_child_ref_mut_recursive(&panel_list);
            let new_component = PreonComponentBuilder::new_from(PreonComponentStack::Panel {
                color: PreonColor::from_hex("#da0037"),
            })
            .with_min_size(0, 48)
            .expand_horizontally()
            .build();

            list.insert_child(0, new_component);
            tree.validate(&mut first_panel); // Update path after inserting new child

            tree.get_child_ref_mut_recursive(&first_panel).data = PreonComponentStack::Panel {
                color: PreonColor::from_rgba(rng.gen(), rng.gen(), rng.gen(), 1.0),
            };

            user_events.push(PreonUserEvent::ForceUpdate);
        }
        PreonEvent::WindowResized(_) => println!("RESIZE"),
        PreonEvent::WindowClosed => println!("Then he died..."),
        _ => {}
    });
}
