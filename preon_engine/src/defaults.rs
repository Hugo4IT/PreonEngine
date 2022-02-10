use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::context::{
    input::MouseButton, ElementLayout, ElementLayoutDescriptor, LayoutProvider, PreonContext,
    PreonContextState,
};

pub trait VerticalSupport {
    fn begin_vertical(&mut self);
    fn end_vertical(&mut self);
}

impl VerticalSupport for PreonContext {
    fn begin_vertical(&mut self) {
        match self.state {
            PreonContextState::Layout => self.push_layout_provider(VerticalLayout::provider()),
            _ => {
                // self.get_layout();
            }
        }
    }

    fn end_vertical(&mut self) {
        match self.state {
            PreonContextState::Layout => self.pop_layout_provider(),
            _ => {
                self.get_layout();
            }
        }
    }
}

pub struct VerticalLayout;
impl VerticalLayout {
    pub fn layout(
        descriptors: Vec<ElementLayoutDescriptor>,
    ) -> (Vec<ElementLayout>, PhysicalSize<f64>) {
        let combined_min_width = descriptors
            .clone()
            .iter()
            .fold(0f64, |a, l| l.actual_min_size().width.max(a));

        let mut current_y = 0f64;
        (
            descriptors
                .iter()
                .map(|layout| {
                    if let Some((position, size)) = layout.override_layout {
                        ElementLayout { position, size }
                    } else {
                        let new_layout = ElementLayout {
                            position: PhysicalPosition::new(0.0, current_y),
                            size: layout.actual_min_size(),
                        };
                        current_y += layout.actual_min_size().height;
                        new_layout
                    }
                })
                .collect::<Vec<ElementLayout>>(),
            PhysicalSize::new(combined_min_width, current_y),
        )
    }

    pub fn provider() -> LayoutProvider {
        LayoutProvider::new(VerticalLayout::layout)
    }
}

pub trait HorizontalSupport {
    fn begin_horizontal(&mut self);
    fn end_horizontal(&mut self);
}

impl HorizontalSupport for PreonContext {
    fn begin_horizontal(&mut self) {
        match self.state {
            PreonContextState::Layout => self.push_layout_provider(HorizontalLayout::provider()),
            _ => {
                // self.get_layout();
            }
        }
    }

    fn end_horizontal(&mut self) {
        match self.state {
            PreonContextState::Layout => self.pop_layout_provider(),
            _ => {
                self.get_layout();
            }
        }
    }
}

pub struct HorizontalLayout;
impl HorizontalLayout {
    pub fn layout(
        descriptors: Vec<ElementLayoutDescriptor>,
    ) -> (Vec<ElementLayout>, PhysicalSize<f64>) {
        let combined_min_height = descriptors
            .clone()
            .iter()
            .fold(0f64, |a, l| l.actual_min_size().height.max(a));

        let mut current_x = 0f64;
        (
            descriptors
                .iter()
                .map(|layout| {
                    if let Some((position, size)) = layout.override_layout {
                        ElementLayout { position, size }
                    } else {
                        let new_layout = ElementLayout {
                            position: PhysicalPosition::new(current_x, 0.0),
                            size: layout.actual_min_size(),
                        };
                        current_x += layout.actual_min_size().width;
                        new_layout
                    }
                })
                .collect::<Vec<ElementLayout>>(),
            PhysicalSize::new(current_x, combined_min_height),
        )
    }

    pub fn provider() -> LayoutProvider {
        LayoutProvider::new(HorizontalLayout::layout)
    }
}

pub trait CheckBoxSupport {
    fn checkbox(&mut self, prop: &mut bool);
}

impl CheckBoxSupport for PreonContext {
    fn checkbox(&mut self, prop: &mut bool) {
        match self.state {
            PreonContextState::Update => {
                let layout = self.get_layout();
                if layout.is_hovered(self.get_mouse())
                    && self.get_mouse().get_button(MouseButton::Left).just_released
                {
                    *prop = !*prop;
                    self.flag_changed();
                }
            }
            PreonContextState::Layout => self.set_layout(ElementLayoutDescriptor {
                min_size: PhysicalSize::new(50.0, 50.0),
                ..Default::default()
            }),
            PreonContextState::Render => {
                let layout = self.get_layout();
                self.renderer.rect.queue_render(layout);
            }
        }
    }
}
