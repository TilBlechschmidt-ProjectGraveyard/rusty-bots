use std::collections::HashMap;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable, Decodable};
use std::sync::mpsc;
use std::thread;


pub enum MemoryProtocol {
    Write(String, Vec<u8>),
    Read(String),
    Delete(String),
    Clear
}

pub struct PublicMemory {
    sender: mpsc::Sender<MemoryProtocol>,
    receiver: mpsc::Receiver<Option<Vec<u8>>>
}

impl PublicMemory {
    /// Writes an object to the memory.
    pub fn write<T: Encodable>(&mut self, name: String, object: T) -> bool {
        match encode(&object, SizeLimit::Infinite) {
            Ok(data) => {
                match self.sender.send(MemoryProtocol::Write(name, data)) {
                    Ok(_) => true,
                    _ => false
                }
            },
            _ => false
        }
    }

    /// Reads an object from the memory.
    pub fn read<T: Decodable>(&mut self, name: String) -> Option<T> {
        match self.sender.send(MemoryProtocol::Read(name)) {
            Err(_) => return None,
            _ => {}
        }
        match self.receiver.recv() {
            Ok(option_data) => {
                match option_data {
                    Some(data) => {
                        match decode(&data[..]) {
                            Ok(object) => {
                                Some(object)
                            },
                            Err(_) => None
                        }
                    },
                    None => None
                }
            },
            _ => None
        }
    }

    /// Removes and returns an object from the memory.
    pub fn delete<T: Decodable>(&mut self, name: String) -> Option<T> {
        match self.sender.send(MemoryProtocol::Delete(name)) {
            Err(_) => return None,
            _ => {}
        }
        match self.receiver.recv() {
            Ok(option_data) => {
                match option_data {
                    Some(data) => {
                        match decode(&data[..]) {
                            Ok(object) => {
                                Some(object)
                            },
                            Err(_) => None
                        }
                    },
                    None => None
                }
            },
            _ => None
        }
    }

    /// Clears the memory.
    pub fn clear(&mut self) -> bool {
        match self.sender.send(MemoryProtocol::Clear) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

/// A Memory type to read and write data.
#[derive(Clone, Debug)]
pub struct Memory {
    memory: HashMap<String, Vec<u8>>
}

impl Memory {
    /// Creates empty Memory.
    pub fn new() -> Memory {
        Memory {
            memory: HashMap::new()
        }
    }

    // pub fn get_public_memory(&self) -> PublicMemory {
    //     let (sender, thread_receiver) = mpsc::channel::<MemoryProtocol>();
    //     let (thread_sender, receiver) = mpsc::channel::<Option<Vec<u8>>>();
    //
    //     thread::spawn(move || {
    //         loop {
    //             match thread_receiver.recv() {
    //                 Ok(data) => {
    //                     match data {
    //                         MemoryProtocol::Clear => self.clear(),
    //                         MemoryProtocol::Write(name, data) => self.write_raw(name, data)
    //                     }
    //                 },
    //                 _ => break
    //             };
    //         }
    //     });
    //
    //     PublicMemory {
    //         sender: sender,
    //         receiver: receiver
    //     }
    // }

    /// Writes `Vec<u8>` to the memory.
    pub fn write_raw(&mut self, name: String, data: Vec<u8>) {
        self.memory.insert(name, data);
    }

    /// Reads `Vec<u8>` from the memory.
    pub fn read_raw(&self, name: String) -> Option<&Vec<u8>> {
        self.memory.get(&name)
    }

    /// Removes and returns `Vec<u8>` from the memory.
    pub fn delete_raw(&mut self, name: String) -> Option<Vec<u8>> {
        self.memory.remove(&name)
    }

    /// Clears the memory.
    pub fn clear(&mut self) {
        self.memory.clear()
    }
}
