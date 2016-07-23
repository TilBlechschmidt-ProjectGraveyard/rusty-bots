use std::collections::HashMap;

/// Default type to save coordinates.
pub type Coordinate = i32;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

/// Location in a `Map`.
pub struct Position {
    /// x component
    pub x: Coordinate,
    /// y component
    pub y: Coordinate
}

impl Position {
    /// Creates a new instance `Position`.
    pub fn new(x: Coordinate, y: Coordinate) -> Position {
        Position {
            x: x,
            y: y
        }
    }

    //pub fn add()
}

pub enum TileType {
    Plain,
    Water,
    Rock
}

pub struct Tile {
    pub tileType: TileType
}

pub struct Map {

}

pub struct MapPart {
    pub tiles: HashMap<Position, Tile>
}
