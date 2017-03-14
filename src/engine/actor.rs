use engine::component::Component;

/// Any object that can be placed in a level
pub trait Actor: RenderHandler + UpdateHandler {}

pub struct GenericActor {
    root_component: Component,
}
