use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

// Import all the proto types in this private module.
pub mod proto_types {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

// Re-export the enums.
pub use proto_types::Ability;
pub use proto_types::GameStatus;
pub use proto_types::HeisterColor;
pub use proto_types::HeisterSymbol;
pub use proto_types::PossibleTeleportEntry;
pub use proto_types::SquareType;
pub use proto_types::WallType;

// Re-export the MainMessage.
pub use proto_types::MainMessage;
pub mod main_message {
    pub use super::proto_types::main_message::Body;
}

pub trait Internal {
    type P;

    // TODO Use From and Into here instead.
    fn from_proto(proto: Self::P) -> Self;
    fn to_proto(&self) -> Self::P;
}

pub const TIMER_DURATION_SECS: u64 = 5 * 60;

pub const DOOR_TYPES: [&'static WallType; 4] = [
    &WallType::PurpleDoor,
    &WallType::OrangeDoor,
    &WallType::GreenDoor,
    &WallType::YellowDoor,
];

pub const TELEPORTER_TYPES: [&'static SquareType; 4] = [
    &SquareType::PurpleTeleportPad,
    &SquareType::OrangeTeleportPad,
    &SquareType::GreenTeleportPad,
    &SquareType::YellowTeleportPad,
];

pub const HEISTER_COLORS: [&'static HeisterColor; 4] = [
    &HeisterColor::Purple,
    &HeisterColor::Orange,
    &HeisterColor::Green,
    &HeisterColor::Yellow,
];

pub const DIRECTIONS: [&'static MoveDirection; 4] = [
    &MoveDirection::North,
    &MoveDirection::East,
    &MoveDirection::South,
    &MoveDirection::West,
];

pub static ESCAPED: &'static MapPosition = &MapPosition { x: 500, y: 500 };

pub fn get_wall_color(wall: proto_types::WallType) -> Option<HeisterColor> {
    match wall {
        WallType::PurpleDoor => Some(HeisterColor::Purple),
        WallType::OrangeDoor => Some(HeisterColor::Orange),
        WallType::GreenDoor => Some(HeisterColor::Green),
        WallType::YellowDoor => Some(HeisterColor::Yellow),
        _wildcard => None,
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TilePosition {
    x: u32,
    y: u32,
}

impl Internal for TilePosition {
    type P = proto_types::TilePosition;

    fn from_proto(proto: proto_types::TilePosition) -> Self {
        TilePosition {
            x: proto.x,
            y: proto.y,
        }
    }

    fn to_proto(&self) -> proto_types::TilePosition {
        proto_types::TilePosition {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
}

impl Internal for MapPosition {
    type P = proto_types::MapPosition;

    fn from_proto(proto: proto_types::MapPosition) -> Self {
        MapPosition {
            x: proto.x,
            y: proto.y,
        }
    }

    fn to_proto(&self) -> proto_types::MapPosition {
        proto_types::MapPosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl MapPosition {
    pub fn is_adjacent(&self, pos: &MapPosition) -> bool {
        if self.x == pos.x {
            let abs_distance = (self.y - pos.y).abs();
            return abs_distance == 1;
        } else if self.y == pos.y {
            let abs_distance = (self.x - pos.x).abs();
            return abs_distance == 1;
        } else {
            return false;
        }
    }

    /// Returns None in two cases:
    /// 1. move is not in a single cardinal direction (ie. nw, sse)
    /// 2. pos == self
    pub fn get_move_direction(&self, pos: &MapPosition) -> Option<MoveDirection> {
        if self.x == pos.x {
            // Then we suppose it's a move in the y direction
            if self.y == pos.y {
                return None; // (2) pos == self
            } else if self.y > pos.y {
                return Some(MoveDirection::North);
            } else {
                return Some(MoveDirection::South);
            }
        } else {
            if self.y != pos.y {
                return None; // (1) non-cardinal direction
            } else if self.x > pos.x {
                return Some(MoveDirection::West);
            } else {
                return Some(MoveDirection::East);
            }
        }
    }

    /// Given a position and direction, return the position if you were to
    /// "Move" in that direction (one square)
    pub fn move_in_direction(&self, direction: &MoveDirection) -> MapPosition {
        match direction {
            MoveDirection::North => MapPosition {
                x: self.x,
                y: self.y - 1,
            },
            MoveDirection::East => MapPosition {
                x: self.x + 1,
                y: self.y,
            },
            MoveDirection::South => MapPosition {
                x: self.x,
                y: self.y + 1,
            },
            MoveDirection::West => MapPosition {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    /// With respect to a given (presumed) Tile MapPosition, return the respective
    /// MapPosition of a TileEntrance from one of this tile's doors (in a given direction)
    fn entrance_position(&self, dir: &MoveDirection) -> MapPosition {
        match dir {
            MoveDirection::North => MapPosition {
                x: self.x + 2,
                y: self.y - 1,
            },
            MoveDirection::East => MapPosition {
                x: self.x + 4,
                y: self.y + 2,
            },
            MoveDirection::South => MapPosition {
                x: self.x + 1,
                y: self.y + 4,
            },
            MoveDirection::West => MapPosition {
                x: self.x - 1,
                y: self.y + 1,
            },
        }
    }

    /// From a tile exit square (one from which a player might initiate a PlaceTile move),
    /// figure out the MapPosition of the tile that the heister is on.
    /// (Useful for looking up which tile a heister might currently be on)
    /// * You might notice - this is the same as new_tile_position, but with opposite
    /// directions swapped. That's true! That's the magic of the game.
    pub fn current_tile_position(&self, dir: &MoveDirection) -> MapPosition {
        match dir {
            MoveDirection::North => MapPosition {
                x: self.x - 2,
                y: self.y,
            },
            MoveDirection::West => MapPosition {
                x: self.x,
                y: self.y - 1,
            },
            MoveDirection::South => MapPosition {
                x: self.x - 1,
                y: self.y - 3,
            },
            MoveDirection::East => MapPosition {
                x: self.x - 3,
                y: self.y - 2,
            },
        }
    }

    /// From a tile entrance and move direction of the tile's orientation,
    /// return the MapPosition for that new tile to place it in the absolute grid
    /// This is doable since every tile has an entry square in some rotation of
    /// (1, 3) - except for starting tiles
    pub fn new_tile_position(&self, dir: &MoveDirection) -> MapPosition {
        match dir {
            MoveDirection::North => MapPosition {
                x: self.x - 1,
                y: self.y - 3,
            },
            MoveDirection::East => MapPosition {
                x: self.x,
                y: self.y - 1,
            },
            MoveDirection::South => MapPosition {
                x: self.x - 2,
                y: self.y,
            },
            MoveDirection::West => MapPosition {
                x: self.x - 3,
                y: self.y - 2,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MoveDirection {
    North,
    East,
    South,
    West,
}

impl MoveDirection {
    pub fn opposite(&self) -> MoveDirection {
        match self {
            MoveDirection::North => MoveDirection::South,
            MoveDirection::East => MoveDirection::West,
            MoveDirection::South => MoveDirection::North,
            MoveDirection::West => MoveDirection::East,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    pub squares: Vec<Square>,
    pub position: MapPosition,
    pub name: String,
    pub num_rotations: u32,
}

impl Internal for Tile {
    type P = proto_types::Tile;

    fn from_proto(proto: proto_types::Tile) -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for proto_square in proto.squares {
            let square = Square::from_proto(proto_square);
            squares.push(square);
        }
        Tile {
            squares,
            position: MapPosition::from_proto(proto.position.unwrap()),
            name: proto.name.to_string(),
            num_rotations: proto.num_rotations,
        }
    }

    fn to_proto(&self) -> proto_types::Tile {
        let mut proto_squares: Vec<proto_types::Square> = Vec::new();
        for square in &self.squares {
            let proto_square = square.to_proto();
            proto_squares.push(proto_square);
        }
        proto_types::Tile {
            squares: proto_squares,
            position: Some(self.position.to_proto()),
            name: self.name.to_string(),
            num_rotations: self.num_rotations,
        }
    }
}

impl From<SerializableTile> for Tile {
    fn from(item: SerializableTile) -> Self {
        let mut squares = vec![];
        for serializable_square in item.squares {
            squares.push(Square::from(serializable_square));
        }
        Tile {
            squares,
            position: item.position,
            name: item.name,
            num_rotations: item.num_rotations,
        }
    }
}

impl Tile {
    fn door_square_indices() -> HashMap<MoveDirection, usize> {
        let dirs_to_square_indices: HashMap<MoveDirection, usize> = [
            (MoveDirection::North, 2),
            (MoveDirection::East, 11),
            (MoveDirection::South, 13),
            (MoveDirection::West, 4),
        ]
        .iter()
        .cloned()
        .collect();
        dirs_to_square_indices
    }

    fn square_idx_to_map_pos(&self, i: usize) -> MapPosition {
        let sq_x = (i % 4) as i32;
        let sq_y = (i / 4) as i32;
        let grid_x = self.position.x + sq_x;
        let grid_y = self.position.y + sq_y;
        MapPosition {
            x: grid_x,
            y: grid_y,
        }
    }

    pub fn pp(&self) -> String {
        let mut square_strs: Vec<String> = Vec::new();
        // should have 12 entries, for 3 rows of each square (4 squares in a tile)
        for i in (0..15).step_by(4) {
            let mut top_row = String::new();
            let mut middle_row = String::new();
            let mut bottom_row = String::new();
            for j in 0..4 {
                top_row += &self.squares.get(i + j).unwrap().pp_top_row();
                middle_row += &self.squares.get(i + j).unwrap().pp_middle_row();
                bottom_row += &self.squares.get(i + j).unwrap().pp_bottom_row();
            }
            square_strs.push(top_row);
            square_strs.push(middle_row);
            square_strs.push(bottom_row);
        }
        let mut pp = String::new();
        for line in square_strs {
            pp += format!("{}\n", line).as_str();
        }
        pp
    }

    pub fn to_matrix(&self) -> Vec<Vec<Square>> {
        let temp_sq = Square::default();
        let mut m: Vec<Vec<Square>> = vec![vec![temp_sq; 4]; 4];
        let mut i = 0;
        for row in 0..4 {
            for col in 0..4 {
                let square = self.squares.get(i).unwrap();
                m[row][col] = *square;
                i += 1;
            }
        }
        m
    }

    pub fn from_matrix(
        m: Vec<Vec<Square>>,
        name: String,
        position: MapPosition,
        num_rotations: u32,
    ) -> Tile {
        let mut squares: Vec<Square> = Vec::new();
        for row in 0..4 {
            for col in 0..4 {
                let square = m.get(row).unwrap().get(col).unwrap();
                squares.push(*square);
            }
        }
        Tile {
            position,
            squares,
            name,
            num_rotations,
        }
    }

    pub fn rotate_matrix_clockwise(m: &Vec<Vec<Square>>) -> Vec<Vec<Square>> {
        let n = 4;
        let x = 2;
        let y = n - 1;
        let temp_sq = Square::default();
        let mut rotated: Vec<Vec<Square>> = vec![vec![temp_sq; n]; n];

        for a in 0..(x + 1) {
            for b in a..(y - a + 1) {
                let k = m[a][b];
                rotated[a][b] = m[y - b][a].rotate_clockwise();
                rotated[y - b][a] = m[y - a][y - b].rotate_clockwise();
                rotated[y - a][y - b] = m[b][y - a].rotate_clockwise();
                rotated[b][y - a] = k.rotate_clockwise();
            }
        }
        rotated
    }

    /// In any tile, we can _KNOW_ the index of "door" squares.
    /// 0    1   *2    3
    /// 4*   5    6    7
    /// 8    9   10  *11
    /// 12  13*  14   15
    pub fn get_entrance_squares(&self) -> HashMap<MoveDirection, Square> {
        let mut map: HashMap<MoveDirection, Square> = HashMap::new();
        let dirs_to_square_indices = Tile::door_square_indices();
        for (dir, square_index) in dirs_to_square_indices {
            let square = *self.squares.get(square_index).unwrap();
            map.insert(dir, square);
        }
        map
    }

    pub fn has_door_in_dir(&self, dir: MoveDirection) -> bool {
        let square_index = *Tile::door_square_indices().get(&dir).unwrap();
        let square = self.squares.get(square_index).unwrap();
        square.has_door()
    }

    pub fn open_door_in_dir(&mut self, dir: MoveDirection) -> () {
        let square_index = *Tile::door_square_indices().get(&dir).unwrap();
        let square = self.squares.get_mut(square_index).unwrap();
        square.open_door(dir)
    }

    pub fn close_door_in_dir(&mut self, dir: MoveDirection) -> () {
        let square_index = *Tile::door_square_indices().get(&dir).unwrap();
        let square = self.squares.get_mut(square_index).unwrap();
        square.close_door(dir)
    }

    /// This returns possible entrance positions (not contained by this tile)
    pub fn adjacent_entrances(&self) -> HashMap<MoveDirection, MapPosition> {
        let mut map: HashMap<MoveDirection, MapPosition> = HashMap::new();
        for dir in self.get_entrance_squares().keys() {
            let pos = self.position.entrance_position(dir);
            map.insert(*dir, pos);
        }
        map
    }

    pub fn get_escalator_dest(&self, pos: &MapPosition) -> Option<MapPosition> {
        let escalator_idx_and_squares: Vec<(usize, &Square)> = self
            .squares
            .iter()
            .enumerate()
            .filter(|(_, sq)| sq.square_type == SquareType::Escalator)
            .collect();
        for (i, _sq) in escalator_idx_and_squares {
            let sq_pos = self.square_idx_to_map_pos(i);
            if sq_pos != *pos {
                return Some(sq_pos);
            }
        }
        None
    }

    /// Returns a Map from color to MapPositions for all teleporters within the tile
    /// NOTE: Assumption: Each tile has no more than 1 teleporter per color
    pub fn get_teleporters(&self) -> HashMap<HeisterColor, MapPosition> {
        let mut map: HashMap<HeisterColor, MapPosition> = HashMap::new();
        for (idx, square) in self.squares.iter().enumerate() {
            if !TELEPORTER_TYPES.contains(&&square.square_type) {
                continue;
            }
            // Match teleporter type to color,... helper???
            match square.color() {
                Some(color) => {
                    map.insert(color, self.square_idx_to_map_pos(idx));
                }
                None => {}
            }
        }
        // fn square_idx_to_map_pos(&self, i: usize) -> MapPosition {
        map
    }

    pub fn flip_timer(&mut self) -> () {
        for (idx, square) in self.squares.iter().enumerate() {
            if square.square_type == SquareType::TimerFlip {
                let mut flipped_square = square.clone();
                flipped_square.square_type = SquareType::TimerFlipUsed;
                self.squares[idx] = flipped_square;
                return; // return early, save the cycles.
            }
        }
    }
}

pub enum StartingTile {
    A(Tile),
    B(Tile),
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Square {
    pub north_wall: WallType,
    pub east_wall: WallType,
    pub south_wall: WallType,
    pub west_wall: WallType,
    pub square_type: SquareType,
}

impl Internal for Square {
    type P = proto_types::Square;

    fn from_proto(proto: proto_types::Square) -> Self {
        Square {
            north_wall: WallType::from_i32(proto.north_wall).unwrap(),
            east_wall: WallType::from_i32(proto.east_wall).unwrap(),
            south_wall: WallType::from_i32(proto.south_wall).unwrap(),
            west_wall: WallType::from_i32(proto.west_wall).unwrap(),
            square_type: SquareType::from_i32(proto.square_type).unwrap(),
        }
    }

    fn to_proto(&self) -> proto_types::Square {
        proto_types::Square {
            north_wall: i32::from(self.north_wall),
            east_wall: i32::from(self.east_wall),
            south_wall: i32::from(self.south_wall),
            west_wall: i32::from(self.west_wall),
            square_type: i32::from(self.square_type),
        }
    }
}

impl From<SerializableSquare> for Square {
    fn from(item: SerializableSquare) -> Self {
        Square {
            north_wall: WallType::from_i32(i32::from(item.north_wall)).unwrap(),
            east_wall: WallType::from_i32(i32::from(item.east_wall)).unwrap(),
            south_wall: WallType::from_i32(i32::from(item.south_wall)).unwrap(),
            west_wall: WallType::from_i32(i32::from(item.west_wall)).unwrap(),
            square_type: SquareType::from_i32(i32::from(item.square_type)).unwrap(),
        }
    }
}

impl Square {
    pub fn color(&self) -> Option<HeisterColor> {
        match self.square_type {
            SquareType::PurpleTeleportPad => Some(HeisterColor::Purple),
            SquareType::OrangeTeleportPad => Some(HeisterColor::Orange),
            SquareType::GreenTeleportPad => Some(HeisterColor::Green),
            SquareType::YellowTeleportPad => Some(HeisterColor::Yellow),
            SquareType::PurpleItem => Some(HeisterColor::Purple),
            SquareType::OrangeItem => Some(HeisterColor::Orange),
            SquareType::GreenItem => Some(HeisterColor::Green),
            SquareType::YellowItem => Some(HeisterColor::Yellow),
            SquareType::PurpleEscape => Some(HeisterColor::Purple),
            SquareType::OrangeEscape => Some(HeisterColor::Orange),
            SquareType::GreenEscape => Some(HeisterColor::Green),
            SquareType::YellowEscape => Some(HeisterColor::Yellow),
            _wildcard => None,
        }
    }

    pub fn is_item(&self) -> bool {
        match self.square_type {
            SquareType::PurpleItem
            | SquareType::YellowItem
            | SquareType::OrangeItem
            | SquareType::GreenItem => true,
            _wildcard => false,
        }
    }

    pub fn is_teleport(&self) -> bool {
        match self.square_type {
            SquareType::PurpleTeleportPad
            | SquareType::YellowTeleportPad
            | SquareType::OrangeTeleportPad
            | SquareType::GreenTeleportPad => true,
            _wildcard => false,
        }
    }

    pub fn is_escape(&self) -> bool {
        match self.square_type {
            SquareType::PurpleEscape
            | SquareType::YellowEscape
            | SquareType::OrangeEscape
            | SquareType::GreenEscape => true,
            _wildcard => false,
        }
    }

    pub fn teleport_matches_heister(&self, color: HeisterColor) -> bool {
        let square_type = self.square_type;
        match color {
            HeisterColor::Purple => square_type == SquareType::PurpleTeleportPad,
            HeisterColor::Yellow => square_type == SquareType::YellowTeleportPad,
            HeisterColor::Orange => square_type == SquareType::OrangeTeleportPad,
            HeisterColor::Green => square_type == SquareType::GreenTeleportPad,
        }
    }

    pub fn rotate_clockwise(&self) -> Square {
        let rotated_clockwise_90degrees = Square {
            north_wall: self.west_wall,
            east_wall: self.north_wall,
            south_wall: self.east_wall,
            west_wall: self.south_wall,
            square_type: self.square_type,
        };
        rotated_clockwise_90degrees
    }

    pub fn get_walls(&self) -> HashMap<MoveDirection, WallType> {
        let mut walls: HashMap<MoveDirection, WallType> = HashMap::new();
        walls.insert(MoveDirection::North, self.north_wall);
        walls.insert(MoveDirection::East, self.east_wall);
        walls.insert(MoveDirection::South, self.south_wall);
        walls.insert(MoveDirection::West, self.west_wall);
        walls
    }

    pub fn has_door(&self) -> bool {
        DOOR_TYPES.contains(&&self.north_wall)
            | DOOR_TYPES.contains(&&self.east_wall)
            | DOOR_TYPES.contains(&&self.south_wall)
            | DOOR_TYPES.contains(&&self.west_wall)
    }

    /// Return the square's (exit) door, if it has one
    pub fn get_door_wall(&self) -> Option<WallType> {
        let walls = self.get_walls();
        let door = walls
            .values()
            .find(|&wt| DOOR_TYPES.iter().any(|&dt| wt == dt));
        match door {
            Some(d) => {
                let ret = d.clone();
                Some(ret.clone())
            }
            None => None,
        }
    }

    /// Return the direction of the square's (exit) door, if it has one
    pub fn get_door_direction(self) -> Option<MoveDirection> {
        match self.get_door_wall() {
            Some(_) => (),
            None => return None,
        };
        for (dir, wall) in self.get_walls().iter() {
            if DOOR_TYPES.contains(&wall) {
                return Some(dir.clone());
            }
        }
        None
    }

    /// Return whether or not my square_type matches the given color
    /// (or false, if not a teleport)
    pub fn teleport_matches_color(&self, color: HeisterColor) -> bool {
        match self.square_type {
            SquareType::PurpleTeleportPad => color == HeisterColor::Purple,
            SquareType::OrangeTeleportPad => color == HeisterColor::Orange,
            SquareType::GreenTeleportPad => color == HeisterColor::Green,
            SquareType::YellowTeleportPad => color == HeisterColor::Yellow,
            _wildcard => false,
        }
    }

    pub fn open_door(&mut self, dir: MoveDirection) -> () {
        match dir {
            MoveDirection::North => {
                if DOOR_TYPES.contains(&&self.north_wall) {
                    self.north_wall = WallType::Clear;
                }
            }
            MoveDirection::East => {
                if DOOR_TYPES.contains(&&self.east_wall) {
                    self.east_wall = WallType::Clear;
                }
            }
            MoveDirection::South => {
                if DOOR_TYPES.contains(&&self.south_wall) {
                    self.south_wall = WallType::Clear;
                }
            }
            MoveDirection::West => {
                if DOOR_TYPES.contains(&&self.west_wall) {
                    self.west_wall = WallType::Clear;
                }
            }
        };
    }

    /// Once you close a door, it CANNOT be re-opened via open_door()
    pub fn close_door(&mut self, dir: MoveDirection) -> () {
        match dir {
            MoveDirection::North => {
                self.north_wall = WallType::Impassable;
            }
            MoveDirection::East => {
                self.east_wall = WallType::Impassable;
            }
            MoveDirection::South => {
                self.south_wall = WallType::Impassable;
            }
            MoveDirection::West => {
                self.west_wall = WallType::Impassable;
            }
        };
    }

    fn pp_wall(w: WallType, vertical: bool) -> String {
        let wallchar = match vertical {
            true => "|",
            false => "_",
        };
        match w {
            WallType::Clear => " ".to_string(),
            WallType::Impassable => wallchar.to_string(),
            WallType::PurpleDoor => "P".to_string(),
            WallType::OrangeDoor => "O".to_string(),
            WallType::YellowDoor => "Y".to_string(),
            WallType::GreenDoor => "G".to_string(),
        }
    }

    fn pp_space(s: SquareType) -> String {
        match s {
            SquareType::TimerFlip => "x".to_string(),
            SquareType::TimerFlipUsed => "*".to_string(),
            SquareType::Escalator => "Z".to_string(),
            SquareType::Filled => "&".to_string(),
            SquareType::YellowTeleportPad => "@".to_string(),
            SquareType::YellowItem => "i".to_string(),
            SquareType::YellowEscape => "e".to_string(),
            SquareType::GreenTeleportPad => "@".to_string(),
            SquareType::GreenItem => "i".to_string(),
            SquareType::GreenEscape => "e".to_string(),
            SquareType::PurpleTeleportPad => "@".to_string(),
            SquareType::PurpleItem => "i".to_string(),
            SquareType::PurpleEscape => "e".to_string(),
            SquareType::OrangeTeleportPad => "@".to_string(),
            SquareType::OrangeItem => "i".to_string(),
            SquareType::OrangeEscape => "e".to_string(),
            _wildcard => " ".to_string(),
        }
    }

    pub fn pp_top_row(&self) -> String {
        format!(".{}.", Self::pp_wall(self.north_wall, false))
    }

    pub fn pp_middle_row(&self) -> String {
        format!(
            "{}{}{}",
            Self::pp_wall(self.west_wall, true),
            Self::pp_space(self.square_type),
            Self::pp_wall(self.east_wall, true),
        )
    }

    pub fn pp_bottom_row(&self) -> String {
        format!(".{}.", Self::pp_wall(self.south_wall, false))
    }

    pub fn pp(&self) -> String {
        format!(
            "{}\n{}\n{}\n",
            self.pp_top_row(),
            self.pp_middle_row(),
            self.pp_bottom_row()
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Heister {
    pub heister_color: HeisterColor,
    pub map_position: MapPosition,
    pub has_taken_item: bool,
    pub has_escaped: bool,
}

impl Internal for Heister {
    type P = proto_types::Heister;

    fn from_proto(proto: proto_types::Heister) -> Self {
        Heister {
            heister_color: HeisterColor::from_i32(proto.heister_color).unwrap(),
            map_position: MapPosition::from_proto(proto.map_position.unwrap()),
            has_taken_item: proto.has_taken_item,
            has_escaped: proto.has_escaped,
        }
    }

    fn to_proto(&self) -> proto_types::Heister {
        proto_types::Heister {
            heister_color: i32::from(self.heister_color),
            map_position: Some(self.map_position.to_proto()),
            has_taken_item: self.has_taken_item,
            has_escaped: self.has_escaped,
        }
    }
}

impl Heister {
    pub fn get_initial(heister_color: HeisterColor, starting_tile: &StartingTile) -> Self {
        let map_position = match starting_tile {
            // | | | | |
            // | |p|o| |
            // | |y|g| |
            // | | | | |
            StartingTile::A(_) => match heister_color {
                HeisterColor::Yellow => MapPosition { x: 1, y: 2 },
                HeisterColor::Purple => MapPosition { x: 1, y: 1 },
                HeisterColor::Green => MapPosition { x: 2, y: 2 },
                HeisterColor::Orange => MapPosition { x: 2, y: 1 },
            },
            _ => MapPosition { x: 0, y: 0 }, // TODO Do this for starting B side.
        };
        Heister {
            heister_color,
            map_position,
            has_taken_item: false,
            has_escaped: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub abilities: Vec<Ability>,
}

impl Internal for Player {
    type P = proto_types::Player;

    fn from_proto(proto: proto_types::Player) -> Self {
        let mut abilities = Vec::<Ability>::new();
        for proto_ability in proto.abilities {
            let ability = Ability::from_i32(proto_ability).unwrap();
            abilities.push(ability);
        }
        Player {
            name: proto.name,
            abilities,
        }
    }

    fn to_proto(&self) -> proto_types::Player {
        let mut proto_abilities: Vec<i32> = Vec::new();
        for ability in &self.abilities {
            let proto_ability = i32::from(ability.clone());
            proto_abilities.push(proto_ability);
        }
        proto_types::Player {
            name: self.name.clone(),
            abilities: proto_abilities,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StartGame {}

impl Internal for StartGame {
    type P = proto_types::StartGame;

    fn from_proto(_proto: proto_types::StartGame) -> Self {
        StartGame {}
    }

    fn to_proto(&self) -> proto_types::StartGame {
        proto_types::StartGame {}
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Move {
    pub heister_color: HeisterColor,
    pub position: MapPosition,
}

impl Internal for Move {
    type P = proto_types::Move;

    fn from_proto(proto: proto_types::Move) -> Self {
        Move {
            heister_color: HeisterColor::from_i32(proto.heister_color).unwrap(),
            position: MapPosition::from_proto(proto.position.unwrap()),
        }
    }

    fn to_proto(&self) -> proto_types::Move {
        proto_types::Move {
            heister_color: i32::from(self.heister_color),
            position: Some(self.position.to_proto()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InvalidRequest {
    pub reason: String,
}

impl Internal for InvalidRequest {
    type P = proto_types::InvalidRequest;

    fn from_proto(proto: proto_types::InvalidRequest) -> Self {
        InvalidRequest {
            reason: proto.reason,
        }
    }

    fn to_proto(&self) -> proto_types::InvalidRequest {
        proto_types::InvalidRequest {
            reason: self.reason.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlaceTile {
    pub tile_entrance: MapPosition,
}

impl Internal for PlaceTile {
    type P = proto_types::PlaceTile;

    fn from_proto(proto: proto_types::PlaceTile) -> Self {
        PlaceTile {
            tile_entrance: MapPosition::from_proto(proto.tile_entrance.unwrap()),
        }
    }

    fn to_proto(&self) -> proto_types::PlaceTile {
        proto_types::PlaceTile {
            tile_entrance: Some(self.tile_entrance.to_proto()),
        }
    }
}

// JSON Serialization for Tiles
// Since we can't directly add these derives on the proto_types
#[derive(Serialize, Deserialize)]
pub struct SerializableSquare {
    pub north_wall: i32,
    pub east_wall: i32,
    pub south_wall: i32,
    pub west_wall: i32,
    pub square_type: i32,
}

impl From<Square> for SerializableSquare {
    fn from(item: Square) -> Self {
        SerializableSquare {
            north_wall: i32::from(item.north_wall),
            east_wall: i32::from(item.east_wall),
            south_wall: i32::from(item.south_wall),
            west_wall: i32::from(item.west_wall),
            square_type: i32::from(item.square_type),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableTile {
    pub position: MapPosition,
    pub squares: Vec<SerializableSquare>,
    pub name: String,
    pub num_rotations: u32,
}

impl From<Tile> for SerializableTile {
    fn from(item: Tile) -> Self {
        let mut serializable_squares = vec![];
        for square in item.squares {
            serializable_squares.push(SerializableSquare::from(square));
        }
        SerializableTile {
            squares: serializable_squares,
            position: item.position,
            name: item.name,
            num_rotations: item.num_rotations,
        }
    }
}
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PlayerName(pub String);
