use std::collections::HashMap;

/// Struct to save a position in the world
pub struct Position {
    /// x element
    x: i32,
    /// y element
    y: i32
}

pub struct Tile {

}

pub struct Map {

}

pub struct MapPart {
    pub tiles: HashMap<Position, Tile>
}
