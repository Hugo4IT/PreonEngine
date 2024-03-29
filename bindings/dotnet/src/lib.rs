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
    pub button_state: preon_engine::events::PreonButtonState,
    pub WindowResized_new_size_x: u32,
    pub WindowResized_new_size_y: u32,
    pub ComponentPressed_id: *mut i8,
    pub MouseInput_button: u16,
    pub KeyboardInput_key: preon_engine::events::PreonKeyCode,
    pub ReceivedCharacter_ch: char,
}

impl PreonEventBinding {
    pub fn from_kind(kind: u8) -> Self {
        Self {
            kind,
            WindowResized_new_size_x: 0,
            WindowResized_new_size_y: 0,
            ComponentPressed_id: core::ptr::null_mut(),
            button_state: preon_engine::events::PreonButtonState::Released,
            MouseInput_button: 0,
            KeyboardInput_key: preon_engine::events::PreonKeyCode::A,
            ReceivedCharacter_ch: '\0',
        }
    }
}

impl From<preon_engine::events::PreonEvent> for PreonEventBinding {
    #[inline(always)]
    fn from(event: preon_engine::events::PreonEvent) -> Self {
        match event {
            preon_engine::prelude::PreonEvent::WindowOpened => PreonEventBinding::from_kind(0),
            preon_engine::prelude::PreonEvent::WindowResized(new_size) => PreonEventBinding {
                WindowResized_new_size_x: new_size.x,
                WindowResized_new_size_y: new_size.y,
                ..PreonEventBinding::from_kind(1)
            },
            preon_engine::prelude::PreonEvent::WindowClosed => PreonEventBinding::from_kind(2),
            preon_engine::prelude::PreonEvent::Update => PreonEventBinding::from_kind(3),
            preon_engine::prelude::PreonEvent::LayoutUpdate => PreonEventBinding::from_kind(4),
            preon_engine::prelude::PreonEvent::ComponentPressed(id, state) => PreonEventBinding {
                ComponentPressed_id: to_cstring!(id),
                button_state: state,
                ..PreonEventBinding::from_kind(5)
            },
            preon_engine::prelude::PreonEvent::MouseInput(button, state) => PreonEventBinding {
                MouseInput_button: match button {
                    preon_engine::events::PreonMouseButton::Left => 0,
                    preon_engine::events::PreonMouseButton::Middle => 1,
                    preon_engine::events::PreonMouseButton::Right => 2,
                    preon_engine::events::PreonMouseButton::Other(other) => other,
                },
                button_state: state,
                ..PreonEventBinding::from_kind(6)
            },
            preon_engine::prelude::PreonEvent::KeyboardInput(key, state) => PreonEventBinding {
                KeyboardInput_key: key,
                button_state: state,
                ..PreonEventBinding::from_kind(7)
            },
            preon_engine::prelude::PreonEvent::ReceivedCharacter(ch) => PreonEventBinding {
                ReceivedCharacter_ch: ch,
                ..PreonEventBinding::from_kind(8)
            }
        }
    }
}

// impl From<PreonEventBinding> for preon_engine::events::PreonEvent {
//     fn from(binding: PreonEventBinding) -> Self {
//         match binding.kind {
//             0 => preon_engine::events::PreonEvent::WindowOpened,
//             1 => preon_engine::events::PreonEvent::WindowResized(preon_engine::types::PreonVector::new(binding.WindowResized_new_size_x, binding.WindowResized_new_size_y)),
//             2 => preon_engine::events::PreonEvent::WindowClosed,
//             3 => preon_engine::events::PreonEvent::Update,
//             4 => preon_engine::events::PreonEvent::LayoutUpdate,
//             5 => preon_engine::events::PreonEvent::ComponentPressed(to_string!(binding.ComponentPressed_id), binding.ComponentPressed_state),
//             6 => preon_engine::events::PreonEvent::MouseInput(binding.MouseInput_button, binding.MouseInput_state),
//             7 => preon_engine::events::PreonEvent::MouseInput(binding.MouseInput_button, binding.MouseInput_state),
//             _ => panic!("Invalid event binding!"),
//         }
//     }
// }

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

    receive_events ( receive_events: bool )
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
pub unsafe extern "C" fn preon__run(engine: PreonEngineBinding, callback: extern "C" fn(PreonComponentBinding, PreonEventBinding, PreonUserEventEmitterBinding) -> bool) {
    let mut engine = *Box::from_raw(engine.inner);

    let juan = engine.load_image(&include_bytes!("../../../res/juan.png")[..]);
    let font_normal = engine.load_font(&include_bytes!("../../../res/Montserrat-Regular.otf")[..]);

    preon_module_wgpu::preon::run(engine, move |tree, event, user_events| {
        if callback(
            PreonComponentBinding {
                inner: tree as *mut preon_engine::components::PreonComponent,
            },
            event.into(),
            PreonUserEventEmitterBinding {
                inner: user_events as *mut preon_engine::events::PreonEventEmitter<preon_engine::events::PreonUserEvent>,
            },
        ) {
            user_events.push(preon_engine::events::PreonUserEvent::ForceUpdate);
        }
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
pub unsafe extern "C" fn PreonComponent__style_set_background_color(component: PreonComponentBinding, color: preon_engine::types::PreonColor) {
    component.inner.as_mut().unwrap().style.background = preon_engine::style::PreonBackground::Color(color);
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_clear_background_color(component: PreonComponentBinding) {
    component.inner.as_mut().unwrap().style.background = preon_engine::style::PreonBackground::None;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_foreground_color(component: PreonComponentBinding, color: preon_engine::types::PreonColor) {
    component.inner.as_mut().unwrap().style.foreground_color = color;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_align_items(component: PreonComponentBinding, alignment: preon_engine::types::PreonAlignment) {
    component.inner.as_mut().unwrap().style.align_items = alignment;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_cross_align_items(component: PreonComponentBinding, alignment: preon_engine::types::PreonAlignment) {
    component.inner.as_mut().unwrap().style.cross_align_items = alignment;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_layout(component: PreonComponentBinding, layout: preon_engine::layout::PreonLayout) {
    component.inner.as_mut().unwrap().style.layout = layout;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_margin(component: PreonComponentBinding, margin: preon_engine::types::PreonBorder) {
    component.inner.as_mut().unwrap().style.margin = margin;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_padding(component: PreonComponentBinding, padding: preon_engine::types::PreonBorder) {
    component.inner.as_mut().unwrap().style.padding = padding;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_border(component: PreonComponentBinding, border: preon_engine::types::PreonBorder) {
    component.inner.as_mut().unwrap().style.border = border;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_corner_radius(component: PreonComponentBinding, corners: preon_engine::types::PreonCorners) {
    component.inner.as_mut().unwrap().style.corner_radius = corners;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_size_flags(component: PreonComponentBinding, size_flags: u8) {
    component.inner.as_mut().unwrap().style.size_flags = size_flags;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__style_set_min_size(component: PreonComponentBinding, min_size: preon_engine::types::PreonVector<i32>) {
    component.inner.as_mut().unwrap().style.min_size = min_size;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__text_style_set_font(component: PreonComponentBinding, font: PreonFontBinding) {
    component.inner.as_mut().unwrap().style.text_style.font = Some(*Box::from_raw(font.inner));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__text_style_set_font_size(component: PreonComponentBinding, font_size: f32) {
    component.inner.as_mut().unwrap().style.text_style.size = font_size;
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__get_child_ref_mut_by_id(component: PreonComponentBinding, id: *const c_char) -> PreonComponentBinding {
    let id = to_string!(id);

    let inner = component
        .inner
        .as_mut()
        .unwrap()
        .get_child_raw_by_id(id.clone())
        .expect(&format!("PreonComponent.get_child_ref_mut_by_id failed, could not find child with id {}", id));

    PreonComponentBinding {
        inner,
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__new() -> PreonComponentBinding {
    let component = preon_engine::components::PreonComponent::new();

    PreonComponentBinding {
        inner: Box::into_raw(Box::new(component)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__add_child(component: PreonComponentBinding, child: PreonComponentBinding) {
    component.inner.as_mut().unwrap().add_child(*Box::from_raw(child.inner));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__insert_child(component: PreonComponentBinding, idx: u16, child: PreonComponentBinding) {
    component.inner.as_mut().unwrap().insert_child(idx, *Box::from_raw(child.inner));
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__remove_child(component: PreonComponentBinding, idx: u16) {
    component.inner.as_mut().unwrap().remove_child(idx);
}

#[no_mangle]
pub unsafe extern "C" fn PreonComponent__clear_children(component: PreonComponentBinding) {
    component.inner.as_mut().unwrap().clear_children();
}