use crate::load_map::tile_1a;
use crate::manager::GameHandle;
use crate::utils::get_current_time_secs;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

// Import all the proto types in this private module.
mod proto_types {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

// Re-export the enums.
pub use proto_types::Ability;
pub use proto_types::GameStatus;
pub use proto_types::HeisterColor;
pub use proto_types::HeisterSymbol;
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

pub const DOOR_TYPES: [&'static WallType; 4] = [
    &WallType::PurpleDoor,
    &WallType::OrangeDoor,
    &WallType::GreenDoor,
    &WallType::YellowDoor,
];

pub const HEISTER_COLORS: [&'static HeisterColor; 4] = [
    &HeisterColor::Purple,
    &HeisterColor::Orange,
    &HeisterColor::Green,
    &HeisterColor::Yellow,
];

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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MoveDirection {
    North,
    East,
    South,
    West,
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
    pub fn is_teleport(&self) -> bool {
        match self.square_type {
            SquareType::PurpleTeleportPad
            | SquareType::YellowTeleportPad
            | SquareType::OrangeTeleportPad
            | SquareType::GreenTeleportPad => true,
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
            // | |y|o| |
            // | |p|g| |
            // | | | | |
            StartingTile::A(_) => match heister_color {
                HeisterColor::Yellow => MapPosition { x: 1, y: 1 },
                HeisterColor::Purple => MapPosition { x: 1, y: 2 },
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
pub struct GameState {
    pub game_name: GameHandle,
    pub game_started: u64,
    pub timer_runs_out: u64,
    pub tiles: Vec<Tile>,
    pub heisters: Vec<Heister>,
    pub all_items_taken: bool,
    pub remaining_tiles: u32,
    pub game_status: GameStatus,
    pub players: Vec<Player>,
    pub possible_placements: Vec<MapPosition>,
}

impl Internal for GameState {
    type P = proto_types::GameState;

    fn from_proto(proto: proto_types::GameState) -> Self {
        let game_name = GameHandle(proto.game_name);
        let tiles = proto
            .tiles
            .iter()
            .map(|t| Tile::from_proto(t.clone()))
            .collect();
        let heisters = proto
            .heisters
            .iter()
            .map(|h| Heister::from_proto(h.clone()))
            .collect();
        let players = proto
            .players
            .iter()
            .map(|p| Player::from_proto(p.clone()))
            .collect();
        let game_status = GameStatus::from_i32(proto.game_status).unwrap(); // TODO Handle this gracefully?
        let possible_placements = proto
            .possible_placements
            .iter()
            .map(|pp| MapPosition::from_proto(pp.clone()))
            .collect();
        GameState {
            game_name,
            game_started: proto.game_started,
            timer_runs_out: proto.timer_runs_out,
            tiles,
            heisters,
            all_items_taken: proto.all_items_taken,
            remaining_tiles: proto.remaining_tiles,
            game_status,
            players,
            possible_placements,
        }
    }

    fn to_proto(&self) -> proto_types::GameState {
        let tiles = self.tiles.iter().map(|t| t.to_proto()).collect();
        let heisters = self.heisters.iter().map(|h| h.to_proto()).collect();
        let players = self.players.iter().map(|p| p.to_proto()).collect();
        let possible_placements = self
            .possible_placements
            .iter()
            .map(|pp| pp.to_proto())
            .collect();
        let game_status = i32::from(self.game_status);
        proto_types::GameState {
            game_name: self.game_name.0.to_string(),
            game_started: self.game_started,
            timer_runs_out: self.timer_runs_out,
            tiles,
            heisters,
            all_items_taken: self.all_items_taken,
            remaining_tiles: self.remaining_tiles,
            game_status,
            players,
            possible_placements,
        }
    }
}

const TIMER_DURATION_SECS: u64 = 5 * 60;

impl GameState {
    pub fn new(game_name: GameHandle) -> Self {
        let game_started = get_current_time_secs();
        let timer_runs_out = game_started + TIMER_DURATION_SECS;
        let starting_tile = tile_1a();
        let starting_tile_enum = StartingTile::A(starting_tile.clone());
        let tiles = vec![starting_tile.clone()];
        let mut heisters = Vec::new();
        heisters.push(Heister::get_initial(
            HeisterColor::Yellow,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Purple,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Green,
            &starting_tile_enum,
        ));
        heisters.push(Heister::get_initial(
            HeisterColor::Orange,
            &starting_tile_enum,
        ));
        let possible_placements: Vec<MapPosition> = Vec::new();
        GameState {
            game_name,
            game_started,
            timer_runs_out,
            tiles,
            heisters,
            all_items_taken: false,
            remaining_tiles: 8,
            game_status: GameStatus::Staging,
            players: vec![],
            possible_placements,
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

#[allow(dead_code, unused_imports)]
mod tests {
    use super::{Square, Tile};
    use crate::load_map::tile_1a;
    use serde_json;
    #[test]
    fn load_map_position() {
        let map_position_json = "{\"x\":3,\"y\":5}";
        let mp: super::MapPosition =
            serde_json::from_str(map_position_json).expect("Failed to load from json");
        assert_eq!(mp.x, 3);
        assert_eq!(mp.y, 5);
        let out_json = serde_json::to_string(&mp).expect("Failed to write back to json");
        assert_eq!(map_position_json, out_json);
    }

    #[test]
    fn tile_rotation_identity() {
        let t = tile_1a();
        let m = t.to_matrix();
        let mut m2 = Tile::rotate_matrix_clockwise(&m);
        for _ in 0..3 {
            m2 = Tile::rotate_matrix_clockwise(&m2);
        }
        let u = Tile::from_matrix(m2, t.name.clone(), t.position.clone(), 0);
        assert_eq!(t, u);
    }
}
