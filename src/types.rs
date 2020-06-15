use crate::manager::GameHandle;
use crate::utils::get_current_time_secs;

// Import all the proto types in this private module.
mod proto_types {
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}


// Re-export the enums.
pub use proto_types::Ability;
pub use proto_types::GameStatus;
pub use proto_types::HeisterColor;
pub use proto_types::HeisterName;
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct Tile {
    pub squares: Vec<Square>,
    pub position: MapPosition,
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
        }
    }
}

pub enum StartingTile {
    A(Tile),
    B(Tile),
}

#[derive(Clone, Debug, Default)]
pub struct Square {
    north_wall: WallType,
    east_wall: WallType,
    south_wall: WallType,
    west_wall: WallType,
    square_type: SquareType,
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

#[derive(Clone, Debug, Default)]
pub struct Heister {
    heister_color: HeisterColor,
    map_position: MapPosition,
    has_taken_item: bool,
    has_escaped: bool,
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
            StartingTile::A(_) => match heister_color {
                HeisterColor::Yellow => MapPosition { x: 1, y: 2 },
                HeisterColor::Purple => MapPosition { x: 1, y: 2 },
                HeisterColor::Green => MapPosition { x: 2, y: 2 },
                HeisterColor::Orange => MapPosition { x: 2, y: 1 },
            }
            _ => MapPosition { x: 0, y: 0 },  // TODO Do this for starting B side.
        };
        Heister {
            heister_color,
            map_position,
            has_taken_item: false,
            has_escaped: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct Move {
    heister_color: HeisterColor,
    position: MapPosition,
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

#[derive(Clone, Debug, Default)]
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
}

impl Internal for GameState {
    type P = proto_types::GameState;

    fn from_proto(proto: proto_types::GameState) -> Self {
        let game_name = GameHandle(proto.game_name);
        let tiles = proto.tiles.iter().map(|t| Tile::from_proto(t.clone())).collect();
        let heisters = proto.heisters.iter().map(|h| Heister::from_proto(h.clone())).collect();
        let players = proto.players.iter().map(|p| Player::from_proto(p.clone())).collect();
        let game_status = GameStatus::from_i32(proto.game_status).unwrap(); // TODO Handle this gracefully?
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
        }
    }

    fn to_proto(&self) -> proto_types::GameState {
        let tiles = self.tiles.iter().map(|t| t.to_proto()).collect();
        let heisters = self.heisters.iter().map(|h| h.to_proto()).collect();
        let players = self.players.iter().map(|p| p.to_proto()).collect();
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
        }
    }
}

const TIMER_DURATION_SECS: u64 = 5 * 60;

impl GameState {
    pub fn new(game_name: GameHandle) -> Self {
        let game_started = get_current_time_secs();
        let timer_runs_out = game_started + TIMER_DURATION_SECS;
        let starting_tile = Tile { squares: vec![], position: MapPosition {x:0, y:0} };
        let tiles = vec![starting_tile.clone()];
        let mut heisters = Vec::new();
        let starting_tile_enum = StartingTile::A(starting_tile);
        heisters.push(Heister::get_initial(HeisterColor::Yellow, &starting_tile_enum));
        heisters.push(Heister::get_initial(HeisterColor::Purple, &starting_tile_enum));
        heisters.push(Heister::get_initial(HeisterColor::Green, &starting_tile_enum));
        heisters.push(Heister::get_initial(HeisterColor::Orange, &starting_tile_enum));
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
        }
    }
}

#[derive(Clone, Default)]
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
