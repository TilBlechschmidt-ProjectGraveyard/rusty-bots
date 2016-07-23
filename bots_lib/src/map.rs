use std::collections::HashMap;
use location::{Location, Distance, Coordinate};

const CHUNK_SIZE: Coordinate = 100;
/// Default type of `Map` seeds.
pub type Seed = usize;

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

impl Tile {
    /// Returns a `Tile` with a given `TileType`.
    pub fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type: tile_type
        }
    }
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

fn generate_tile(_loc: Location, _seed: Seed) -> Tile {
    Tile::new(TileType::Plain) // TODO implement generator
}


/// The map in a game.
pub struct Map {
    seed: Seed,
    chunks: HashMap<Location, Chunk>
}

impl Map {
    /// Creates an empty `Map`.
    pub fn new() -> Map {
        Map {
            seed: 0, //TODO random seed
            chunks: HashMap::new()
        }
    }

    /// Creates an empty `Map`.
    pub fn from_seed(seed: Seed) -> Map {
        Map {
            seed: seed,
            chunks: HashMap::new()
        }
    }

    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&mut self, loc: Location) -> &Tile {
        let chunk = self.chunks.entry(loc / CHUNK_SIZE).or_insert(Chunk::new());
        chunk.tiles.entry(loc).or_insert(generate_tile(loc, self.seed))
    }

    /// Returns a `&mut Tile` at a given `Location`.
    pub fn get_tile_mut(&mut self, loc: Location) -> &mut Tile {
        let chunk = self.chunks.entry(loc / CHUNK_SIZE).or_insert(Chunk::new());
        chunk.tiles.entry(loc).or_insert(generate_tile(loc, self.seed))
    }

    /// Returns a section of the map containing all `Tile`s with a maximum distance from a `Location`.
    #[allow(non_snake_case)]
    pub fn get_map_section(&mut self, location: Location, radius: i32) -> HashMap<Location, Tile> {
        let mut result = HashMap::new();

        for Δx in -radius..radius+1 {
            for Δy in -radius..radius+1 {
                let loc = location + (Δx, Δy);
                if location.linear_distance_to(&loc) <= radius as Distance {
                    result.insert(loc, self.get_tile(loc).clone());
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
