// Load data from the data/ directory into the game

use crate::types::{MapPosition, SerializableTile, Square, SquareType, Tile, WallType};
use log::info;
use std::collections::HashMap;

pub fn load_serializable_tiles_from_json() -> HashMap<String, SerializableTile> {
    // TODO
    // Ideally takes a path and then loads it as a hashmap of SerializableTiles
    // Intended as a helper for load_tiles_from_json
    let tile_map = HashMap::<String, SerializableTile>::new();
    tile_map
}

pub fn load_tiles_from_json() -> HashMap<String, Tile> {
    // TODO
    // Ideally takes a path, then returns a hashmap of Tiles
    let tile_map = HashMap::<String, Tile>::new();
    tile_map
}

#[test]
pub fn serialize_file_test() -> () {
    // for discovering how to write these
    let mut my_squares: Vec<Square> = Vec::new();
    for _ in 0..16 {
        let sq = Square {
            north_wall: WallType::Clear,
            east_wall: WallType::Clear,
            south_wall: WallType::Clear,
            west_wall: WallType::Clear,
            square_type: SquareType::Normal,
        };
        my_squares.push(sq);
    }
    let my_pos = MapPosition { x: 0, y: 0 };
    let tile = Tile {
        squares: my_squares,
        position: my_pos,
    };

    let st: SerializableTile = SerializableTile::from(tile);
    let serialized = serde_json::to_string(&st).unwrap();
    info!("{}", serialized);
}
