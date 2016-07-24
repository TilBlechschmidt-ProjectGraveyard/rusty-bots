use std::collections::HashMap;
use location::{Location, Distance, Coordinate};
use noise::{Brownian2, Seed, perlin2};

const CHUNK_SIZE: Coordinate = 10;

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

fn generate_tile(loc: Location, seed: u32) -> Tile { // TODO implement generator
    let seed = Seed::new(seed);
    let noise = Brownian2::new(perlin2, 4).wavelength(64.0);
    let val = noise.apply(&seed, &[loc.x as f32, loc.y as f32]);

    if val < 0.0 {
        Tile::new(TileType::Water)
    } else if val < 0.5 {
        Tile::new(TileType::Plain)
    } else {
        Tile::new(TileType::Rock)
    }

    // Tile::new(TileType::Rock)
}


/// The map in a game.
#[derive(Clone, Debug)]
pub struct Map {
    seed: u32,
    chunks: HashMap<Location, Chunk>
}

impl Map {
    /// Creates an empty `Map`.
    pub fn new() -> Map {
        Map {
            seed: 3, //TODO random seed
            chunks: HashMap::new()
        }
    }

    /// Creates an empty `Map`.
    pub fn from_seed(seed: u32) -> Map {
        Map {
            seed: seed,
            chunks: HashMap::new()
        }
    }

    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&mut self, loc: Location) -> &Tile {
        // let chunk_loc = loc / CHUNK_SIZE;
        // let optional_chunk = self.chunks.get(&chunk_loc);
        // let chunk = match optional_chunk {
        //     Some(chunk) => chunk,
        //     None => {
        //         self.chunks.insert(loc / CHUNK_SIZE, Chunk::new());
        //         self.chunks.get(&chunk_loc).unwrap()
        //     }
        // };
        //
        // let optional_tile = chunk.tiles.get(&loc);
        // match optional_tile {
        //     Some(tile) => tile,
        //     None => {
        //         chunk.tiles.insert(loc, generate_tile(loc, self.seed));
        //         chunk.tiles.get(&loc).unwrap()
        //     }
        // }
        let seed = self.seed;
        let chunk = self.chunks.entry(loc / CHUNK_SIZE).or_insert_with(|| Chunk::new()); //TODO to slow
        chunk.tiles.entry(loc).or_insert_with(|| generate_tile(loc, seed)) //TODO to slow
    }

    /// Returns a `&mut Tile` at a given `Location`.
    pub fn get_tile_mut(&mut self, loc: Location) -> &mut Tile {
        let seed = self.seed;
        let chunk = self.chunks.entry(loc / CHUNK_SIZE).or_insert_with(|| Chunk::new());
        chunk.tiles.entry(loc).or_insert_with(|| generate_tile(loc, seed))
    }

    /// Returns a section of the map containing all `Tile`s with a maximum distance from a `Location`.
    #[allow(non_snake_case)]
    pub fn get_map_section(&mut self, location: Location, radius: i32) -> MapSection {
        let mut result = HashMap::new();
        // let mut result2 = Vec::new();
        let radius_squared = (radius * radius) as Distance;

        for Δx in -radius..radius+1 {
            for Δy in -radius..radius+1 {
                let loc = location + (Δx, Δy);
                if location.linear_distance_squared_to(&loc) <= radius_squared {
                    let tile = self.get_tile(loc).clone(); //TODO This is slow
                    result.insert(loc, tile);
                    // result2.push(self.get_tile(loc).clone());
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

    #[allow(non_snake_case, missing_docs)]
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
