use std::collections::HashMap;
use location::{Location, Distance, Coordinate};

const CHUNK_SIZE: Coordinate = 10;
/// Default type of `Map` seeds.
pub type Seed = usize;

/// Default tile types for `Tiles`.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum TileType {
    Plain,
    Water,
    Rock
}

/// A tile in a map.
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

fn generate_tile(loc: Location, _seed: Seed) -> Tile { // TODO implement generator
    if loc.y > 0 {
        Tile::new(TileType::Plain)
    } else {
        Tile::new(TileType::Rock)
    }
}


/// The map in a game.
#[derive(Clone, Debug)]
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
    pub fn get_map_section(&mut self, location: Location, radius: i32) -> MapSection {
        let mut result = HashMap::new();

        for Δx in -radius..radius+1 {
            for Δy in -radius..radius+1 {
                let loc = location + (Δx, Δy);
                if location.linear_distance_to(&loc) <= radius as Distance {
                    result.insert(loc, self.get_tile(loc).clone());
                }
            }
        }
        // println!("{:?}", self.chunks.keys().collect::<Vec<_>>();
        MapSection::new(location, radius, result)
    }
}

/// A part of a map that is visible
#[derive(Clone, Debug)]
pub struct MapSection {
    /// Center of the map section.
    pub center: Location,
    /// Radius of the map section.
    pub radius: i32,
    /// A `HashMap` containing the `Tile`s
    pub tiles: HashMap<Location, Tile>
}

impl MapSection {
    /// Returns a `MapSection`.
    pub fn new(center: Location, radius: i32, tiles: HashMap<Location, Tile>) -> MapSection {
        MapSection {
            center: center,
            radius: radius,
            tiles: tiles
        }
    }

    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&self, loc: Location) -> Option<&Tile> {
        self.tiles.get(&loc)
    }

    #[allow(non_snake_case)]
    pub fn print(&self) {
        for Δy in -self.radius..self.radius+1 {
            let mut line = String::with_capacity(self.radius as usize * 2 + 1);
            for Δx in -self.radius..self.radius+1 {
                let loc = self.center + (Δx, Δy);
                line = line + match self.get_tile(loc) {
                    Some(tile) => {
                        match tile.tile_type {
                            TileType::Plain => "__",
                            TileType::Water => "~~",
                            TileType::Rock => "ΔΔ"
                        }
                    },
                    None => "  "
                }
            }
            println!("{}", line);
        }
    }
}
