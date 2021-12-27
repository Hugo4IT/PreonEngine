#![no_std]

//! A #![no_std] (using `alloc`) User Interface engine

use core::{any::Any, cell::RefCell};

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use components::panel::Panel;
use rendering::RenderPass;

extern crate alloc;

pub mod components;
pub mod rendering;
pub mod types;

pub struct PreonEngine {
    fn_init: Vec<fn(Box<dyn Any>) -> Box<dyn Any>>,
    fn_update: Vec<fn(&mut Box<dyn Any>)>,
    fn_render: Vec<fn(&Box<dyn Any>, &mut RenderPass)>,
    fn_destroy: Vec<fn(Box<dyn Any>)>,
    component_data: Vec<ComponentDataHolder>,
}

impl PreonEngine {
    fn _new() -> PreonEngine {
        PreonEngine {
            fn_init: Vec::new(),
            fn_update: Vec::new(),
            fn_render: Vec::new(),
            fn_destroy: Vec::new(),
            component_data: Vec::new(),
        }
    }

    pub fn new() -> PreonEngine {
        let mut engine = PreonEngine::_new();

        engine.add_type::<Panel>();

        engine
    }

    pub fn add_type<T: Component>(&mut self) -> usize {
        self.add_type_directly(T::init, T::update, T::render, T::destroy)
    }

    pub fn add_type_directly(
        &mut self,
        init_function: fn(Box<dyn Any>) -> Box<dyn Any>,
        update_function: fn(&mut Box<dyn Any>),
        render_function: fn(&Box<dyn Any>, &mut RenderPass),
        destroy_function: fn(Box<dyn Any>),
    ) -> usize {
        let current_index = self.fn_init.len();

        self.fn_init.push(init_function);
        self.fn_update.push(update_function);
        self.fn_render.push(render_function);
        self.fn_destroy.push(destroy_function);

        current_index
    }

    pub fn add_component(
        &mut self,
        parent: Option<&ComponentReference>,
        component_type: usize,
        input_data: Box<dyn Any>,
    ) -> ComponentReference {
        let reference_index = Rc::new(RefCell::new(self.component_data.len()));
        let reference_clone = reference_index.clone();

        self.component_data.push(ComponentDataHolder {
            reference_index,
            component_type,
            children: Vec::new(),
            parent: parent.and(Some(parent.unwrap().clone())),
            data: self.fn_init.get(component_type).unwrap()(input_data),
        });

        reference_clone
    }

    pub fn insert_component(
        &mut self,
        parent: Option<ComponentReference>,
        index: usize,
        component_type: usize,
        input_data: Box<dyn Any>,
    ) -> ComponentReference {
        let reference_index = Rc::new(RefCell::new(index));
        let reference_clone = reference_index.clone();

        if let Some(parent) = parent.clone() {
            self.get_component_mut(&parent)
                .children
                .push(reference_index.clone());
        }

        for i in index..self.component_data.len() {
            *self.get_component_raw_mut(i).reference_index.borrow_mut() += 1;
        }

        self.component_data.insert(
            index,
            ComponentDataHolder {
                reference_index,
                component_type,
                children: Vec::new(),
                parent,
                data: self.fn_init.get(component_type).unwrap()(input_data),
            },
        );

        reference_clone
    }

    pub fn remove_component(&mut self, reference: ComponentReference) {
        let index = *reference.borrow();
        for i in index + 1..self.component_data.len() {
            *self.get_component_raw_mut(i).reference_index.borrow_mut() -= 1;
        }
        self.component_data.remove(index);
    }

    pub fn get_component(&self, reference: &ComponentReference) -> &ComponentDataHolder {
        self.get_component_raw(*(*reference).borrow())
    }

    pub fn get_component_raw(&self, index: usize) -> &ComponentDataHolder {
        self.component_data.get(index).unwrap()
    }

    pub fn get_component_mut(
        &mut self,
        reference: &ComponentReference,
    ) -> &mut ComponentDataHolder {
        self.get_component_raw_mut(*(*reference).borrow())
    }

    pub fn get_component_raw_mut(&mut self, index: usize) -> &mut ComponentDataHolder {
        self.component_data.get_mut(index).unwrap()
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self) {
        for ComponentDataHolder {
            component_type,
            data,
            ..
        } in self.component_data.iter_mut()
        {
            self.fn_update.get_mut(*component_type).unwrap()(data);
        }
    }

    pub fn render(&mut self) {
        let render_pass = RenderPass::new();
    }
}

pub struct ComponentDataHolder {
    reference_index: ComponentReference,
    component_type: usize,
    children: Vec<ComponentReference>,
    parent: Option<ComponentReference>,
    data: Box<dyn Any>,
}

impl ComponentDataHolder {
    pub fn get_data<T: Any>(&self) -> &T {
        self.data.downcast_ref::<T>().unwrap()
    }
}

pub type ComponentReference = Rc<RefCell<usize>>;

pub trait Component {
    fn init(input: Box<dyn Any>) -> Box<dyn Any>;
    fn update(data: &mut Box<dyn Any>);
    fn render(data: &Box<dyn Any>, pass: &mut RenderPass);
    fn destroy(data: Box<dyn Any>);
}
