use utils::PreonData;

pub mod utils;

pub struct PreonCore {
    pub type_free_functions:    Vec<fn(&PreonData)>,
    pub init_functions:         Vec<fn() -> PreonData>,
    pub update_functions:       Vec<fn(&mut PreonData)>,
    pub free_functions:         Vec<fn(&PreonData)>,
    pub instance_data:          Vec<PreonData>,
    pub type_data:              Vec<PreonData>,
}

impl PreonCore {
    pub fn init() -> Self {

        Self {
            type_free_functions: Vec::new(),
            init_functions: Vec::new(),
            update_functions: Vec::new(),
            free_functions: Vec::new(),
            instance_data: Vec::new(),
            type_data: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        for (index, data) in self.instance_data.iter_mut().enumerate() {
            self.update_functions.get(index).unwrap().to_owned()(data);
        }
    }

    pub fn free(&mut self) {
        for (index, data) in self.instance_data.iter().enumerate() {
            self.free_functions.get(index).unwrap()(data);
        }
        for (index, data) in self.type_data.iter().enumerate() {
            self.type_free_functions.get(index).unwrap()(data);
        }
    }

    pub fn register(
        &mut self,
        type_init:  fn() -> PreonData,
        type_free:  fn(&PreonData),
        init:       fn() -> PreonData,
        update:     fn(&mut PreonData),
        free:       fn(&PreonData),
    ) -> usize {
        let type_data = type_init();

        self.type_data.push(type_data);

        self.type_free_functions.push(type_free);
        self.init_functions.push(init);
        self.update_functions.push(update);
        self.free_functions.push(free);

        self.free_functions.len() - 1
    }
}

pub trait PreonRenderer {
    fn init() -> Self;
    fn start(&mut self, core: &PreonCore);
    fn update(&mut self, core: &PreonCore) -> bool;
    fn render(&mut self, core: &mut PreonCore);

    fn register(&mut self, core: &mut PreonCore);
}


pub trait Renderable<T> {}
pub trait PreonShape {
    fn get_renderable<T: PreonRenderer>(&self) -> dyn Renderable<T>;
}