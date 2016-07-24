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

    for user in users.iter() {
        plugins.load(user.to_string());


        let welcome_fn = plugins.get_symbol::<fn(mpsc::Sender<String>) -> usize>(user.to_string(), "welcome");
        // let _ = welcome_fn.unwrap()();
        println!("{:?}", welcome_fn.unwrap()(tx.clone()));
    }

    println!("Map");
    let mut map = Map::new();

    for i in 0..800 {
        // println!("{:?}", i);
        let map_section = map.get_map_section(Location::new(i, 0), 40);
        // println!("{}", i);
        map_section.print();
        thread::sleep(Duration::from_millis(30));
    }

    for user in users.iter() {
        plugins.load(user.to_string());

        let welcome_fn = plugins.get_symbol::<fn(mpsc::Sender<String>) -> usize>(user.to_string(), "welcome");
        // let _ = welcome_fn.unwrap()();
        println!("{:?}", welcome_fn.unwrap()(tx.clone()));
    }
    tx.send("stop".to_string()).unwrap();



    // println!("Hello, world!");
}
