extern crate find_folder;
extern crate nalgebra as na;
extern crate piston;
extern crate piston_window;

mod event_traits;
mod rect;
mod simple_map;

use piston_window::*;
use event_traits::RenderHandler;
use std::rc::Rc;

fn main() {
    // prepare piston window
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();

    // prepare assets folder
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();


    // prepare demo texture
    let stone_tile = assets.join("Isometric").join("stone_E.png");
    let stone_tile = Texture::from_path(
        &mut window.factory,
        &stone_tile,
        Flip::None,
        &TextureSettings::new()
        ).unwrap();
    let stone_tile = Rc::new(stone_tile);

    let r = rect::GameRect {
        x: 100.0,
        y: 100.0,
        w: 100.0,
        h: 100.0,
        color: [1.0, 0.0, 0.0, 1.0],
    };

    let s = simple_map::SimpleSprite {
        pos: [100.0, 100.0],
        texture: stone_tile.clone(),
    };

    while let Some(e) = window.next() {
        match e {
            Input::Render(args) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);
                    r.render(args, c, g);
                    s.render(args, c, g);
                });
            }
            _ => {}
        }
    }
}
