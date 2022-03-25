use std::any::{Any, TypeId};

use self::{system::{System, SystemId, SysFunc}, component::{Component, ComponentId}, entity::{Entity, EntityId}};

pub mod entity;
pub mod component;
pub mod system;
pub use paste;

pub trait IntoComponentList {
    fn apply(self, ecs: &mut ECS) -> Vec<ComponentId>;
}

pub trait IntoSystem {
    fn exec(self, comps: Vec<&mut Component>);
    fn query() -> Vec<TypeId>;
}

// impl<A: Any> IntoSystem for fn(A) {
//     fn apply(self, ecs: &mut ECS) -> SystemId {
//         ecs.add_system_raw(|comps| {
//             let c_0: &mut A = comps[0].data.downcast_mut().unwrap();
//             self(c_0);
//         }, vec![TypeId::of::<A>()])
//     }
// }

macro_rules! gen_impls {
    (($lidx:tt => $left:ident), $(($ridx:tt => $right:ident),)*) => {
        impl<$left, $($right),*> IntoComponentList for ($left, $($right),*)
        where
            $left: std::any::Any,
            $($right: std::any::Any),*
        {
            fn apply(self, ecs: &mut ECS) -> Vec<ComponentId> {
                vec![
                    ecs.add_component(self.$lidx),
                    $(ecs.add_component(self.$ridx)),*
                ]
            }
        }

        impl<$left, $($right),*> IntoSystem for fn(&mut $left, $(&mut $right),*)->()
        where
            $left: std::any::Any,
            $($right: std::any::Any,)*
        {
            fn exec(self, mut comps: Vec<&mut Component>) {
                paste::paste! {
                    let [<c_ $lidx>]: &mut $left = comps.pop().unwrap().data.downcast_mut().unwrap();
                    $(let [<c_ $ridx>]: &mut $right = comps.pop().unwrap().data.downcast_mut().unwrap();)*
                    self([<c_ $lidx>], $([<c_ $ridx>]),*)
                }
            }

            fn query() -> Vec<TypeId> {
                vec![
                    TypeId::of::<$left>(),
                    $(TypeId::of::<$right>(),)*
                ]
            }
        }

        gen_impls!($(($ridx => $right),)*);
    };
    () => {};
}

#[macro_export] macro_rules! fn_with_args {
    (1) => {fn(&mut _)};
    (2) => {fn(&mut _, &mut _)};
    (3) => {fn(&mut _, &mut _, &mut _)};
    (4) => {fn(&mut _, &mut _, &mut _, &mut _)};
    (5) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _)};
    (6) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (7) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (8) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (9) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (10) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (11) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (12) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (13) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (14) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (15) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (16) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (17) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (18) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (19) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (20) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (21) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (22) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (23) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (24) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (25) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
    (26) => {fn(&mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _, &mut _)};
}

#[macro_export] macro_rules! system {
    ($name:ident, $argcount:tt) => {
        mod $name {
            use preon_ecs::fn_with_args;
            pub fn system(comps: Vec<&mut preon_ecs::component::Component>) {
                use preon_ecs::IntoSystem;
                IntoSystem::exec(super::$name as fn_with_args!($argcount), comps);
            }
        }
    };
}

gen_impls!(
    (25 => A),
    (24 => B),
    (23 => C),
    (22 => D),
    (21 => E),
    (20 => F),
    (19 => G),
    (18 => H),
    (17 => I),
    (16 => J),
    (15 => K),
    (14 => L),
    (13 => M),
    (12 => N),
    (11 => O),
    (10 => P),
    (9 => Q),
    (8 => R),
    (7 => S),
    (6 => T),
    (5 => U),
    (4 => V),
    (3 => W),
    (2 => X),
    (1 => Y),
    (0 => Z),
);

pub struct ECS {
    entities: Vec<Entity>,
    components: Vec<Component>,
    systems: Vec<System>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Vec::new(),
            components: Vec::new(),
            systems: Vec::new()
        }
    }

    pub fn update(&mut self) {
        
    }

    #[inline]
    pub fn add_entity<T: IntoComponentList>(&mut self, components: T) -> EntityId {
        let ids = components.apply(self);
        let e_id = EntityId(self.entities.len());

        self.entities.push(Entity {
            id: e_id,
            components: ids,
        });

        e_id
    }

    #[inline]
    pub fn add_component<T: Any>(&mut self, data: T) -> ComponentId {
        let c_id = ComponentId(self.components.len(), data.type_id());
        self.components.push(Component {
            data: Box::new(data),
            id: c_id,
        });

        c_id
    }

    #[inline]
    pub fn add_system<T>(&mut self, _system_function: T, caller_function: SysFunc) -> SystemId
    where
        T: IntoSystem
    {
        self.add_system_raw(caller_function, T::query())
    }

    #[inline]
    pub fn add_system_raw(&mut self, func: fn(Vec<&mut Component>), query: Vec<TypeId>) -> SystemId {
        let s_id = SystemId(self.systems.len());
        self.systems.push(System {
            function: func,
            id: s_id,
            query,
        });

        s_id
    }

    #[inline]
    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(id.0)
    }
    
    #[inline]
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id.0)
    }

    #[inline]
    pub fn get_component(&self, id: EntityId) -> Option<&Component> {
        self.components.get(id.0)
    }
    
    #[inline]
    pub fn get_component_mut(&mut self, id: EntityId) -> Option<&mut Component> {
        self.components.get_mut(id.0)
    }

    #[inline]
    pub fn get_system(&self, id: EntityId) -> Option<&System> {
        self.systems.get(id.0)
    }
    
    #[inline]
    pub fn get_system_mut(&mut self, id: EntityId) -> Option<&mut System> {
        self.systems.get_mut(id.0)
    }
}