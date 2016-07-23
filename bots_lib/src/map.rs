use std::collections::HashMap;
use location::{Location, Distance, Coordinate};

const CHUNK_SIZE: Coordinate = 100;

/// Default tile types for `Tiles`.
#[allow(missing_docs)]
#[derive(Clone)]
pub enum TileType {
    Plain,
    Water,
    Rock
}

/// A tile in a map.
#[derive(Clone)]
pub struct Tile {
    /// The type of the tile.
    pub tile_type: TileType
}

struct Chunk {
    tiles: HashMap<Location, Tile>
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            tiles: HashMap::new()
        }
    }
}


/// The map in a game.
pub struct Map {
    chunk: HashMap<Location, Chunk>


}

impl Map {
    /// Creates an empty `Map`.
    pub fn new() -> Map {
        Map {
            chunk: HashMap::new()
        }
    }

    /// Inserts a tile in the tile structure of a `Map` at a given `Location`.
    pub fn insert_tile(&mut self, loc: Location, tile: Tile) {
        let chunk = self.chunk.entry(loc / CHUNK_SIZE).or_insert(Chunk::new());
        chunk.tiles.insert(loc, tile);
    }

    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&self, loc: Location) -> Option<&Tile> {
        self.chunk.get(&(loc / CHUNK_SIZE)).and_then(|chunk: _| {chunk.tiles.get(&loc)})
    }

    /// Returns a `&mut Tile` at a given `Location`.
    pub fn get_tile_mut(&mut self, loc: Location) -> Option<&mut Tile> {
        self.chunk.get_mut(&(loc / CHUNK_SIZE)).and_then(|chunk: _| {chunk.tiles.get_mut(&loc)})
    }

    /// Returns a section of the map containing all `Tile`s with a maximum distance from a `Location`.
    #[allow(non_snake_case)]
    pub fn get_map_section(&self, location: Location, radius: i32) -> HashMap<Location, Tile> {
        let mut result = HashMap::new();

        for Δx in -radius..radius+1 {
            for Δy in -radius..radius+1 {
                let loc = location + (Δx, Δy);
                if location.linear_distance_to(&loc) <= radius as Distance {
                    match self.get_tile(loc) {
                        Some(tile) => { result.insert(loc, tile.clone()); },
                        None => {}
                    }
                }
            }
        }
        result
    }
}

/// A part of a map that is visible
pub struct MapSection {
    /// A `HashMap` for the `Tile`s
    pub tiles: HashMap<Location, Tile>
}

impl MapSection {
    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&self, loc: Location) -> Option<&Tile> {
        self.tiles.get(&loc)
    }
}
