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
        let plugin_name = LIB_PREFIX.to_string() + user;
        let plugin = plugins.load(plugin_name.clone());
        let welcome_fn = plugin.unwrap().get_welcome().unwrap();

        // let welcome_fn = plugins.get_symbol::<fn() -> String>(lib, "welcome");
        unsafe {
            println!("{:?}", welcome_fn());
        }
        // welcome_fn
    }

    // plugins.reset();

    println!("Map");
    // let mut map = Map::new();
    //
    // for i in 0..100 {
    //     // println!("{:?}", i);
    //     let map_section = map.get_map_section(Location::new(i, 0), 40);
    //     // println!("{}", i);
    //     map_section.print();
    //     thread::sleep(Duration::from_millis(30))
    // }



    println!("Hello, world!");
}
