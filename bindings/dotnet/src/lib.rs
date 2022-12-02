#![allow(unused_unsafe)]

extern crate preon_engine;

use preon_engine::components::hbox::PreonComponentBuilderHBoxExtension;
use preon_engine::components::vbox::PreonComponentBuilderVBoxExtension;
use preon_engine::components::static_texture::PreonComponentBuilderStaticTextureExtension;
use preon_engine::components::panel::PreonComponentBuilderPanelExtension;
use preon_engine::components::label::PreonComponentBuilderLabelExtension;
use preon_engine::components::button::PreonComponentBuilderButtonExtension;
use preon_engine::style::PreonComponentBuilderStyleExtension;
use preon_engine::style::PreonComponentBuilderTextStyleExtension;

use std::os::raw::c_char;

const PREON_EVENT_SIZE: usize = core::mem::size_of::<preon_engine::events::PreonEvent>() - 4;

macro_rules! to_string {
    ($cstring:ident) => (unsafe { ffi::CStr::from_ptr($cstring).to_str().unwrap().to_string() });
    ($cstring:expr) => (unsafe { ffi::CStr::from_ptr($cstring).to_str().unwrap().to_string() });
}

macro_rules! to_cstring {
    ($string:ident) => (ffi::CString::new($string).unwrap().into_raw());
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DataHolder<T: Copy>(T);
unsafe impl<T: Copy> bytemuck::Zeroable for DataHolder<T> {}
unsafe impl<T: Copy + 'static> bytemuck::Pod for DataHolder<T> {}

#[derive(Debug)]
#[repr(C)]
pub struct PreonEventBinding {
    pub kind: u8,
    pub WindowResized_new_size_x: u32,
    pub WindowResized_new_size_y: u32,
    pub ComponentPressed_id: *mut i8,
    pub ComponentPressed_state: preon_engine::events::PreonButtonState,
    pub MouseInput_button: preon_engine::events::PreonMouseButton,
    pub MouseInput_state: preon_engine::events::PreonMouseButtonState,
}

impl PreonEventBinding {
    pub fn from_kind(kind: u8) -> Self {
        Self {
            kind,
            WindowResized_new_size_x: 0,
            WindowResized_new_size_y: 0,
            ComponentPressed_id: core::ptr::null_mut(),
            ComponentPressed_state: preon_engine::events::PreonButtonState::MouseEnter,
            MouseInput_button: preon_engine::events::PreonMouseButton::Left,
            MouseInput_state: preon_engine::events::PreonMouseButtonState::Pressed,
        }
    }
}

impl From<preon_engine::events::PreonEvent> for PreonEventBinding {
    #[inline(always)]
    fn from(event: preon_engine::events::PreonEvent) -> Self {
        match event {
            preon_engine::prelude::PreonEvent::WindowResized(new_size) => PreonEventBinding {
                WindowResized_new_size_x: new_size.x,
                WindowResized_new_size_y: new_size.y,
                ..PreonEventBinding::from_kind(0)
            },
            preon_engine::prelude::PreonEvent::WindowOpened => PreonEventBinding::from_kind(1),
            preon_engine::prelude::PreonEvent::WindowClosed => PreonEventBinding::from_kind(2),
            preon_engine::prelude::PreonEvent::Update => PreonEventBinding::from_kind(3),
            preon_engine::prelude::PreonEvent::LayoutUpdate => PreonEventBinding::from_kind(4),
            preon_engine::prelude::PreonEvent::ComponentPressed(id, state) => PreonEventBinding {
                ComponentPressed_id: to_cstring!(id),
                ComponentPressed_state: state,
                ..PreonEventBinding::from_kind(5)
            },
            preon_engine::prelude::PreonEvent::MouseInput(button, state) => PreonEventBinding {
                MouseInput_button: button,
                MouseInput_state: state,
                ..PreonEventBinding::from_kind(6)
            }
        }
    }
}

impl From<PreonEventBinding> for preon_engine::events::PreonEvent {
    fn from(binding: PreonEventBinding) -> Self {
        match binding.kind {
            0 => preon_engine::events::PreonEvent::WindowResized(preon_engine::types::PreonVector::new(binding.WindowResized_new_size_x, binding.WindowResized_new_size_y)),
            1 => preon_engine::events::PreonEvent::WindowOpened,
            2 => preon_engine::events::PreonEvent::WindowClosed,
            3 => preon_engine::events::PreonEvent::Update,
            4 => preon_engine::events::PreonEvent::LayoutUpdate,
            5 => preon_engine::events::PreonEvent::ComponentPressed(to_string!(binding.ComponentPressed_id), binding.ComponentPressed_state),
            6 => preon_engine::events::PreonEvent::MouseInput(binding.MouseInput_button, binding.MouseInput_state),
            _ => panic!("Invalid event binding!"),
        }
    }
}

use std::ffi;

#[repr(transparent)]
pub struct PreonUserEventEmitterBinding {
    inner: *mut preon_engine::events::PreonEventEmitter<preon_engine::events::PreonUserEvent>,
}

#[repr(transparent)]
pub struct PreonImageBinding {
    inner: *mut preon_engine::rendering::PreonImage,
}

impl From<PreonImageBinding> for &preon_engine::rendering::PreonImage {
    fn from(image: PreonImageBinding) -> Self {
        unsafe { image.inner.as_ref().unwrap() }
    }
}

#[repr(transparent)]
pub struct PreonFontBinding {
    inner: *mut preon_engine::rendering::PreonFont,
}

impl From<PreonFontBinding> for &preon_engine::rendering::PreonFont {
    fn from(font: PreonFontBinding) -> Self {
        unsafe { font.inner.as_ref().unwrap() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn preon__init() {
    env_logger::init();
}

#[repr(transparent)]
pub struct PreonEngineBinding {
    pub inner: *mut preon_engine::PreonEngine,
}

#[no_mangle]
pub unsafe extern "C" fn PreonEngine__new() -> PreonEngineBinding {
    PreonEngineBinding { inner: Box::into_raw(Box::new(preon_engine::PreonEngine::new())) }
}

#[no_mangle]
pub unsafe extern "C" fn PreonEngine__set_tree(engine: PreonEngineBinding, tree: PreonComponentBinding) {
    engine.inner.as_mut().unwrap().set_tree(*Box::from_raw(tree.inner));
}

#[repr(transparent)]
pub struct PreonComponentBinding {
    pub inner: *mut preon_engine::components::PreonComponent,
}

#[repr(transparent)]
pub struct PreonComponentBuilderBinding {
    pub inner: *mut preon_engine::components::PreonComponentBuilder,
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__new() -> PreonComponentBuilderBinding {
    PreonComponentBuilderBinding {
        inner: Box::into_raw(Box::new(preon_engine::components::PreonComponentBuilder::new())),
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__build(component_builder: PreonComponentBuilderBinding) -> PreonComponentBinding {
    PreonComponentBinding {
        inner: Box::into_raw(Box::new(component_builder.inner.as_mut().unwrap().build())),
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__end(component_builder: PreonComponentBuilderBinding) {
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
    start_hbox ( )
    empty_hbox ( )
    start_vbox ( )
    empty_vbox ( )
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
pub unsafe extern "C" fn PreonComponentBuilder__id_string(component_builder: PreonComponentBuilderBinding, id: *const c_char) {
    component_builder.inner.as_mut().unwrap().id_string(to_string!(id));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__start_label(component_builder: PreonComponentBuilderBinding, text: *const c_char) {
    component_builder.inner.as_mut().unwrap().start_label(to_string!(text));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__empty_label(component_builder: PreonComponentBuilderBinding, text: *const c_char) {
    component_builder.inner.as_mut().unwrap().empty_label(to_string!(text));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__start_button(component_builder: PreonComponentBuilderBinding, text: *const c_char) {
    component_builder.inner.as_mut().unwrap().start_button(to_string!(text));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponentBuilder__empty_button(component_builder: PreonComponentBuilderBinding, text: *const c_char) {
    component_builder.inner.as_mut().unwrap().empty_button(to_string!(text));
}

#[no_mangle]
pub unsafe extern "C" fn preon__run(engine: PreonEngineBinding, callback: extern "C" fn(PreonComponentBinding, PreonEventBinding, PreonUserEventEmitterBinding)) {
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
pub unsafe extern "C" fn PreonComponent__get_text(component: PreonComponentBinding) -> *mut i8 {
    let text = component.inner.as_ref().unwrap().text.as_str();
    to_cstring!(text)
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__set_text(component: PreonComponentBinding, text: *const c_char) {
    component.inner.as_mut().unwrap().text = to_string!(text);
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__get_child_ref_mut_by_id(component: PreonComponentBinding, id: *const c_char) -> PreonComponentBinding {
    let id = to_string!(id);

    let inner = component
        .inner
        .as_mut()
        .unwrap()
        .get_child_raw_by_id(id)
        .unwrap();

    PreonComponentBinding {
        inner,
    }
}