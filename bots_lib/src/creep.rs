use location::Location;
use std::sync::mpsc;
use map::ID;
use memory::Memory;
use std::thread;
use std::marker;


/// A two way channel.
#[derive(Debug)]
pub struct Getter<R, T> {
    sender: mpsc::Sender<R>,
    receiver: mpsc::Receiver<T>
}

impl<T: 'static + marker::Send, R: 'static + marker::Send> Getter<R, T> {
    /// Returns a new `Getter`.
    pub fn new<F>(f: F) -> Getter<R, T> where F: Fn(R) -> T + 'static + marker::Send {
        let (thread_sender, receiver) = mpsc::channel::<T>();
        let (sender, thread_receiver) = mpsc::channel::<R>();
        thread::spawn(move || {
            loop {
                match thread_receiver.recv() {
                    Ok(data) => {
                        match thread_sender.send(f(data)) {
                            _ => {}
                        }
                    },
                    _ => break
                };
            }
        });

        Getter {
            sender: sender,
            receiver: receiver
        }
    }

    /// Requests and returns data.
    pub fn get(&self, data: R) -> Option<T> {
        match self.sender.send(data) {
            Ok(_) => {
                match self.receiver.recv() {
                    Ok(recv_data) => Some(recv_data),
                    _ => None
                }
            },
            _ => { None }
        }
    }
}

/// A backend creep.
#[derive(Clone, Debug)]
pub struct ServerCreep {
    /// Location of the creep.
    pub location: Location,
    /// Memory of the creep.
    pub memory: Memory,

    pub user_id: ID
}

impl ServerCreep {
    pub fn new(location: Location, user_id: ID) -> ServerCreep {
        ServerCreep {
            location: location,
            memory: Memory::new(),
            user_id: user_id
        }
    }
}

/// The user.
#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub creeps: Vec<ID>,
    pub print_sender: mpsc::Sender<String>
}

impl User {
    /// Returns a new user with a given name and a `mpsc::Sender` to send to user console.
    pub fn new(name: String, print_sender: mpsc::Sender<String>) -> User {
        User {
            name: name,
            creeps: Vec::new(),
            print_sender: print_sender
        }
    }
}


/// A Creep for the script.
#[derive(Debug)]
pub struct Creep {
    /// Location of the creep.
    pub location: Location,
    console_channel: mpsc::Sender<String>,
}

impl Creep {
    /// Returns a new `Creep` with given parameters.
    pub fn new(location: Location, console_channel: mpsc::Sender<String>) -> Creep {
        Creep {
            location: location,
            console_channel: console_channel
        }
    }

    /// Prints a message to the user console.
    pub fn print(&self, msg: String) -> bool {
        match self.console_channel.send(msg) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
