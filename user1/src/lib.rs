extern crate bots_lib;

use bots_lib::location::Location;
use bots_lib::creep::Creep;

#[no_mangle]
pub fn welcome(creep: Creep) -> usize {

    // CONSOLE = Some(tx.clone());

    let pos = Location::new(2, 2) + (-1, -1);
    // creep.print("Hello".to_string());
    // creep.print(format!("Hello2 {:?}", pos));
    // console.push("Hello".as_bytes());
    // println!();
    // format!("Hello2 {:?}", pos)
    //"Hello".to_string()
    5
}
