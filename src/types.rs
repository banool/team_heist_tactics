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

pub struct MapPosition {
    x: i32,
    y: i32,
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

pub struct Tile {
    squares: Vec<Square>,
    position: MapPosition,
}

impl Internal for Tile {
    type P = proto_types::Tile;

    fn from_proto(proto: proto_types::Tile) -> Self {
        let mut squares : Vec<Square> = vec![];
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
        let mut proto_squares : Vec<proto_types::Square> = vec![];
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

// TODO Implement the rest
// * player
// * move
// * invalidrequest
// * mainmessage

#[derive(Clone, Default)]
pub struct GameState {
    pub game_name: String,
    pub game_started: u64,
    pub game_status: GameStatus,
}

impl Internal for GameState {
    type P = proto_types::GameState;

    fn from_proto(proto: proto_types::GameState) -> Self {
        let game_status = GameStatus::from_i32(proto.game_status).unwrap(); // TODO Handle this gracefully?
        GameState {
            game_name: proto.game_name,
            game_started: proto.game_started,
            game_status,
        }
    }

    fn to_proto(&self) -> proto_types::GameState {
        // TODO Fully specify this, get rid of default.
        proto_types::GameState {
            game_name: self.game_name.to_string(),
            game_started: self.game_started,
            ..Default::default()
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
