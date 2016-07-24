extern crate bots_lib;

use std::sync::mpsc;
use bots_lib::location::Location;

#[no_mangle]
pub fn welcome(tx: mpsc::Sender<String>) -> usize {

    // CONSOLE = Some(tx.clone());

    let pos = Location::new(2, 2) + (-1, -1);
    tx.send("Hello".to_string()).unwrap();
    tx.send(format!("Hello2 {:?}", pos)).unwrap();
    // console.push("Hello".as_bytes());
    // println!();
    // format!("Hello2 {:?}", pos)
    //"Hello".to_string()
    5
}
