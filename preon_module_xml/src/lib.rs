use std::{borrow::Cow, str::FromStr};

use log::{error, info};
use preon_engine::{
    components::{AddHBox, AddPanel, AddVBox, NoCustomComponents, PreonComponentBuilder, AddLabel},
    rendering::PreonStaticRenderData,
    types::{PreonBorder, PreonColor, PreonVector, PreonVectorAble},
    PreonEngine,
};
use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader,
};

pub trait ParseXMLAttribute {
    fn parse_xml(input: String) -> Self;
}

impl<T: PreonVectorAble + FromStr> ParseXMLAttribute for PreonVector<T> {
    fn parse_xml(input: String) -> Self {
        let strings = input.split(' ').collect::<Vec<&str>>();

        if let Ok(x) = T::from_str(strings[0]) {
            if let Ok(y) = T::from_str(strings[1]) {
                Self { x, y }
            } else {
                panic!("No integer could be parsed from the given Y value")
            }
        } else {
            panic!("No integer could be parsed from the given X value")
        }
    }
}

impl ParseXMLAttribute for PreonColor {
    fn parse_xml(input: String) -> Self {
        info!("Found PreonColor({})", input);

        PreonColor::from_hex(&input)
    }
}

impl ParseXMLAttribute for PreonBorder {
    fn parse_xml(input: String) -> Self {
        let strings = input.split(' ').collect::<Vec<&str>>();

        if let (Ok(t), Ok(r), Ok(b), Ok(l)) = (
            i32::from_str(strings[0]),
            i32::from_str(strings[1]),
            i32::from_str(strings[2]),
            i32::from_str(strings[3]),
        ) {
            Self {
                top: t,
                right: r,
                bottom: b,
                left: l,
            }
        } else {
            panic!("No PreonBorder could be parsed from {}", input)
        }
    }
}

pub fn get_variable<T: ParseXMLAttribute>(value: String) -> Option<T> {
    if value.starts_with("var") {
        None
    } else if value.starts_with("rgba") {
        None
    } else {
        Some(T::parse_xml(value))
    }
}

pub fn get_engine_from_xml(file_buffer: &str) -> PreonEngine<NoCustomComponents> {
    let mut reader = Reader::from_str(file_buffer);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut component_builder = PreonComponentBuilder::new();
    let mut is_reading_content = false;
    let mut buffer = Vec::new();

    'reader_loop: loop {
        match reader.read_event(&mut buffer) {
            Ok(Event::Start(ref e)) => {
                info!("Element: {}", String::from_utf8(e.name().to_vec()).unwrap());
                match e.name() {
                    b"content" => is_reading_content = true,
                    other => {
                        if is_reading_content {
                            match other {
                                b"vbox" => component_builder = component_builder.start_vbox(),
                                b"hbox" => component_builder = component_builder.start_hbox(),
                                b"panel" => {
                                    component_builder = component_builder.start_panel(
                                        get_variable(
                                            String::from_utf8(
                                                e.attributes()
                                                    .find(|a| a.as_ref().unwrap().key == b"color")
                                                    .unwrap_or_else(|| {
                                                        Ok(Attribute {
                                                            key: b"color",
                                                            value: Cow::Owned(b"#ff0000".to_vec()),
                                                        })
                                                    })
                                                    .unwrap()
                                                    .value
                                                    .to_vec(),
                                            )
                                            .unwrap(),
                                        )
                                        .unwrap(),
                                    )
                                }
                                b"label" => {
                                    component_builder = component_builder.start_label(
                                            String::from_utf8(
                                                e.attributes()
                                                    .find(|a| a.as_ref().unwrap().key == b"text")
                                                    .unwrap_or_else(|| {
                                                        Ok(Attribute {
                                                            key: b"text",
                                                            value: Cow::Owned(b"UNSET".to_vec()),
                                                        })
                                                    })
                                                    .unwrap()
                                                    .value
                                                    .to_vec(),
                                            )
                                            .unwrap(),
                                    )
                                }
                                _ => error!("Unrecognized component {:?}", other),
                            }

                            for (key, value) in e
                                .attributes()
                                .map(|a| {
                                    String::from_utf8(a.unwrap().key.get(..).unwrap().to_vec())
                                        .unwrap()
                                })
                                .zip(e.attributes().map(|a| {
                                    String::from_utf8(a.unwrap().value.get(..).unwrap().to_vec())
                                        .unwrap()
                                }))
                            {
                                match key.as_str() {
                                    "minSize" => {
                                        let new_min_size = get_variable::<PreonVector<i32>>(value).unwrap();
                                        component_builder = component_builder
                                            .with_min_size(new_min_size.x, new_min_size.y)
                                    }
                                    "padding" => {
                                        component_builder = component_builder.with_padding(get_variable(value).unwrap());
                                    }
                                    "margin" => {
                                        component_builder = component_builder.with_margin(get_variable(value).unwrap());
                                    }
                                    "border" => {
                                        component_builder = component_builder.with_border(get_variable(value).unwrap());
                                    }
                                    "expand" => match value.as_str() {
                                        "none" => (),
                                        "both" => component_builder = component_builder.expand(),
                                        "horizontal" => component_builder = component_builder.expand_horizontally(),
                                        "vertical" => component_builder = component_builder.expand_vertically(),
                                        v => error!("Unrecognized expand value {}, possible values: none, both, horizontal, vertical", v)
                                    }
                                    "fit" => match value.as_str() {
                                        "none" => (),
                                        "both" => component_builder = component_builder.fit_children(),
                                        "horizontal" => component_builder = component_builder.fit_children_horizontally(),
                                        "vertical" => component_builder = component_builder.fit_children_vertically(),
                                        v => error!("Unrecognized fit value {}, possible values: none, both, horizontal, vertical", v)
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
            Ok(Event::Text(ref e)) => info!("{}", e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::End(ref e)) => {
                if e.name() == b"content" {
                    is_reading_content = false;
                } else if is_reading_content {
                    component_builder = component_builder.end();
                }
            }
            Ok(Event::Eof) => break 'reader_loop,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    buffer.clear();

    PreonEngine::new(PreonStaticRenderData::empty(), component_builder.build())
}
