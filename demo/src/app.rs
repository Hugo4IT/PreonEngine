use preon_engine::{prelude::*, style::{PreonStyle, PreonBackground}};
use preon_module_wgpu::preon;
use rand::Rng;

pub fn app() {
    env_logger::init();

    let mut rng = rand::thread_rng();
    let mut first_panel: Vec<usize> = Vec::new();
    let mut panel_list: Vec<usize> = Vec::new();
    let mut label: Vec<usize> = Vec::new();

    #[rustfmt::skip]
    let engine: PreonEngine = PreonEngine::new(
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
                        .start_label("Such art.".to_string())
                            .with_min_size(0, 200)
                            .expand_horizontally()
                            .store_path(&mut label)
                        .end()
                        .store_path(&mut panel_list)
                    .end()
                .end()
                .start_panel_hex("#d3d3d3")
                    .expand()
                    .start_vbox()
                        .expand_horizontally()
                        .start_label(format!("Size of PreonComponent: {}", std::mem::size_of::<PreonComponent>()))
                            .expand_horizontally()
                            .with_min_size(0, 20)
                            .bold()
                        .end()
                        .start_vbox()
                            .style(PreonStyle {
                                background: PreonBackground::Color(PreonColor::from_hex("#da0037")),
                                margin: PreonBorder::from_single(10),
                                padding: PreonBorder::from_single(10),
                                ..Default::default()
                            })
                            .start_label_str("Label 1").with_min_size(200, 20).end()
                            .start_label_str("Label 2").with_min_size(200, 20).end()
                            .start_label_str("Label 3").with_min_size(200, 20).end()
                            .start_label_str("Label 4").with_min_size(200, 20).end()
                            .start_label_str("Label 5").with_min_size(200, 20).end()
                        .end()
                    .end()
                .end()
            .end()
        .build(),
    );

    preon::run(engine, move |tree, event, user_events| match event {
        PreonEvent::WindowOpened => {
            println!("Over the hills far away, Ferris came to play!");

            tree.get_child_ref_mut_recursive(&label).text = "Poggers".to_string();

            let list = tree.get_child_ref_mut_recursive(&panel_list);
            let new_component = PreonComponentBuilder::new()
            .style(PreonStyle { foreground_color: PreonColor::from_hex("#da0037"), ..Default::default() })
            .with_min_size(0, 48)
            .expand_horizontally()
            .build();


            list.insert_child(0, new_component);

            // Update path after inserting new child
            tree.validate(&mut first_panel);
            tree.validate(&mut label);

            tree.get_child_ref_mut_recursive(&first_panel).style.background = PreonBackground::Color(
                    PreonColor::from_rgba(rng.gen(), rng.gen(), rng.gen(), 1.0));


            user_events.push(PreonUserEvent::ForceUpdate);
        }
        PreonEvent::WindowResized(_) => println!("RESIZE"),
        PreonEvent::WindowClosed => println!("Then he died..."),
        _ => {}
    });
}
