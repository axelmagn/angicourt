extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate specs;

mod game;
mod engine;

use piston_window::*;
use engine::event_traits::RenderHandler;
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

    let stone_tile2 = assets.join("Isometric").join("stoneTile_E.png");
    let stone_tile2 = Texture::from_path(
        &mut window.factory,
        &stone_tile2,
        Flip::None,
        &TextureSettings::new()
        ).unwrap();
    let stone_tile2 = Rc::new(stone_tile2);


    let r = game::rect::GameRect {
        x: 100.0,
        y: 100.0,
        w: 100.0,
        h: 100.0,
        color: [1.0, 0.0, 0.0, 1.0],
    };

    let s = game::simple_map::SimpleSprite {
        texture: stone_tile.clone(),
        pos: [0.0, 0.0],
    };
    let s = Rc::new(s);

    let s2 = game::simple_map::SimpleSprite {
        texture: stone_tile2.clone(),
        pos: [0.0, 0.0],
    };
    let s2 = Rc::new(s2);

    let m = game::simple_map::SimpleMap {
        sprites: vec!(s.clone(), s2.clone()),
        seed: 42,
        pos: [512.0, 512.0],
        width: 10,
        height: 10,
    };

    while let Some(e) = window.next() {
        match e {
            Input::Render(args) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);
                    Ok(())
                        .and_then(|_| r.render(args, c, g))
                        .and_then(|_| m.render(args, c, g))
                        .unwrap();
                });
            }
            _ => {}
        }
    }
}
