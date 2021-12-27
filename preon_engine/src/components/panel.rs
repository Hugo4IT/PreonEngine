use core::{any::Any, borrow::Borrow};

use alloc::boxed::Box;

use crate::{types::Color, Component, RenderPass};

#[derive(Default, Debug)]
pub struct Panel(pub Color);

impl Panel {
    pub fn empty() -> Box<dyn Any> {
        Box::new(Panel::default())
    }
}

impl Component for Panel {
    fn init(input: Box<dyn Any>) -> Box<dyn Any> {
        Box::new(Panel(
            *input
                .downcast::<Color>()
                .expect("input_data must be of type Color when creating a Panel.")
                .borrow(),
        ))
    }

    fn update(data: &mut Box<dyn Any>) {
        let data: &mut Panel = data.downcast_mut().unwrap();
    }

    fn render(data: &Box<dyn Any>, pass: &mut RenderPass) {
        todo!()
    }

    fn destroy(data: Box<dyn Any>) {
        todo!()
    }
}
