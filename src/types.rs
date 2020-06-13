// Import all the proto types in this private module.
mod proto_types {
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

// Re-export the enums.
pub use proto_types::GameStatus;
pub use proto_types::HeisterColor;
pub use proto_types::HeisterSymbol;
pub use proto_types::HeisterName;
pub use proto_types::SquareType;
pub use proto_types::WallType;
pub use proto_types::Ability;

// Re-export the MainMessage.
pub use proto_types::MainMessage;
pub mod main_message {
    pub use super::proto_types::main_message::Body;
}


trait Internal {
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

    fn from_proto(proto: proto_types::TilePosition) -> TilePosition {
        TilePosition { x: proto.x, y: proto.y }
    }

    fn to_proto(&self) -> proto_types::TilePosition {
        proto_types::TilePosition { x: self.x, y: self.y }
    }
}
