#![warn(missing_docs)]
//! Server for the rusty_bots game

extern crate libloading;
extern crate bots_lib;
extern crate glob;

mod plugin_handler;

use bots_lib::map::Map;
use bots_lib::location::Location;

use std::time::Duration;
use std::thread;


const PLUGIN_PATH: &'static str = "user";
const LIB_PREFIX: &'static str = "lib";

fn main() {
    let mut plugins = plugin_handler::PluginHandler::new(PLUGIN_PATH.to_string());
    let users = vec!["user1"];

    for user in users {
        let lib = LIB_PREFIX.to_string() + user;
        plugins.load(lib.clone());

        let welcome_fn = plugins.get_symbol::<fn() -> String>(lib, "welcome");
        println!("{:?}", welcome_fn.unwrap());
    }

    println!("Map");
    let mut map = Map::new();

    for i in 0..100 {
        println!("{:?}", i);
        let map_section = map.get_map_section(Location::new(0, 0), 60);
        // println!("{}", i);
        // map_section.print();
        // thread::sleep(Duration::from_millis(10))
    }



    println!("Hello, world!");
}
