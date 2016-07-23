use std::collections::HashMap;
use location::Location;

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
    pub tiles: HashMap<Location, Tile>
}
