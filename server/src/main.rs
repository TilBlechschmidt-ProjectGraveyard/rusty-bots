#![warn(missing_docs)]
//! Server for the rusty_bots game

extern crate libloading;
extern crate bots_lib;
extern crate glob;

mod plugin_handler;

use bots_lib::map::Map;
use bots_lib::location::Location;
use bots_lib::creep::{ServerCreep, Creep, User};
use bots_lib::memory::Memory;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;


const PLUGIN_PATH: &'static str = "user";

fn main() {
    let mut plugins = plugin_handler::PluginHandler::new(PLUGIN_PATH.to_string());


    // let (tx, rx) = mpsc::channel::<String>();
    //
    // thread::spawn(move || {
    //     loop {
    //         match rx.recv() {
    //             Ok(msg) => println!("{:?}", msg),
    //             _ => break
    //         }
    //     }
    // });


    let mut map = Map::new();


    let users = vec!["user1"];
    let mut g_creep_id = 0;
    for username in users.iter() {
        plugins.load(username.to_string());

        let (tx, rx) = mpsc::channel::<String>(); // TODO use rx
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(msg) => println!("{:?}", msg), //TODO use user console
                    _ => break
                }
            }
        });

        let user_id = map.add_user(User::new(username.to_string(), tx.clone()));
        g_creep_id = map.add_creep(ServerCreep::new(Location::new(0, 0), user_id)).unwrap();
        // map.update_user_tick(user_id, );

        match plugins.get_symbol::<fn(Creep)>(username.to_string(), "init") {
            Some(init_fn) => {
                println!("init");
                for creep in map.get_user_creeps(user_id).unwrap() {
                    init_fn(creep);
                }
            },
            None => {}
        }

        // let mut memory: Memory = Memory::new();
        //
        // let memory_getter = Getter::new(|_| {memory});
        //
        // let loc = Location::new(0, 0);
        // let map_section = map.get_map_section(loc, 40);
        // let creep = Creep::new(loc, tx.clone());
        //
        // let welcome_fn = plugins.get_symbol::<fn(Creep) -> usize>(username.to_string(), "welcome");
        // println!("{:?}", welcome_fn.unwrap()(creep));
    }
    loop {
        map.move_creep(g_creep_id, Location::new(100, 100));
        for (&user_id, user) in map.users.iter() {
            match plugins.get_symbol::<fn(Creep)>(user.name.clone(), "tick") {
                Some(tick_fn) => {
                    println!("tick");
                    for creep in map.get_user_creeps(user_id).unwrap() {
                        tick_fn(creep);
                    }
                },
                None => {}
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }

    // println!("Map");
}
