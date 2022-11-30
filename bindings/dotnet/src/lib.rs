extern crate preon_engine;

use preon_engine::components::hbox::PreonComponentBuilderHBoxExtension;
use preon_engine::components::vbox::PreonComponentBuilderVBoxExtension;
use preon_engine::components::static_texture::PreonComponentBuilderStaticTextureExtension;
use preon_engine::components::panel::PreonComponentBuilderPanelExtension;
use preon_engine::components::label::PreonComponentBuilderLabelExtension;
use preon_engine::style::PreonComponentBuilderStyleExtension;
use preon_engine::style::PreonComponentBuilderTextStyleExtension;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DataHolder<T: Copy>(T);
unsafe impl<T: Copy> bytemuck::Zeroable for DataHolder<T> {}
unsafe impl<T: Copy + 'static> bytemuck::Pod for DataHolder<T> {}

const PREON_EVENT_SIZE: usize = core::mem::size_of::<preon_engine::events::PreonEvent>();

#[repr(C)]
pub struct PreonEventBinding {
    kind: u8,
    data: [u8; PREON_EVENT_SIZE],
}

impl PreonEventBinding {
    pub fn from_data<T>(kind: u8, data: T) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable
    {
        let data_ref = &[data];

        let mut data = bytemuck::cast_slice(data_ref).to_vec();
        data.resize(PREON_EVENT_SIZE, 0);

        let mut data_arr = [0u8; PREON_EVENT_SIZE];
        data_arr.copy_from_slice(data.as_slice());

        Self {
            kind,
            data: data_arr,
        }
    }

    pub fn from_kind(kind: u8) -> Self {
        Self {
            kind,
            data: [0u8; PREON_EVENT_SIZE],
        }
    }
}

impl From<preon_engine::events::PreonEvent> for PreonEventBinding {
    fn from(event: preon_engine::events::PreonEvent) -> Self {
        match event {
            preon_engine::prelude::PreonEvent::WindowResized(new_size) => PreonEventBinding::from_data(0, DataHolder((new_size.x, new_size.y))),
            preon_engine::prelude::PreonEvent::WindowOpened => PreonEventBinding::from_kind(1),
            preon_engine::prelude::PreonEvent::WindowClosed => PreonEventBinding::from_kind(2),
            preon_engine::prelude::PreonEvent::Update => PreonEventBinding::from_kind(3),
            preon_engine::prelude::PreonEvent::LayoutUpdate => PreonEventBinding::from_kind(4),
            preon_engine::prelude::PreonEvent::Button(id, state) => PreonEventBinding::from_data(5, DataHolder((id, state))),
        }
    }
}

impl From<preon_engine::events::PreonUserEvent> for PreonEventBinding {
    fn from(event: preon_engine::events::PreonUserEvent) -> Self {
        match event {
            preon_engine::prelude::PreonUserEvent::WindowResized(new_size) => PreonEventBinding::from_data(0, DataHolder(new_size)),
            preon_engine::prelude::PreonUserEvent::WindowOpened => PreonEventBinding::from_kind(1),
            preon_engine::prelude::PreonUserEvent::WindowClosed => PreonEventBinding::from_kind(2),
            preon_engine::prelude::PreonUserEvent::MouseMove(position) => PreonEventBinding::from_data(3, DataHolder(position)),
            preon_engine::prelude::PreonUserEvent::ForceUpdate => PreonEventBinding::from_kind(4),
        }
    }
}

#[repr(C)]
pub struct PreonUserEventEmitterBinding {
    inner: *mut preon_engine::events::PreonEventEmitter<preon_engine::events::PreonUserEvent>,
}

#[repr(C)]
pub struct StringBinding {
    length: usize,
    string: *const u8,
}

impl From<StringBinding> for String {
    fn from(string: StringBinding) -> Self {
        unsafe {
            String::from_raw_parts(string.string as *mut u8, string.length, string.length)
        }
    }
}

impl From<String> for StringBinding {
    fn from(string: String) -> Self {
        StringBinding {
            length: string.len(),
            string: string.as_ptr(),
        }
    }
}

#[repr(C)]
pub struct PreonImageBinding {
    inner: *mut preon_engine::rendering::PreonImage,
}

impl From<PreonImageBinding> for &preon_engine::rendering::PreonImage {
    fn from(image: PreonImageBinding) -> Self {
        unsafe { image.inner.as_ref().unwrap() }
    }
}

#[repr(C)]
pub struct PreonFontBinding {
    inner: *mut preon_engine::rendering::PreonFont,
}

impl From<PreonFontBinding> for &preon_engine::rendering::PreonFont {
    fn from(font: PreonFontBinding) -> Self {
        unsafe { font.inner.as_ref().unwrap() }
    }
}

#[no_mangle]
pub unsafe extern fn preon__init() {
    env_logger::init();
}

#[repr(C)]
pub struct PreonEngineBinding {
    pub inner: *mut preon_engine::PreonEngine,
}

#[no_mangle]
pub unsafe extern fn PreonEngine__new() -> PreonEngineBinding {
    PreonEngineBinding { inner: Box::into_raw(Box::new(preon_engine::PreonEngine::new())) }
}

#[no_mangle]
pub unsafe extern fn PreonEngine__set_tree(engine: PreonEngineBinding, tree: PreonComponentBinding) {
    engine.inner.as_mut().unwrap().set_tree(*Box::from_raw(tree.inner));
}

#[repr(C)]
pub struct PreonComponentBinding {
    pub inner: *mut preon_engine::components::PreonComponent,
}

#[repr(C)]
pub struct PreonComponentBuilderBinding {
    pub inner: *mut preon_engine::components::PreonComponentBuilder,
}

#[no_mangle]
pub unsafe extern fn PreonComponentBuilder__new() -> PreonComponentBuilderBinding {
    PreonComponentBuilderBinding {
        inner: Box::into_raw(Box::new(preon_engine::components::PreonComponentBuilder::new())),
    }
}

#[no_mangle]
pub unsafe extern fn PreonComponentBuilder__build(component_builder: PreonComponentBuilderBinding) -> PreonComponentBinding {
    PreonComponentBinding {
        inner: Box::into_raw(Box::new(component_builder.inner.as_mut().unwrap().build())),
    }
}

#[no_mangle]
pub unsafe extern fn PreonComponentBuilder__end(component_builder: PreonComponentBuilderBinding) {
    component_builder.inner.as_mut().unwrap().end();
}

macro_rules! component_builder_funcs {
    ($funcname:ident ( $($($paramname:ident: $paramtype:ty),+)? )) => {
        paste::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn [<PreonComponentBuilder__ $funcname>](component_builder: PreonComponentBuilderBinding, $($($paramname: $paramtype),+)?) {
                component_builder.inner.as_mut().unwrap().$funcname($($(($paramname as $paramtype).into()),+)?);
            }
        }
    };
    ($funcname:ident ( $($($paramname:ident: $paramtype:ty),+)? ) $($funcnameright:ident ( $($($paramnameright:ident: $paramtyperight:ty),+)? ))+) => {
        component_builder_funcs!($funcname ( $($($paramname: $paramtype),+)? ));
        component_builder_funcs!($($funcnameright ( $($($paramnameright: $paramtyperight),+)? ))+);
    };
}

component_builder_funcs!(
    id_string ( id: StringBinding )
    start_hbox ( )
    empty_hbox ( )
    start_vbox ( )
    empty_vbox ( )
    start_label ( text: StringBinding )
    empty_label ( text: StringBinding )
    start_panel ( color: preon_engine::types::PreonColor )
    empty_panel ( color: preon_engine::types::PreonColor )
    panel_color ( color: preon_engine::types::PreonColor )
    start_static_texture ( image: PreonImageBinding )

    background_image ( image: PreonImageBinding )
    background_color ( color: preon_engine::types::PreonColor )
    foreground_color ( color: preon_engine::types::PreonColor )
    align_items ( align: preon_engine::types::PreonAlignment )
    cross_align_items ( align: preon_engine::types::PreonAlignment )
    layout ( layout: preon_engine::layout::PreonLayout )
    margin ( margin: preon_engine::types::PreonBorder )
    padding ( padding: preon_engine::types::PreonBorder )
    border ( border: preon_engine::types::PreonBorder )
    corner_radius ( corner_radius: preon_engine::types::PreonCorners )
    min_size ( min_size: preon_engine::types::PreonVector<i32> )
    fit_children (  )
    fit_children_horizontally (  )
    fit_children_vertically (  )
    expand (  )
    expand_horizontally (  )
    expand_vertically (  )

    text_vertical_align ( align: preon_engine::types::PreonAlignment )
    text_horizontal_align ( align: preon_engine::types::PreonAlignment )
    font ( font: PreonFontBinding )
    font_size ( size: f32 )
);

#[no_mangle]
pub unsafe extern "C" fn preon__run(engine: PreonEngineBinding, callback: extern fn(PreonComponentBinding, PreonEventBinding, PreonUserEventEmitterBinding)) {
    let mut engine = *Box::from_raw(engine.inner);

    let juan = engine.load_image(&include_bytes!("../../../res/juan.png")[..]);
    let font_normal = engine.load_font(&include_bytes!("../../../res/Montserrat-Regular.otf")[..]);

    preon_module_wgpu::preon::run(engine, move |tree, event, user_events| {
        callback(
            PreonComponentBinding {
                inner: tree as *mut preon_engine::components::PreonComponent,
            },
            event.into(),
            PreonUserEventEmitterBinding {
                inner: user_events as *mut preon_engine::events::PreonEventEmitter<preon_engine::events::PreonUserEvent>,
            },
        )
    });
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__get_text(component: PreonComponentBinding) -> StringBinding {
    let component = component.inner.as_ref().unwrap();
    StringBinding {
        length: component.text.len(),
        string: component.text.as_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__set_text(component: PreonComponentBinding, text: StringBinding) {
    component.inner.as_mut().unwrap().text = text.into();
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__get_child_ref_mut_by_id(component: PreonComponentBinding, id: StringBinding) -> PreonComponentBinding {
    let id = String::from(id);
    println!("1");
    let inner = component
        .inner
        .as_mut()
        .unwrap();
    println!("2 {}", id);
    let inner = inner
        .get_child_raw_by_id(id);
    println!("3");
    let inner = inner
        .unwrap();
    println!("4");

    PreonComponentBinding {
        inner: inner,
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__test(component: PreonComponentBinding, text: StringBinding) -> StringBinding {
    println!("Rust Received: {}", String::from(text));

    println!("Before!");
    println!("{:?}", component.inner.as_ref().unwrap());
    println!("After!");

    "This is a message from Rust".to_string().into()
}