use std::collections::HashMap;

/// Default type to save coordinates.
pub type Coordinate = i32;

/// Default enum to save directions
#[allow(missing_docs)]
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

/// Default tile types for `Tiles`;
#[allow(missing_docs)]
pub enum TileType {
    Plain,
    Water,
    Rock
}

/// A tile in a map
pub struct Tile {
    /// The type of the tile
    pub tile_type: TileType
}

struct Chunk {
    tiles: HashMap<Position, Tile>
}


/// The map in a game
pub struct Map {
    chunk: HashMap<Position, Tile>


}

impl Map {
    pub fn new() -> Map {
        Map {
            chunk: HashMap::new()
        }
    }

    fn getTile(&self, loc: Position) {
        self.chunk.get(loc / 100).tiles.get(loc)
    }
}

/// A part of a map that is visible
pub struct MapSection {
    /// A `HashMap` for the `Tile`s
    pub tiles: HashMap<Position, Tile>
}
