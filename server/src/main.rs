#![warn(missing_docs)]
//! Server for the rusty_bots game

extern crate libloading;
extern crate bots_lib;
extern crate glob;

mod plugin_handler;

use bots_lib::map::Map;
use bots_lib::location::Location;
use bots_lib::creep::Creep;

use std::thread;
use std::sync::mpsc;


const PLUGIN_PATH: &'static str = "user";

fn main() {
    let mut plugins = plugin_handler::PluginHandler::new(PLUGIN_PATH.to_string());
    let users = vec!["user1"];

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => println!("{:?}", msg),
                _ => break
            }
        }
    });

    let mut map = Map::new();

    for user in users.iter() {
        plugins.load(user.to_string());



        let loc = Location::new(0, 0);
        let map_section = map.get_map_section(loc, 40);
        let creep = Creep::new(loc, tx.clone());

        let welcome_fn = plugins.get_symbol::<fn(Creep) -> usize>(user.to_string(), "welcome");
        println!("{:?}", welcome_fn.unwrap()(creep));
    }

    println!("Map");

    // for i in 0..1 {
    //     let loc = Location::new(i, 0);
    //     let map_section = map.get_map_section(loc, 40);
    //     let map_section = map_section + map.get_map_section(loc + (20, 0), 40);
    //     map_section.print(loc, 40);
    //     thread::sleep(Duration::from_millis(30));
    // }

    // for user in users.iter() {
    //     plugins.load(user.to_string());
    //
    //
    //
    //     let loc = Location::new(0, 0);
    //     let creep = Creep::new(map.get_map_section(loc, 40), loc, tx.clone());
    //
    //     let welcome_fn = plugins.get_symbol::<fn(Creep) -> usize>(user.to_string(), "welcome");
    //     println!("{:?}", welcome_fn.unwrap()(creep));
    // }

    // println!("Hello, world!");
}
