
use map::MapSection;
use location::Location;
use std::sync::mpsc;

pub struct ServerCreep {


}




pub struct Creep {
    // pub map: MapSection,
    pub location: Location,
    console: mpsc::Sender<String>
}

impl Creep {
    pub fn new(map: MapSection, location: Location, console: mpsc::Sender<String>) -> Creep {
        Creep {
            // map: map,
            location: location,
            console: console
        }
    }

    pub fn print(&self, msg: String) -> bool {
        match self.console.send(msg) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
