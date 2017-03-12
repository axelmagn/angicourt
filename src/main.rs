extern crate find_folder;
extern crate piston;
extern crate piston_window;

mod event_traits;
mod rect;

use piston_window::*;
use event_traits::RenderHandler;

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

    let r = rect::GameRect {
        x: 100.0,
        y: 100.0,
        w: 100.0,
        h: 100.0,
        color: [1.0, 0.0, 0.0, 1.0],
    };

    while let Some(e) = window.next() {
        match e {
            Input::Render(args) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);
                    r.render(args, c, g);
                    image(&stone_tile, c.transform, g);
                });
            }
            _ => {}
        }
    }
}
