use preon_engine::PreonEngine;

pub trait PreonApp {
    fn build(engine: &mut PreonEngine);
}

pub fn start<T: PreonApp>() {
    #[cfg(feature = "logging")]
    env_logger::init();
}