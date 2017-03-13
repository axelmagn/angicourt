//! Traits related to handling piston window events.

use piston_window::context::Context;
use piston_window::G2d;
use piston_window::{RenderArgs, UpdateArgs};


/// Anything which can be rendered
pub trait RenderHandler {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult;
}

/// Anything which can be updated
pub trait UpdateHandler {
    fn update(&mut self, args: UpdateArgs) -> UpdateResult;
}

pub type RenderResult = Result<bool, ()>;
pub type UpdateResult = Result<(), ()>;
