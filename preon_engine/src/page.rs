use preon_ecs::ECS;

pub struct Page {
    ecs: ECS,
}

impl Page {
    pub fn new() -> Page {
        Page {
            ecs: ECS::new(),
        }
    }
}