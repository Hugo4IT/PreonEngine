use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::{defaults::VerticalSupport, renderer::Renderer};

use self::input::{MouseState, PreonContextInput};

pub mod input;

pub enum PreonContextState {
    Update,
    Layout,
    Render,
}

pub struct PreonContext {
    pub state: PreonContextState,
    pub input: PreonContextInput,
    pub changed: bool,
    pub renderer: Renderer,
    layout: Vec<ElementLayout>,
    layout_index: usize,
    layout_providers: Vec<LayoutProvider>,

    current_layout_origin: PhysicalPosition<f64>,
    layout_origins: Vec<PhysicalPosition<f64>>,
    layout_origin_index: usize,
}

impl PreonContext {
    pub(crate) fn new(window: &Window) -> PreonContext {
        PreonContext {
            input: PreonContextInput::default(),
            state: PreonContextState::Update,
            changed: false,

            renderer: Renderer::new(&window),
            layout: Vec::new(),
            layout_index: 0,
            layout_providers: Vec::new(),

            current_layout_origin: PhysicalPosition::default(),
            layout_origins: Vec::new(),
            layout_origin_index: 0,
        }
    }

    #[inline]
    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub(crate) fn prepare_render(&mut self) {
        self.state = PreonContextState::Render;
        self.layout_origin_index = 0;
        self.layout_index = 0;

        self.renderer.prepare_render();

        self.begin_vertical();
    }

    pub(crate) fn finish_render(&mut self) {
        self.end_vertical();

        match self.renderer.render() {
            Ok(_) => (),
            Err(wgpu::SurfaceError::Outdated) => self.renderer.reconfigure(),
            Err(wgpu::SurfaceError::Lost) => self.renderer.reconfigure(),
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory!"),
            Err(wgpu::SurfaceError::Timeout) => self.renderer.reconfigure(),
        }
    }

    pub(crate) fn prepare_layout(&mut self) {
        self.state = PreonContextState::Layout;
        self.layout_origin_index = 0;
        self.layout_index = 0;
        self.layout.clear();

        self.begin_vertical();
    }

    pub(crate) fn finish_layout(&mut self) {
        self.end_vertical()
    }

    pub fn push_layout_provider(&mut self, provider: LayoutProvider) {
        match self.state {
            PreonContextState::Layout => {
                self.layout_providers.push(provider);
            },
            _ => {
                println!("Layout Provider.");
                self.current_layout_origin = self.get_origin();
            }
        }
    }

    pub fn pop_layout_provider(&mut self) {
        match self.state {
            PreonContextState::Layout => {
                let provider = self.layout_providers.pop().unwrap();
                let (layouts, combined_size) = provider.collect_layouts();
        
                if let Some(previous) = self.layout_providers.last_mut() {
                    previous.push_element(ElementLayoutDescriptor {
                        min_size: combined_size,
                        ..Default::default()
                    });
                }

                self.layout.extend(layouts.into_iter());

            },
            _ => ()
        }
    }

    #[inline]
    pub fn get_origin(&mut self) -> PhysicalPosition<f64> {
        let origin = self.layout_origins[self.layout_origin_index];
        self.layout_origin_index += 1;
        origin
    }

    /// Calling `self.get_layout` is **required** for any non-Layout state element update
    #[inline]
    pub fn get_layout(&mut self) -> ElementLayout {
        let layout = self.layout[self.layout_index];
        println!("Layout: {:#?}, Origin: {:?}", layout, self.current_layout_origin);
        self.layout_index += 1;
        ElementLayout {
            position: PhysicalPosition::new(
                layout.position.x + self.current_layout_origin.x,
                layout.position.y + self.current_layout_origin.y,
            ),
            size: layout.size,
            is_provider: layout.is_provider
        }
    }

    pub fn set_layout(&mut self, layout: ElementLayoutDescriptor) {
        self.layout_providers
            .last_mut()
            .unwrap()
            .push_element(layout);
    }

    pub(crate) fn prepare_update(&mut self) {
        self.changed = false;
        self.layout_index = 0;
        self.layout_origin_index = 0;
        self.state = PreonContextState::Update;

        self.begin_vertical();
    }

    pub(crate) fn finish_update(&mut self) -> bool {
        self.end_vertical();

        self.input.reset();
        self.changed
    }

    #[inline]
    pub fn flag_changed(&mut self) {
        self.changed = true
    }

    #[inline]
    pub fn get_mouse(&self) -> MouseState {
        self.input.mouse
    }

    pub fn draw_rectangle(&mut self, rect: ElementLayout) {
        self.renderer.rect.queue_render(rect);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementLayout {
    pub position: PhysicalPosition<f64>,
    pub size: PhysicalSize<f64>,
    is_provider: bool,
}

impl ElementLayout {
    pub fn is_hovered(&self, mouse: MouseState) -> bool {
        let left = self.position.x;
        let right = self.position.x + self.size.width;
        let top = self.position.y;
        let bottom = self.position.y + self.size.height;

        mouse.position.x > left
            && mouse.position.x < right
            && mouse.position.y > top
            && mouse.position.y < bottom
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElementLayoutDescriptor {
    pub override_layout: Option<(PhysicalPosition<f64>, PhysicalSize<f64>)>,
    pub min_size: PhysicalSize<f64>,
    pub margins: PhysicalSize<f64>,
}

impl ElementLayoutDescriptor {
    #[inline]
    pub fn actual_min_size(&self) -> PhysicalSize<f64> {
        PhysicalSize::new(
            self.min_size.width + self.margins.width,
            self.min_size.height + self.margins.height,
        )
    }
}

impl Default for ElementLayoutDescriptor {
    fn default() -> ElementLayoutDescriptor {
        ElementLayoutDescriptor {
            override_layout: None,
            min_size: PhysicalSize::default(),
            margins: PhysicalSize::default(),
        }
    }
}

pub type LayoutProviderFunction =
    fn(Vec<ElementLayoutDescriptor>) -> (Vec<ElementLayout>, PhysicalSize<f64>);

#[derive(Debug)]
pub struct LayoutProvider {
    elements: Vec<ElementLayoutDescriptor>,
    function: LayoutProviderFunction,
}

impl LayoutProvider {
    pub fn new(function: LayoutProviderFunction) -> LayoutProvider {
        LayoutProvider {
            elements: Vec::new(),
            function,
        }
    }

    pub fn push_element(&mut self, element: ElementLayoutDescriptor) {
        self.elements.push(element);
    }

    pub fn collect_layouts(&self) -> (Vec<ElementLayout>, PhysicalSize<f64>) {
        self.function.clone()(self.elements.clone())
    }
}
