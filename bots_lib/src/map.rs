use std::collections::HashMap;
use location::Location;

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
    tiles: HashMap<Location, Tile>
}


/// The map in a game
pub struct Map {
    chunk: HashMap<Location, Chunk>


}

impl Map {
    /// Creates an empty `Map`
    pub fn new() -> Map {
        Map {
            chunk: HashMap::new()
        }
    }

    
    pub fn get_tile(&self, loc: Location) -> Option<&Tile> {
        self.chunk.get(&(loc / 100)).and_then(|chunk: _| {chunk.tiles.get(&loc)})
    }

    pub fn get_tile_mut(&mut self, loc: Location) -> Option<&mut Tile> {
        self.chunk.get_mut(&(loc / 100)).and_then(|chunk: _| {chunk.tiles.get_mut(&loc)})
    }

    pub fn get_map_section(&self, location: Location, radius: i32) {
        for Δx in (-radius..radius+1) {
            for Δy in (-radius..radius+1) {
                let loc = location + (Δx, Δy);
            }
        }
    }
}

/// A part of a map that is visible
pub struct MapSection {
    /// A `HashMap` for the `Tile`s
    pub tiles: HashMap<Location, Tile>
}

impl MapSection {
    pub fn get_tile(&self, loc: Location) -> Option<&Tile> {
        self.tiles.get(&loc)
    }
}
