use core::any::Any;

use alloc::boxed::Box;

use crate::{types::color::Color, Component, RenderPass};

#[derive(Default, Debug)]
pub struct Panel(pub Color);

impl Panel {
    pub fn empty() -> Box<dyn Any> {
        Box::new(Panel::default())
    }
}

impl Component for Panel {
    fn init(input: Box<dyn Any>) -> Box<dyn Any> {
        input
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
