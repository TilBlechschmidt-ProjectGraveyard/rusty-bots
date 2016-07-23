extern crate bots_lib;

use bots_lib::location::Location;

#[no_mangle]
pub fn welcome() -> String {
    let pos = Location::new(2, 2) + (-1, -1);
    println!("{:?}", pos);
    "Hello2".to_string()
}
