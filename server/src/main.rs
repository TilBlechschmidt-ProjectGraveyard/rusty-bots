#![warn(missing_docs)]

extern crate libloading;
extern crate bots_lib;
extern crate glob;

mod plugin_handler;

fn main() {
    let mut plugins = plugin_handler::PluginHandler::new("/Volumes/Data/Programming/rusty-bots/server/user".to_string());

    plugins.load("user1".to_string());

    let welcome_fn = plugins.get_symbol::<fn() -> String>("user1".to_string(), "welcome");

    println!("{:?}", welcome_fn.unwrap()());

    println!("Hello, world!");
}
