use piston_window::context::Context;
use piston_window::G2d;
use piston_window::{RenderArgs, UpdateArgs};
use piston_window::rectangle;

use event_traits::{RenderHandler, RenderResult};

pub struct GameRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub color: [f32; 4],
}

impl RenderHandler for GameRect {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult {
        rectangle(self.color,
                  [self.x, self.y, self.w, self.h],
                  context.transform,
                  graphics);
        Ok(true)
    }
}
