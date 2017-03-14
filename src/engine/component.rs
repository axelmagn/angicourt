use piston_window::math::{Matrix2d, identity};
use std::rc::Rc;

use engine::event_traits::{UpdateHandler, RenderHandler};

/// An actor component encapsulating behavior.
///
/// Components can handle update and render events, they are transformable, and they can have
/// children.
pub struct Component {
    transform: Matrix2d,
    children: Vec<Rc<Component>>,
    render_handlers: Vec<Rc<RenderHandler>>,
    update_handlers: Vec<Rc<UpdateHandler>>,
}

impl Component {
    /// Create a new component.
    pub fn new() -> Self {
        Component {
            transform: identity(),
            children: Vec::new(),
            render_handlers: Vec::new(),
            update_handlers: Vec::new(),
        }
    }

    pub fn add_behavior<T: Any>(&mut self, behavior: T) -> Result<(), ()> {}
}

pub enum ComponentError {
    /// Thrown when a behavior is passed to a component that does not implement the expected
    /// traits.
    NotBehavior,
}

#[test]
fn test_new_component() {
    let c = Component::new();
}
