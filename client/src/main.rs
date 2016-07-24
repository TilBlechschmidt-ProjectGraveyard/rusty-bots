#![warn(missing_docs)]
//! Client for rusty-bots displaying the world

extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate gfx;
extern crate uuid;
extern crate drag_controller;
extern crate bots_lib;

use std::cmp;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use std::rc::Rc;
use std::path::PathBuf;
use gfx::Factory;
use uuid::Uuid;

use piston_window::*;
use drag_controller::*;
use sprite::*;
// use ai_behavior::{
//     Action,
//     Sequence,
//     Wait,
//     WaitForever,
//     While
// };

use bots_lib::map::{ Map, MapSection, CustomMap, TileDelta, TileType };
use bots_lib::location::Location;

const TEXTURE_FILE_EXTENSION: &'static str = ".png";

const SCALE: f64 = 8.0;

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

    let window_offset = Location::new((width / 2) as i32, (height / 2) as i32);

    // let mut map = Map::new();
    // let map_section = map.get_map_section(Location::new(0, 0), 60);
    // let map_section_new = map.get_map_section(Location::new(50, 0), 60);

    // let mut tile_ids = Vec::new();

    let (delta_tx, delta_rx) = mpsc::channel();

    thread::spawn(move || {
        let mut map = Map::new();
        let mut current_section = MapSection::new(HashMap::new());
        let mut i = 0;
        // loop {
            let new_section = map.get_map_section(Location::new(i, 0), 60);
            delta_tx.send(current_section - new_section.clone()).unwrap();
            current_section = new_section;
            // thread::sleep(std::time::Duration::from_millis(100));
            // i+=1;
        // }
    });

    window.set_ups(30);
    window.set_max_fps(30);

    let mut sprite_ids = CustomMap::new(HashMap::new());
    let mut drag_controller = DragController::new();
    let mut map_location = (0.0, 0.0);

    fn move_viewport<R: gfx::Resources>(map_location: &mut (f64, f64), delta: (f64, f64), sprite_ids: &mut HashMap<Location, Uuid>, scene: &mut Scene<Texture<R>>) {
        map_location.0 += delta.0;
        map_location.1 += delta.1;
        // map_location = (map_location.0 + delta.0, map_location.1 + delta.1);
        for sprite_id in sprite_ids.values() {
            let sprite = scene.child_mut(*sprite_id).unwrap();
            let position = sprite.get_position();
            sprite.set_position(position.0 - (delta.0 as f64 * SCALE), position.1 - (delta.1 as f64 * SCALE));
        }
    };

    let mut drag_position = [0.0, 0.0];
    while let Some(e) = window.next() {

        // Update sprites
        match delta_rx.try_recv() {
            Ok(delta_map) => {
                println!("DRAWING SPRITES");
                for loc in delta_map.tiles.keys() {
                    let tile = delta_map.tiles.get(loc).unwrap();

                    // if (new_loc.x > -(SCALE as i32) && new_loc.x < (width + (SCALE as u32)) as i32) && (new_loc.y > -(SCALE as i32) && new_loc.y < (height + (SCALE as u32)) as i32) {
                        match *tile {
                            TileDelta::Removed(_) => {
                                match sprite_ids.tiles.remove(&loc) {
                                    Some(old_sprite) => {
                                        scene.remove_child(old_sprite).unwrap();
                                    },
                                    None => { unreachable!() }
                                }
                            },
                            TileDelta::New(ref tile_type) | TileDelta::Changed(ref tile_type) => {
                                let mut sprite = match *tile_type {
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
                                let floc = (loc.x as f64, loc.y as f64);
                                // let new_loc = (loc - map_location) * SCALE + window_offset;
                                sprite.set_scale(SCALE / sprite_size, SCALE / sprite_size);
                                sprite.set_position(
                                    (floc.0 - map_location.0) * SCALE + window_offset.x as f64,
                                    (floc.1 - map_location.1) * SCALE + window_offset.y as f64);

                                match sprite_ids.tiles.insert(*loc, scene.add_child(sprite)) {
                                    Some(old_sprite) => {
                                        scene.remove_child(old_sprite).unwrap();
                                    },
                                    None => {}
                                }
                            }
                        }
                    // }
                }
            },
            Err(_) => {}
        }

        // let delta = (-0.1, 0.0);
        // move_viewport(&mut map_location, delta, &mut sprite_ids.tiles, &mut scene);


        drag_controller.event(&e, |drag| {
            match drag {
                Drag::Interrupt => {
                    // println!("Drag got interrupted");
                },
                Drag::Start(x, y) => {
                    drag_position = [x, y];
                    // println!("Dragging from {} {}", x, y);
                },
                Drag::Move(x, y) | Drag::End(x, y) => {
                    let delta = ((drag_position[0] - x) / SCALE, (drag_position[1] - y) / SCALE);
                    // println!("Delta: {:?}", delta);
                    move_viewport(&mut map_location, delta, &mut sprite_ids.tiles, &mut scene);
                    drag_position = [x, y];
                }
            }
            true
        });

        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([38.0 / 255.0, 50.0 / 255.0, 56.0 / 255.0, 1.0], g);

            scene.draw(c.transform, g);
        });
        // thread::sleep(std::time::Duration::from_millis(500));
        // if let Some(_) = e.press_args() {
        //     scene.toggle(id, &seq);
        //     scene.toggle(id, &rotate);
        // }
    }
}
