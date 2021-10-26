use crate::{rendering::PreonRenderPass, types::{PreonBox, PreonColor, PreonVector}};

#[derive(Debug, Clone)]
pub struct PreonComponent<T: PreonCustomComponentStack> {
    pub children: Option<Vec<PreonComponent<T>>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
    pub inner_size: PreonVector<i32>,
    pub outer_size: PreonVector<i32>,
}

impl<T: PreonCustomComponentStack> PreonComponent<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponent<T> {
        PreonComponent {
            children: Some(Vec::new()),
            model: PreonBox::initial(),
            data: component,
            inner_size: PreonVector::zero(),
            outer_size: PreonVector::zero(),
        }
    }

    pub fn get_size(&self) -> (PreonVector<i32>, PreonVector<i32>) {
        (PreonVector::new({
            self.inner_size.x.max(self.model.min_size.x)
        }, {
            self.inner_size.y.max(self.model.min_size.y)
        }),PreonVector::new({
            (self.inner_size + self.model.margin).x.max((self.model.min_size + self.model.margin).x)
        }, {
            (self.inner_size + self.model.margin).y.max((self.model.min_size + self.model.margin).y)
        }))
    }
}

impl<T: PreonCustomComponentStack> Default for PreonComponent<T> {
    fn default() -> Self {
        Self {
            children: None,
            model: PreonBox::initial(),
            data: PreonComponentStack::VBoxComponent,
            inner_size: PreonVector::new(0, 0),
            outer_size: PreonVector::new(0, 0),
        }
    }
}

pub trait PreonCustomComponentStack {
    fn custom_layout(component: &mut Self);

    fn layout<T: PreonCustomComponentStack>(component: &mut PreonComponent<T>) {
        if let Some(mut children) = component.children.take() {
            component.children = Some(children.drain(..).map(|mut f| -> PreonComponent<T> {
                T::layout(&mut f);
                f
            }).collect::<Vec<PreonComponent<T>>>());
        }

        match &mut component.data {
            PreonComponentStack::Custom(ref mut c) => T::custom_layout(c),
            PreonComponentStack::RectComponent { color } => println!("RectComponent called: {}", color),
            PreonComponentStack::VBoxComponent => if component.children.is_some() {
                component.inner_size = {
                    let mut heights = 0;
                    let mut width = 0;

                    component.children = Some(component.children.take().unwrap().drain(..).map(|child| {
                        let s = child.get_size().1;
                        heights += s.y;
                        width = width.max(s.x);

                        child
                    }).collect::<Vec<PreonComponent<T>>>());

                    PreonVector::new(width, heights)
                }
            },
        }
    }

    fn render<T: PreonCustomComponentStack>(data: &PreonComponentStack<T>, pass: &mut PreonRenderPass) {
    }
}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    RectComponent { color: PreonColor },
    VBoxComponent
}