use core::str::FromStr;

use alloc::string::String;

use crate::components::PreonComponent;
 
use super::PreonComponentBuilder;

pub trait AddLabel {
    fn start_label(&mut self, text: String) -> &mut PreonComponentBuilder;
    fn start_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder;
    fn empty_label(&mut self, text: String) -> &mut PreonComponentBuilder;
    fn empty_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder;
    fn bold(&mut self) -> &mut PreonComponentBuilder;
    fn italic(&mut self) -> &mut PreonComponentBuilder;
}

impl AddLabel for PreonComponentBuilder {
    fn start_label(&mut self, text: String) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            text,
            ..Default::default()
        });

        self
    }

    fn start_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder {
        self.start_label(String::from_str(text).unwrap())
    }

    fn empty_label(&mut self, text: String) -> &mut PreonComponentBuilder {
        self.start_label(text).end()
    }

    fn empty_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder {
        self.start_label_str(text).end()
    }

    fn bold(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.bold = true;
        self
    }

    fn italic(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.italic = true;
        self
    }
}