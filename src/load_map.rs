// Load the map from data/tiles/*.json

use crate::types::{MapPosition, SerializableTile, Square, SquareType, Tile, WallType};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// This is the default deck function, so far.
/// Should return all tiles for a default game, minus the starting tile
/// Because right now we always start with tile_1a hardcoded.
pub fn load_tiles_from_json() -> Vec<Tile> {
    // TODO: shuffle tiles?
    // TODO - finish transcribing all the tiles AND ensure they're oriented correctly
    // Ideally takes a path (like data/tiles/), and returns a hashmap of Tiles
    let mut tile_map: Vec<Tile> = Vec::new();
    tile_map.push(tile_2());
    tile_map.push(tile_3());
    tile_map.push(tile_4());
    tile_map
}

pub fn load_tile_json_from_path(s: String) -> Tile {
    let p = Path::new(&s);
    let file = File::open(p).expect("Path should exist");
    let reader = BufReader::new(file);
    let st: SerializableTile =
        serde_json::from_reader(reader).expect("Path should be valid serde JSON of a Tile");
    let t = Tile::from(st);
    t
}

/// Function for helping when we write new tiles, and want to print them to stdout
/// on a one-off basis to store them in files.
pub fn print_tile_json() -> () {
    println!("\n{}\n", tile_2().pp());
    println!("\n{}\n", json!(SerializableTile::from(tile_2())));
}

pub fn tile_1a() -> Tile {
    // Generate the object for Tile 1a
    let mut my_squares: Vec<Square> = Vec::new();
    let sq00 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::TimerFlip,
    };
    my_squares.push(sq00);

    let sq01 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq01);

    let sq02 = Square {
        north_wall: WallType::OrangeDoor,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq02);

    let sq03 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::PurpleTeleportPad,
    };
    my_squares.push(sq03);

    let sq10 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::PurpleDoor,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq10);

    let sq11 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq11.clone());
    // Square 12 is the same as square 11
    my_squares.push(sq11.clone());

    let sq13 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::YellowTeleportPad,
    };
    my_squares.push(sq13);

    let sq20 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::OrangeTeleportPad,
    };
    my_squares.push(sq20);
    // Square 21 is the same as square 11
    my_squares.push(sq11.clone());

    let sq22 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq22);

    let sq23 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::GreenDoor,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Escalator,
    };
    my_squares.push(sq23);

    let sq30 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::GreenTeleportPad,
    };
    my_squares.push(sq30);

    let sq31 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::YellowDoor,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq31);

    let sq32 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Escalator,
    };
    my_squares.push(sq32);

    let sq33 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Filled,
    };
    my_squares.push(sq33);
    let my_pos = MapPosition { x: 0, y: 0 };
    Tile {
        squares: my_squares,
        position: my_pos,
        name: "1a".to_string(),
        num_rotations: 0,
    }
}

pub fn tile_2() -> Tile {
    // Generate the object for Tile 2
    let mut my_squares: Vec<Square> = Vec::new();

    let blocked_square = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Filled,
    };
    my_squares.push(blocked_square);
    // square 01 is also blocked
    my_squares.push(blocked_square.clone());

    let sq02 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Escalator,
    };
    my_squares.push(sq02);

    let sq03 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::PurpleEscape,
    };
    my_squares.push(sq03);

    let sq10 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::OrangeDoor,
        square_type: SquareType::Escalator,
    };
    my_squares.push(sq10);

    // square 11 is also blocked
    my_squares.push(blocked_square.clone());
    // square 12 is also blocked
    my_squares.push(blocked_square.clone());
    // square 13 is also blocked
    my_squares.push(blocked_square.clone());

    let sq20 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq20);

    let sq21 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq21);

    // square 22 is also blocked
    my_squares.push(blocked_square.clone());
    // square 23 is also blocked
    my_squares.push(blocked_square.clone());

    let sq30 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::GreenTeleportPad,
    };
    my_squares.push(sq30);
    let sq31 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq31);

    let sq32 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::PurpleTeleportPad,
    };
    my_squares.push(sq32);

    // square 33 is also blocked
    my_squares.push(blocked_square.clone());

    let my_pos = MapPosition { x: 0, y: 0 };
    Tile {
        squares: my_squares,
        position: my_pos,
        name: "2".to_string(),
        num_rotations: 0,
    }
}

pub fn tile_3() -> Tile {
    // Generate the object for Tile 3
    let mut my_squares: Vec<Square> = Vec::new();

    let blocked_square = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Filled,
    };

    let sq00 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq00);

    let sq01 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq01);

    let sq02 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::OrangeTeleportPad,
    };
    my_squares.push(sq02);

    // sq03 is blocked!
    my_squares.push(blocked_square);

    let sq10 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::PurpleDoor,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq10);

    let sq11 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq11);

    let sq12 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq12);

    let sq13 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::GreenTeleportPad,
    };
    my_squares.push(sq13);

    let sq20 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::TimerFlip,
    };
    my_squares.push(sq20);

    let sq21 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq21);

    let sq22 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq22);

    let sq23 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::YellowDoor,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq23);

    // square 30 is blocked
    my_squares.push(blocked_square);

    let sq31 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq31);

    let sq32 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq32);

    // 33 is also filled
    my_squares.push(blocked_square);

    let my_pos = MapPosition { x: 0, y: 0 };
    Tile {
        squares: my_squares,
        position: my_pos,
        name: "3".to_string(),
        num_rotations: 0,
    }
}

pub fn tile_4() -> Tile {
    // Generate the object for Tile 3
    let mut my_squares: Vec<Square> = Vec::new();

    let blocked_square = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::Filled,
    };

    // first two squares (00, 01) are both blocked
    my_squares.push(blocked_square);
    my_squares.push(blocked_square);

    let sq02 = Square {
        north_wall: WallType::PurpleDoor,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq02);

    let sq03 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::YellowTeleportPad,
    };
    my_squares.push(sq03);

    // suqare 10 is blokced
    my_squares.push(blocked_square);

    let sq11 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::TimerFlip,
    };
    my_squares.push(sq11);

    let sq12 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq12);

    // suqare 13 is blokced
    my_squares.push(blocked_square);

    let sq20 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Impassable,
        square_type: SquareType::OrangeTeleportPad,
    };
    my_squares.push(sq20);

    let sq21 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Clear,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq21);

    let sq22 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Clear,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq22);

    let sq23 = Square {
        north_wall: WallType::Impassable,
        east_wall: WallType::GreenDoor,
        south_wall: WallType::Impassable,
        west_wall: WallType::Clear,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq23);

    // square 30 is blocked
    my_squares.push(blocked_square);

    let sq31 = Square {
        north_wall: WallType::Clear,
        east_wall: WallType::Impassable,
        south_wall: WallType::Clear,
        west_wall: WallType::Impassable,
        square_type: SquareType::Normal,
    };
    my_squares.push(sq31);

    // 3233 are also filled
    my_squares.push(blocked_square);
    my_squares.push(blocked_square);

    let my_pos = MapPosition { x: 0, y: 0 };
    Tile {
        squares: my_squares,
        position: my_pos,
        name: "4".to_string(),
        num_rotations: 0,
    }
}

#[allow(dead_code, unused_imports)]
mod tests {
    #[test]
    pub fn test_1a_and_json_match() {
        let code1a = super::tile_1a();
        let json1a = super::load_tile_json_from_path("data/tiles/1a.json".to_string());
        assert_eq!(code1a, json1a)
    }
}
