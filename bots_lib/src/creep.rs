use location::Location;
use std::sync::mpsc;

/// A backend creep.
#[derive(Clone, Debug)]
pub struct ServerCreep {
    /// Location of the creep.
    pub location: Location
}

// The user.
// #[derive(Clone, Debug)]
// pub struct User {
//     name: String,
//     creeps: Vec<Creep>
// }


/// A Creep for the script.
#[derive(Clone, Debug)]
pub struct Creep {
    /// Location of the creep.
    pub location: Location,
    console: mpsc::Sender<String>
}

impl Creep {
    /// Returns a new `Creep` with given parameters.
    pub fn new(location: Location, console: mpsc::Sender<String>) -> Creep {
        Creep {
            location: location,
            console: console
        }
    }

    /// Prints a message to the user console
    pub fn print(&self, msg: String) -> bool {
        match self.console.send(msg) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
