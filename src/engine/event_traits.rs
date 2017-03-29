//! Traits related to handling piston window events.

use piston_window::context::Context;
use piston_window::{Button, G2d, Motion, RenderArgs, UpdateArgs};
use specs;


/// Anything which can be rendered.
pub trait RenderHandler {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult;
}

impl specs::Component for RenderHandler {
    type storage = specs::VecStorage<Rc<RenderHandler>>;
}

/// Anything which can be updated.
pub trait UpdateHandler {
    fn update(&mut self, args: UpdateArgs) -> UpdateResult;
}

impl specs::Component for UpdateHandler {
    type storage = specs::VecStorage<Rc<UpdateHandler>>;
}

/// Anything which handles an input from the mouse or keyboard.
pub trait ButtonHandler {
    fn press(&mut self, button: Button) -> InputResult;
    fn release(&mut self, button: Button) -> InputResult;
    fn motion(&mut self, motion: Motion) -> InputResult;
}

impl specs::Component for ButtonHandler {
    type storage = specs::HashMapStorage<Rc<ButtonHandler>>;
}

// the types of these are somewhat fluid until we know what sort of
// errors need to be checked for.
pub type RenderResult = Result<bool, ()>;
pub type UpdateResult = Result<(), ()>;
pub type InputResult = Result<(), ()>;
