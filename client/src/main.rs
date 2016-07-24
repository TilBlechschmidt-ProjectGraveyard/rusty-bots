#![warn(missing_docs)]
//! Client for rusty-bots displaying the world

extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate gfx;
extern crate bots_lib;

use std::cmp;

use std::rc::Rc;
use std::path::PathBuf;
use gfx::Factory;

use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While
};

use bots_lib::map::{ Map, TileType };
use bots_lib::location::Location;

const TEXTURE_FILE_EXTENSION: &'static str = ".png";

const SCALE: f64 = 10.0;

struct GameTexture<R: gfx::Resources, F: Factory<R>> {
    path: PathBuf,
    factory: F,
    tex: Option<Rc<Texture<R>>>
}

impl<R: gfx::Resources, F: Factory<R>> GameTexture<R, F> {
    pub fn new(name: String, texture_path: PathBuf, factory: F) -> Result<GameTexture<R, F>, &'static str> {
        let path = texture_path.join(name + TEXTURE_FILE_EXTENSION);
        if path.exists() {
            Ok(GameTexture {
                path: path,
                factory: factory,
                tex: None
            })
        } else {
            Err("File does not exist!")
        }
    }

    pub fn texture(&mut self) -> Rc<Texture<R>> {
        if self.tex.is_some() {
            self.tex.clone().unwrap()
        } else {
            self.tex = Some(Rc::new(Texture::from_path(
                        &mut self.factory,
                        self.path.clone(),
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap()));
            self.tex.clone().unwrap()
        }
    }

    pub fn to_sprite(&mut self) -> Sprite<Texture<R>> {
        Sprite::from_texture(self.texture())
    }
}

fn main() {
    let (width, height) = (1440, 1440);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let textures = assets.join("textures/");

    let mut scene = Scene::new();


    let mut grass = GameTexture::new("grass".to_string(), textures.clone(), window.factory.clone()).unwrap();
    let mut rock = GameTexture::new("rock".to_string(), textures.clone(), window.factory.clone()).unwrap();
    let mut water = GameTexture::new("water".to_string(), textures.clone(), window.factory.clone()).unwrap();

    let window_offset = Location::new((width / 2) as i32, (height / 2) as i32);

    let mut map = Map::new();
    let map_section = map.get_map_section(Location::new(0, 0), 60);
    let map_section = map_section + map.get_map_section(Location::new(50, 0), 60);








    // let id;
    // let mut sprite = grass.to_sprite();
    // sprite.set_scale(0.5, 0.5);
    // let position = [sprite.bounding_box()[2] as f64 / 2.0, sprite.bounding_box()[3] as f64 / 2.0];
    // sprite.set_position(position[0], position[1]);
    // // println!("{:?}", sprite.bounding_box()[2]);
    //
    // id = scene.add_child(sprite);

    // Run a sequence of animations.
    // let seq = Sequence(vec![
    //     Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
    //     Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
    //     Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
    //     Wait(0.5),
    //     Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Blink(1.0, 5)),
    //     While(Box::new(WaitForever), vec![
    //         Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
    //         Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
    //     ]),
    // ]);
    // scene.run(id, &seq);
    //
    // // This animation and the one above can run in parallel.
    // let rotate = Action(Ease(EaseFunction::ExponentialInOut,
    //     Box::new(RotateTo(2.0, 360.0))));
    // scene.run(id, &rotate);
    // scene.run(id, &Action(Ease(EaseFunction::QuadraticInOut, Box::new(Animation::MoveTo(2.0, 500.0, 500.0)))));
    // scene.child_mut(id).unwrap().set_position(20.0, 20.0);

    let mut tile_ids = Vec::new();
    while let Some(e) = window.next() {
        for _ in 0..tile_ids.len() {
            scene.remove_child(tile_ids.pop().unwrap()).unwrap();
        }
        for loc in map_section.tiles.keys() {
            let tile = map_section.tiles.get(loc).unwrap();
            let new_loc = *loc * SCALE + window_offset;

            if (new_loc.x > -(SCALE as i32) && new_loc.x < (width + (SCALE as u32)) as i32) && (new_loc.y > -(SCALE as i32) && new_loc.y < (height + (SCALE as u32)) as i32) {
                let mut sprite = match tile.terrain_type {
                    TileType::Plain => {
                        grass.to_sprite()
                    },
                    TileType::Rock => {
                        rock.to_sprite()
                    },
                    TileType::Water => {
                        water.to_sprite()
                    }
                };
                let sprite_size = sprite.bounding_box()[2];
                sprite.set_scale(SCALE / sprite_size, SCALE / sprite_size);
                sprite.set_position(new_loc.x as f64, new_loc.y as f64);
                tile_ids.push(scene.add_child(sprite));
            }
        }

        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([38.0 / 255.0, 50.0 / 255.0, 56.0 / 255.0, 1.0], g);
            scene.draw(c.transform, g);
        });
        // if let Some(_) = e.press_args() {
        //     scene.toggle(id, &seq);
        //     scene.toggle(id, &rotate);
        // }
    }
}
