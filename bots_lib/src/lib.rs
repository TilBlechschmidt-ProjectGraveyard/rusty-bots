#![warn(missing_docs)]
//! Lib for the communication between a user plugin and the server

extern crate rand;
extern crate noise;
extern crate bincode;
extern crate rustc_serialize;

/// Module to save locations.
pub mod location;
/// Module to save the map.
pub mod map;
/// Module to save the creep objects.
pub mod creep;
/// Module for memorys used in the game.
pub mod memory;
