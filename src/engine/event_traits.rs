//! Traits related to handling piston window events.

use piston_window::context::Context;
use piston_window::{Button, G2d, Motion, RenderArgs, UpdateArgs};


/// Anything which can be rendered.
pub trait RenderHandler {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult;
}

/// Anything which can be updated.
pub trait UpdateHandler {
    fn update(&mut self, args: UpdateArgs) -> UpdateResult;
}

/// Anything which handles an input from the mouse or keyboard.
pub trait ButtonHandler {
    fn press(&mut self, button: Button) -> InputResult;
    fn release(&mut self, button: Button) -> InputResult;
    fn motion(&mut self, motion: Motion) -> InputResult;
}

// the types of these are somewhat fluid until we know what sort of
// errors need to be checked for.
pub type RenderResult = Result<bool, ()>;
pub type UpdateResult = Result<(), ()>;
pub type InputResult = Result<(), ()>;
