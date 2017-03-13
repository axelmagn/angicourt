//! A simple sprite (not meant as a final product)

use na::Vector2;
use piston_window::context::Context;
use piston_window::{G2d, G2dTexture, image, RenderArgs};
use piston_window::math::{Vec2d, translate, multiply};
use std::rc::Rc;

use event_traits::{RenderHandler, RenderResult};


pub struct SimpleSprite {
    pub pos: Vec2d,
    pub texture: Rc<G2dTexture>,
}

impl RenderHandler for SimpleSprite {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult {
        let tex: &G2dTexture = &self.texture;
        let transform = multiply(context.transform, translate(self.pos));
        image(tex, transform, graphics);
        Ok(true)
    }
}
