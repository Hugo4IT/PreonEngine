use preon_core::PreonCore;

pub trait PreonRenderer {
    fn init(core: &PreonCore) -> Self;
    fn start(self: &mut Self, core: &PreonCore);
    fn update(self: &mut Self, core: &PreonCore);
    fn render(self: &Self, core: &PreonCore);
}