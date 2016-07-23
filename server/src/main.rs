#![warn(missing_docs)]
//! Server for the rusty_bots game

extern crate libloading;
extern crate bots_lib;
extern crate glob;

mod plugin_handler;

use bots_lib::map::Map;


const PLUGIN_PATH: &'static str = "user";
const LIB_PREFIX: &'static str = "lib";

fn main() {
    let mut plugins = plugin_handler::PluginHandler::new(PLUGIN_PATH.to_string());
    let users = vec!["user1"];

    for user in users {
        let lib = LIB_PREFIX.to_string() + user;
        plugins.load(lib.clone());

        let welcome_fn = plugins.get_symbol::<fn() -> String>(lib, "welcome");
        println!("{:?}", welcome_fn.unwrap()());
    }

    let map = Map::new();


    println!("Hello, world!");
}
