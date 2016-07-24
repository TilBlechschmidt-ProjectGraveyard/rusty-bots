use rand::{Rng, thread_rng};
use std::ops::{Add, Sub};

use std::collections::HashMap;
use location::{Location, Distance, Coordinate};
use noise::{Brownian2, Seed, perlin2};
use creep::ServerCreep;


const CHUNK_SIZE: Coordinate = 10;

/// Default type for object ids
pub type ID = u64;

/// Default tile types for `Tiles`.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Plain,
    Water,
    Rock
}

/// Default delta tile types.
#[derive(Debug, Clone)]
pub enum TileDelta {
    /// A new tile was added.
    New(TileType),
    /// An existing tile was changed.
    Changed(TileType),
    /// A tile was removed.
    Removed(TileType)
}


#[derive(Clone, Debug)]
struct Chunk {
    tiles: HashMap<Location, TileType>,
    creeps: Vec<ID>
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            tiles: HashMap::new(),
            creeps: Vec::new()
        }
    }
}

fn generate_tile(loc: Location, seed: u32) -> TileType {
    let seed = Seed::new(seed);
    let noise = Brownian2::new(perlin2, 4).wavelength(64.0);
    let val = noise.apply(&seed, &[loc.x as f32, loc.y as f32]);

    if val < 0.0 {
        TileType::Water
    } else if val < 0.4 {
        TileType::Plain
    } else {
        TileType::Rock
    }
}


/// The map in a game.
#[derive(Clone, Debug)]
pub struct Map {
    seed: u32,
    chunks: HashMap<Location, Chunk>,
    creeps: HashMap<ID, ServerCreep>,
    last_id: ID
}

impl Map {
    /// Creates an empty `Map`.
    pub fn new() -> Map {
        let seed = thread_rng().next_u32();
        println!("Generated map w/ seed {}", seed);
        Map::from_seed(seed)
    }

    /// Creates an empty `Map`.
    pub fn from_seed(seed: u32) -> Map {
        Map {
            seed: seed,
            chunks: HashMap::new(),
            creeps: HashMap::new(),
            last_id: 0
        }
    }

    /// Moves a creep to a given location
    pub fn move_creep(&mut self, id: ID, loc: Location) -> bool {
        let old_loc = {
            match self.creeps.get_mut(&id) {
                Some(creep) => {
                    let old_loc = creep.location;
                    creep.location = loc;
                    old_loc
                },
                None => return false
            }
        };
        {
            let ref mut creeps = self.get_chunk(old_loc).creeps;
            match creeps.iter().position(|&r| r == id) {
                Some(index) => {creeps.swap_remove(index); },
                None => return false
            }
        }


        self.get_chunk(loc).creeps.push(id);

        true
    }

    /// Returns the next free id
    pub fn get_id(&mut self) -> ID {
        self.last_id += 1;
        return self.last_id
    }

    fn get_chunk(&mut self, loc: Location) -> &mut Chunk {
        self.chunks.entry(loc / CHUNK_SIZE).or_insert_with(|| Chunk::new())
    }

    /// Returns a `&mut Tile` at a given `Location`.
    pub fn get_tile(&mut self, loc: Location) -> &mut TileType {
        let seed = self.seed;
        let chunk = self.get_chunk(loc);
        chunk.tiles.entry(loc).or_insert_with(|| generate_tile(loc, seed))
    }

    /// Returns a section of the map containing all `Tile`s with a maximum distance from a `Location`.
    #[allow(non_snake_case)]
    pub fn get_map_section(&mut self, location: Location, radius: i32) -> MapSection {
        let mut result = HashMap::new();
        // let mut result2 = Vec::new();
        let radius_squared = (radius * radius) as Distance;

        for delta_x in -radius..radius+1 {
            for delta_y in -radius..radius+1 {
                let loc = location + (delta_x, delta_y);
                if location.linear_distance_squared_to(&loc) <= radius_squared {
                    let tile = self.get_tile(loc).clone(); //TODO This is slow
                    result.insert(loc, tile);
                    // result2.push(self.get_tile(loc).clone());
                }
            }
        }
        // println!("{:?}", self.chunks.keys().collect::<Vec<_>>();
        MapSection::new(result)
    }
}

/// A part of a map that is visible
#[derive(Clone, Debug)]
pub struct MapSection {
    /// A `HashMap` containing the `Tile`s
    pub tiles: HashMap<Location, TileType>
}

impl MapSection {
    /// Returns a `MapSection`.
    pub fn new(tiles: HashMap<Location, TileType>) -> MapSection {
        MapSection {
            tiles: tiles
        }
    }

    /// Returns a `&Tile` at a given `Location`.
    pub fn get_tile(&self, loc: Location) -> Option<&TileType> {
        self.tiles.get(&loc)
    }

    #[allow(non_snake_case, missing_docs)]
    pub fn print(&self, center: Location, radius: i32) {
        for delta_y in -radius..radius+1 {
            let mut line = String::with_capacity(radius as usize * 2 + 1);
            for delta_x in -radius..radius+1 {
                let loc = center + (delta_x, delta_y);
                line = line + match self.get_tile(loc) {
                    Some(tile) => {
                        match tile {
                            &TileType::Plain => "__",
                            &TileType::Water => "~~",
                            &TileType::Rock => "##"
                        }
                    },
                    None => "  "
                }
            }
            println!("{}", line);
        }
    }
}

impl Add for MapSection {
    type Output = MapSection;

    fn add(mut self, _rhs: MapSection) -> MapSection {
        for (&other_loc, other_tile) in _rhs.tiles.iter() {
            self.tiles.entry(other_loc).or_insert(other_tile.clone());
        }
        self
    }
}

impl Sub for MapSection {
    type Output = DeltaMap;

    fn sub(self, _rhs: MapSection) -> DeltaMap {
        let mut tiles: HashMap<Location, TileDelta> = self.tiles.iter().map(|(&loc, tile)| (loc, TileDelta::Removed(tile.clone()))).collect::<HashMap<_, _>>();

        for (&loc, tile) in _rhs.tiles.iter() {
            match self.tiles.get(&loc) {
                Some(old_tile) => {
                    if old_tile != tile {
                        tiles.insert(loc, TileDelta::Changed(tile.clone()));
                    } else {
                        tiles.remove(&loc);
                    }
                },
                None => {
                    tiles.insert(loc, TileDelta::New(tile.clone()));
                }
            }
        }
        DeltaMap::new(tiles)
    }
}

/// The delata between to `MapSection`s.
#[derive(Debug, Clone)]
pub struct DeltaMap {
    /// A `HashMap` containing the `Tile`s
    pub tiles: HashMap<Location, TileDelta>
}

impl DeltaMap {
    /// Returns a `DeltaMap`.
    pub fn new(tiles: HashMap<Location, TileDelta>) -> DeltaMap {
        DeltaMap {
            tiles: tiles
        }
    }
}
