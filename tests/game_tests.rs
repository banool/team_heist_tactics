#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;

use log::info;
use std::collections::HashMap;
use team_heist_tactics::game::{Game, GameHandle, GameOptions, MoveValidity};
use team_heist_tactics::load_map::{tile_1a, tile_2, tile_5, tile_8};
use team_heist_tactics::types::{
    main_message::Body, Heister, HeisterColor, Internal, MainMessage, MapPosition, Move,
    MoveDirection, PlaceTile, PlayerName, Square, Tile, HEISTER_COLORS,
};

lazy_static! {
    static ref FAKE_PLAYER_NAME: PlayerName = PlayerName("fake name".to_string());
}

fn setup_game(handle: String) -> Game {
    let _ = env_logger::builder().is_test(true).try_init();
    let game_handle = GameHandle(handle);
    let game_options = GameOptions::default();
    let mut game = Game::new(game_handle, game_options);
    game.add_player(FAKE_PLAYER_NAME.0.clone()).unwrap();
    game.start_game();
    game
}

/// Adjacent square movement for heisters, to make testing easier
/// Asserts that move was valid & that position is correct for valid move
fn move_heister_in_dir(
    game: &mut Game,
    heister_color: HeisterColor,
    dir: MoveDirection,
    expected_validity: MoveValidity,
) -> MoveValidity {
    let heister_pos = &game
        .game_state
        .get_heister_from_vec(heister_color)
        .unwrap()
        .map_position;
    let position = match dir {
        MoveDirection::North => MapPosition {
            x: heister_pos.x,
            y: heister_pos.y - 1,
        },
        MoveDirection::East => MapPosition {
            x: heister_pos.x + 1,
            y: heister_pos.y,
        },
        MoveDirection::South => MapPosition {
            x: heister_pos.x,
            y: heister_pos.y + 1,
        },
        MoveDirection::West => MapPosition {
            x: heister_pos.x - 1,
            y: heister_pos.y,
        },
    };
    let test_move = Move {
        heister_color,
        position: position.clone(),
    };
    let message = MainMessage {
        body: Some(Body::Move(test_move.to_proto())),
    };
    let validity = game.handle_message(message, &FAKE_PLAYER_NAME);
    assert_eq!(validity, expected_validity);
    match validity.clone() {
        MoveValidity::Valid => {
            let curr_heister_pos = &game
                .game_state
                .get_heister_from_vec(heister_color)
                .unwrap()
                .map_position;
            assert_eq!(curr_heister_pos, &position);
        }
        _invalid => {}
    }
    validity
}

/// TODO: must be generalized for any tile placement
/// currently only works for initial second tile Orange North tile 1a placement
fn place_first_tile_for_color(
    game: &mut Game,
    _heister_color: HeisterColor,
    tile_entrance: MapPosition,
    expected_validity: MoveValidity,
) -> MoveValidity {
    // needs to assert that heister color is correct, etc. or not! i don't care
    let tile_placement = PlaceTile { tile_entrance };
    let message = MainMessage {
        body: Some(Body::PlaceTile(tile_placement.to_proto())),
    };
    let validity = game.handle_message(message, &FAKE_PLAYER_NAME);
    assert_eq!(validity, expected_validity);

    for tile in &game.game_state.tiles {
        if tile.name == "1a".to_string() {
            let mp_00 = MapPosition { x: 0, y: 0 };
            assert_eq!(tile.position, mp_00);
        } else {
            // No matter the tile name, if we use this path to draw it, its
            // position should be here.
            let mp_1neg3 = MapPosition { x: 1, y: -4 };
            assert_eq!(tile.position, mp_1neg3);
        }
    }
    validity
}

/// Assuming that Yellow starts at 1, 1
/// This test tries to move it up (safe),
/// Then back down to its starting square
/// Checks that the moves are accepted as valid
#[test]
pub fn test_can_move_to_free_square() -> () {
    let handle = "test can move to free square".to_string();
    let mut game = setup_game(handle);
    let _ = env_logger::builder().is_test(true).try_init();

    // Confirm yellow heister is where we expect it to be to begin with.
    let heister_color = HeisterColor::Purple;
    let heister_pos = &game
        .game_state
        .get_heister_from_vec(heister_color)
        .unwrap()
        .map_position;
    assert_eq!(heister_pos.x, 1);
    assert_eq!(heister_pos.y, 1);

    // Move Yellow Up into a free space
    let validity = move_heister_in_dir(
        &mut game,
        heister_color,
        MoveDirection::North,
        MoveValidity::Valid,
    );
    assert_eq!(validity, MoveValidity::Valid);

    // Move Yellow back down into the space it occupied - that should be safe
    let validity = move_heister_in_dir(
        &mut game,
        heister_color,
        MoveDirection::South,
        MoveValidity::Valid,
    );
    assert_eq!(validity, MoveValidity::Valid);
}

#[test]
pub fn heister_collision_is_invalid() -> () {
    let handle = "heister collision is invalid".to_string();
    let mut game = setup_game(handle);
    // Assuming that Green starts at 1, 1 and Orange at 2, 1
    // This test tries to move it up and expects an invalid move
    // because Orange is there

    // Confirm green heister is where we expect it to be to begin with.
    let src_position = MapPosition { x: 2, y: 2 };
    let heister_color = HeisterColor::Green;
    let heister_pos = &game
        .game_state
        .get_heister_from_vec(heister_color)
        .unwrap()
        .map_position;
    assert_eq!(heister_pos.x, src_position.x);
    assert_eq!(heister_pos.y, src_position.y);

    let dest_position = MapPosition { x: 2, y: 1 };
    let expected_msg = format!(
        "Heister {:?} is on {:?}",
        HeisterColor::Orange,
        dest_position
    );
    let expected_validity = MoveValidity::Invalid(expected_msg);
    move_heister_in_dir(
        &mut game,
        HeisterColor::Green,
        MoveDirection::North,
        expected_validity,
    );
    let curr_green_pos = game
        .game_state
        .get_heister_from_vec(HeisterColor::Green)
        .unwrap();
    assert_eq!(&curr_green_pos.map_position, &src_position);
}

#[test]
pub fn grid_walls_align() -> () {
    let handle = "grid walls align".to_string();
    let game = setup_game(handle);
    let grid: HashMap<MapPosition, Square> = game.game_state.get_absolute_grid();

    for (mp, square) in grid.iter() {
        // Check left wall lines up.
        if mp.x > 0 {
            let index = MapPosition {
                x: mp.x - 1,
                y: mp.y,
            };
            let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
            let left = grid.get(&index).expect(&msg);
            assert_eq!(square.west_wall, left.east_wall);
        }
        // Check right wall lines up.
        if mp.x < 3 {
            let index = MapPosition {
                x: mp.x + 1,
                y: mp.y,
            };
            let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
            let right = grid.get(&index).expect(&msg);
            assert_eq!(square.east_wall, right.west_wall);
        }
        // Check top wall lines up.
        if mp.y > 0 {
            let index = MapPosition {
                x: mp.x,
                y: mp.y - 1,
            };
            let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
            let above = grid.get(&index).expect(&msg);
            assert_eq!(square.north_wall, above.south_wall);
        }
        // Check bottom wall lines up.
        if mp.y < 3 {
            let index = MapPosition {
                x: mp.x,
                y: mp.y + 1,
            };
            let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
            let below = grid.get(&index).expect(&msg);
            assert_eq!(square.south_wall, below.north_wall);
        }
    }
    info!("All walls line up");
}

/// We test with initial game state (1a), move Orange one square north,
/// and then send a drawTile message.
#[test]
pub fn test_tile_draw() -> () {
    let handle = "grid walls align".to_string();
    let mut game = setup_game(handle);
    let first_tile_entrance = MapPosition { x: 2, y: -1 };

    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::North,
        MoveValidity::Valid,
    );
    place_first_tile_for_color(
        &mut game,
        HeisterColor::Orange,
        first_tile_entrance,
        MoveValidity::Valid,
    );
}

/// Ensure that we generate possible placements that are correct for the color
/// of heister & door.
#[test]
pub fn possible_placements_no_mismatched_results() -> () {
    let handle = "possible placements no mismatched results".to_string();
    let mut game = setup_game(handle);
    // Set up the game such that many heisters are at matching doors

    // Set up correct, happy, matching case first:
    let orange_door_pos = MapPosition { x: 2, y: 0 };
    let purple_door_pos = MapPosition { x: 0, y: 1 };
    let yellow_door_pos = MapPosition { x: 1, y: 3 };
    let green_door_pos = MapPosition { x: 3, y: 2 };
    let mut color_to_door_pos: HashMap<HeisterColor, MapPosition> = HashMap::new();
    color_to_door_pos.insert(HeisterColor::Orange, orange_door_pos);
    color_to_door_pos.insert(HeisterColor::Purple, purple_door_pos);
    color_to_door_pos.insert(HeisterColor::Yellow, yellow_door_pos);
    color_to_door_pos.insert(HeisterColor::Green, green_door_pos);

    let mut heisters: Vec<Heister> = Vec::new();
    for hc in HEISTER_COLORS.iter() {
        let mut h = Heister::default();
        h.heister_color = *hc.clone();
        h.map_position = color_to_door_pos.get(hc).unwrap().clone();
        heisters.push(h);
    }

    // Move heister in place
    game.game_state.heisters = heisters;
    let dest_position = MapPosition { x: 2, y: 0 };
    let test_move = Move {
        heister_color: HeisterColor::Orange,
        position: dest_position,
    };
    let message = MainMessage {
        body: Some(Body::Move(test_move.to_proto())),
    };
    game.handle_message(message, &FAKE_PLAYER_NAME); // don't care if this move is valid

    let pp = game.game_state.possible_placements;
    assert_eq!(pp.len(), 4);
    // TODO: assert the positions in PP are as expected, this is annoying
    // because PP is the tile entrance, not the heister pos.
    // could short circuit it by directly calling the functioning returning the
    // dict?
}

#[test]
pub fn can_use_escalators() -> () {
    let handle = "any heister can walk into an escalator square".to_string();
    let mut game = setup_game(handle);

    // We only need Green (since green on 1a can immediately move to escalator)
    move_heister_in_dir(
        &mut game,
        HeisterColor::Green,
        MoveDirection::South,
        MoveValidity::Valid,
    );

    let green_door_pos = MapPosition { x: 3, y: 2 };
    let esc_move = Move {
        heister_color: HeisterColor::Green,
        position: green_door_pos,
    };
    let message = MainMessage {
        body: Some(Body::Move(esc_move.to_proto())),
    };
    let validity = game.handle_message(message, &FAKE_PLAYER_NAME);
    assert_eq!(validity, MoveValidity::Valid);
}

#[test]
pub fn can_enter_mismatched_teleport() -> () {
    let handle = "any heister can walk into a mismatched teleport square".to_string();
    let mut game = setup_game(handle);

    // We only need Orange (since orange on 1a can immediately move to yellow telly)
    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::East,
        MoveValidity::Valid,
    );
}

/// Helper for rotating tiles.
/// TODO: cleanup - make this a function on the tile!
fn rotate_tile(tile: Tile) -> Tile {
    let mut m = tile.to_matrix();
    for _ in 0..tile.num_rotations {
        m = Tile::rotate_matrix_clockwise(&m);
    }
    Tile::from_matrix(
        m,
        tile.name.clone(),
        tile.position.clone(),
        tile.num_rotations,
    )
}

/// One test case I discovered was that adjacent teleports failed, because
/// they were processed as walking moves that tried to bypass walls.
#[test]
pub fn can_teleport_between_adjacent_teleporters() -> () {
    let handle = "can teleport between adjacent teleporters".to_string();
    // How the heck do we construct adjacent teleporters from the base deck?
    let mut game = setup_game(handle);
    let tile_1 = tile_1a();
    let mut tile_8 = tile_8();
    tile_8.position = MapPosition { x: 5, y: 1 };
    tile_8.num_rotations = 1;
    tile_8 = rotate_tile(tile_8);
    // need to also actually ROTATE tile 8

    let mut tile_2 = tile_2();
    tile_2.position = MapPosition { x: 6, y: -3 };
    tile_2.num_rotations = 0;
    let mut tile_5 = tile_5();
    tile_5.position = MapPosition { x: 1, y: -4 };
    tile_5.num_rotations = 3;
    tile_5 = rotate_tile(tile_5);
    let tiles: Vec<Tile> = vec![tile_1, tile_8, tile_2, tile_5];
    game.game_state.tiles = tiles;

    // Let's have purple try to teleport once to enter the first teleporter
    // that one is at 3, 0
    // Then, we'll have it try to teleport to its next teleporter, at 3, -1

    move_heister_in_dir(
        &mut game,
        HeisterColor::Purple,
        MoveDirection::North,
        MoveValidity::Valid,
    );
    move_heister_in_dir(
        &mut game,
        HeisterColor::Purple,
        MoveDirection::East,
        MoveValidity::Valid,
    );
    move_heister_in_dir(
        &mut game,
        HeisterColor::Purple,
        MoveDirection::East,
        MoveValidity::Valid,
    );
    // THIS move is the attempted teleport. I expect it to succeed, in spite
    // of the wall between these two adjacent squares.
    move_heister_in_dir(
        &mut game,
        HeisterColor::Purple,
        MoveDirection::North,
        MoveValidity::Valid,
    );
}

/// We test with initial game state (1a), move Orange one square north,
/// and then send a drawTile message.
#[test]
pub fn test_new_tile_crossing() -> () {
    let handle = "new tile crossing".to_string();
    let mut game = setup_game(handle);
    let first_tile_entrance = MapPosition { x: 2, y: -1 };

    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::North,
        MoveValidity::Valid,
    );
    place_first_tile_for_color(
        &mut game,
        HeisterColor::Orange,
        first_tile_entrance,
        MoveValidity::Valid,
    );

    // Next, we want to move orange UP, then down.
    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::North,
        MoveValidity::Valid,
    );
    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::South,
        MoveValidity::Valid,
    );
}

// Ensure that a player with no abilities can't do anything.
#[test]
pub fn test_ability_check() -> () {
    let handle = "new tile crossing".to_string();
    let mut game = setup_game(handle);
    game.game_state.players[0].abilities = vec![];
    let first_tile_entrance = MapPosition { x: 2, y: -1 };

    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::North,
        MoveValidity::Invalid("You cannot move heisters North".to_string()),
    );
    place_first_tile_for_color(
        &mut game,
        HeisterColor::Orange,
        first_tile_entrance,
        MoveValidity::Invalid("You cannot reveal tiles".to_string()),
    );
    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::North,
        MoveValidity::Invalid("You cannot move heisters North".to_string()),
    );
    move_heister_in_dir(
        &mut game,
        HeisterColor::Orange,
        MoveDirection::South,
        MoveValidity::Invalid("You cannot move heisters South".to_string()),
    );
}
