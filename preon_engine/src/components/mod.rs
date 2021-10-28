use std::any::Any;

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    types::{PreonBorder, PreonBox, PreonColor, PreonVector},
};

#[derive(Debug, Clone)]
pub struct PreonComponent<T: PreonCustomComponentStack> {
    pub children: Option<Vec<PreonComponent<T>>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
    pub inner_size: PreonVector<i32>,
}

impl<T: PreonCustomComponentStack> PreonComponent<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponent<T> {
        PreonComponent {
            children: Some(Vec::new()),
            model: PreonBox::initial(),
            data: component,
            inner_size: PreonVector::zero(),
        }
    }

    pub fn get_inner_size(&self) -> PreonVector<i32> {
        PreonVector::new(self.inner_size.x.max(self.model.min_size.x), {
            self.inner_size.y.max(self.model.min_size.y)
        }) + self.model.border
    }

    pub fn get_outer_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.model.margin + self.model.border
    }

    pub fn get_content_size(&self) -> PreonVector<i32> {
        self.get_inner_size() - self.model.padding - self.model.margin - self.model.border
    }

    pub fn set_inner_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.model.border;
    }

    pub fn set_outer_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size + self.model.margin + self.model.border;
    }

    pub fn set_content_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size + self.model.padding + self.model.margin + self.model.border;
    }
}

impl<T: PreonCustomComponentStack> Default for PreonComponent<T> {
    fn default() -> Self {
        Self {
            children: None,
            model: PreonBox::initial(),
            data: PreonComponentStack::VBoxComponent,
            inner_size: PreonVector::new(0, 0),
        }
    }
}

pub enum PreonComponentRenderStage {
    Background {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
    Foreground {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
    Border {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        width: PreonBorder,
    },
}

pub trait PreonCustomComponentStack {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(comp: &mut PreonComponent<T>);
    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        stage: PreonComponentRenderStage,
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    );

    fn layout<T: PreonCustomComponentStack + Any + 'static>(mut component: &mut PreonComponent<T>) {
        if let Some(mut children) = component.children.take() {
            component.children = Some(
                children
                    .drain(..)
                    .map(|mut f| -> PreonComponent<T> {
                        T::layout(&mut f);
                        f
                    })
                    .collect::<Vec<PreonComponent<T>>>(),
            );
        }

        match component.data {
            PreonComponentStack::Custom(_) => T::custom_layout::<T>(&mut component),
            PreonComponentStack::RectComponent { .. } => {}
            PreonComponentStack::VBoxComponent => {
                if component.children.is_some() {
                    let mut heights = 0;
                    let mut width = 0;

                    component.children = Some(
                        component
                            .children
                            .take()
                            .unwrap()
                            .drain(..)
                            .map(|child| {
                                let s = child.get_outer_size();
                                heights += s.y;
                                width = width.max(s.x);

                                child
                            })
                            .collect::<Vec<PreonComponent<T>>>(),
                    );

                    component.set_content_size(PreonVector::new(width, heights));
                }
            }
        }
    }

    fn render<T: PreonCustomComponentStack + 'static>(
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    ) {
        if let Some(mut children) = component.children.take() {
            component.children = Some(
                children
                    .drain(..)
                    .map(|mut f| -> PreonComponent<T> {
                        T::render(&mut f, pass);
                        f
                    })
                    .collect::<Vec<PreonComponent<T>>>(),
            );
        }

        let mut stages = vec![
            PreonComponentRenderStage::Border {
                position: PreonVector::zero(),
                size: component.get_inner_size(),
                width: component.model.border,
            },
            PreonComponentRenderStage::Background {
                position: PreonVector::zero(),
                size: component.get_inner_size() - component.model.border,
            },
            PreonComponentRenderStage::Foreground {
                position: PreonVector::zero(),
                size: component.get_content_size(),
            },
        ];

        stages.drain(..).for_each(|stage| match stage {
            PreonComponentRenderStage::Background { position, size } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
                PreonComponentStack::RectComponent { color } => pass.push(PreonShape::Rect {
                    position,
                    size,
                    color,
                }),
                PreonComponentStack::VBoxComponent => {}
            },
            PreonComponentRenderStage::Foreground { .. } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
                _ => {}
            },
            PreonComponentRenderStage::Border { .. } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
                _ => {}
            },
        });
    }
}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    RectComponent { color: PreonColor },
    VBoxComponent,
}
