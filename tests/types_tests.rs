use serde_json;
use team_heist_tactics::load_map::tile_1a;
#[allow(dead_code, unused_imports)]
use team_heist_tactics::types::{MapPosition, Square, Tile};

#[test]
fn load_map_position() {
    let map_position_json = "{\"x\":3,\"y\":5}";
    let mp: MapPosition =
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
