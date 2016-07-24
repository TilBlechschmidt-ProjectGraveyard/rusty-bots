extern crate bots_lib;

use bots_lib::creep::Creep;

#[no_mangle]
pub fn init(creep: Creep) {
    creep.print(format!("Init: {:?}", creep.location));
}

#[no_mangle]
pub fn tick(creep: Creep) {
    creep.print(format!("Tick: {:?}", creep.location));
}
