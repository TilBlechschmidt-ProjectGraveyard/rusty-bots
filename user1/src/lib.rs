extern crate bots_lib;

use bots_lib::creep::Creep;

#[no_mangle]
pub fn welcome(creep: Creep) -> usize {
    // creep.print("Hello".to_string());
    creep.print(format!("Map: {:?}", creep.location));
    // console.push("Hello".as_bytes());
    // println!();
    // format!("Hello2 {:?}", pos)
    //"Hello".to_string()
    5
}
