fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(
            ".types.MainMessage.body",
            "use serde::{Serialize, Deserialize}; #[derive(Serialize, Deserialize)]",
        )
        .type_attribute(".types.InvalidRequest", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Move", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.GameState", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.GameStatus", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Player", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Ability", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Heister", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.WallType", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.SquareType", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Square", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.Tile", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.MapPosition", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.TilePosition", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.HeisterName", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".types.HeisterColor", "#[derive(Serialize, Deserialize)]")
        .compile_protos(&["src/types.proto"], &["src/"])
        .unwrap();
}
