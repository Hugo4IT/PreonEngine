pub struct PreonRenderPipeline {
    pub drawables: Option<Vec<Box<dyn PreonShape>>>,
    pub children: Option<Vec<Option<PreonRenderPipeline>>>,
}

pub trait PreonShape {
    // fn render<T: PreonRenderer>(&mut self);
}
