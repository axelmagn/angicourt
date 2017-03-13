//! A simple sprite (not meant as a final product)

use piston_window::context::Context;
use piston_window::math::{Vec2d, translate, multiply};
use piston_window::{G2d, G2dTexture, image, RenderArgs, Transformed};
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::cmp::{Ord, Ordering};
use rand::{thread_rng, Rng, SeedableRng, StdRng};

use event_traits::{RenderHandler, RenderResult};


pub struct SimpleSprite {
    pub pos: Vec2d,
    pub texture: Rc<G2dTexture>,
}


impl RenderHandler for SimpleSprite {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult {
        let tex: &G2dTexture = &self.texture;
        let c = context.trans(self.pos[0], self.pos[1]);
        image(tex, c.transform, graphics);
        Ok(true)
    }
}



pub struct SimpleMap {
    pub sprites: Vec<Rc<SimpleSprite>>,
    pub seed: usize,
    pub pos: Vec2d,
    pub width: usize,
    pub height: usize,
}

const TILE_HEIGHT: i64 = 149;
const TILE_WIDTH: i64 = 256;
const TILE_OFFSET_X: i64 = TILE_WIDTH / 2;
const TILE_OFFSET_Y: i64 = TILE_HEIGHT / 2;


#[derive(Eq, PartialEq, Debug)]
struct RenderJob {
    dy: i64,
    dx: i64,
}

impl PartialOrd for RenderJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dy != other.dy {
            self.dy.partial_cmp(&other.dy).map(|o| o.reverse())
        } else {
            self.dx.partial_cmp(&other.dx).map(|o| o.reverse())
        }
    }
}

impl Ord for RenderJob {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dy != other.dy {
            self.dy.cmp(&other.dy).reverse()
        } else {
            self.dx.cmp(&other.dx).reverse()
        }
    }
}



impl RenderHandler for SimpleMap {
    fn render(&self, args: RenderArgs, context: Context, graphics: &mut G2d) -> RenderResult {
        let mut render_queue = BinaryHeap::with_capacity(self.width * self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let row = row as i64;
                let col = col as i64;
                let dx = col * TILE_OFFSET_X + row * TILE_OFFSET_X;
                let dy = col * TILE_OFFSET_Y - row * TILE_OFFSET_Y;
                let job = RenderJob { dx: dx, dy: dy };
                render_queue.push(job);
            }
        }

        let seed: &[_] = &[self.seed];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        // let mut rng = thread_rng();
        let c = context.trans(self.pos[0], self.pos[1]);
        while let Some(job) = render_queue.pop() {
            let c = c.trans(job.dx as f64, job.dy as f64);
            let i = rng.gen_range(0, self.sprites.len());
            self.sprites[i].render(args, c, graphics);
        }
        Ok(true)
    }
}
