use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::convert::TryInto;

use crate::load_map;
use crate::manager::{GameHandle, GameOptions};
use crate::types::main_message::Body;
use crate::types::{
    GameState, GameStatus, Heister, HeisterColor, Internal, MainMessage, MapPosition, Move,
    MoveDirection, PlaceTile, Player, Square, Tile, WallType, DOOR_TYPES,
};

use log::{info, trace};

#[derive(Debug)]
pub struct Game {
    pub game_handle: GameHandle,
    pub game_state: GameState,
    pub tile_deck: Vec<Tile>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MoveValidity {
    Valid,
    Invalid(String),
}

impl Game {
    pub fn new(game_handle: GameHandle, _game_options: GameOptions) -> Game {
        let game_state = GameState::new(game_handle.clone());
        let tile_deck: Vec<Tile> = load_map::load_tiles_from_json();
        Game {
            game_handle,
            game_state,
            tile_deck,
        }
    }

    fn draw_tile(&mut self) -> Option<Tile> {
        let tile = self.tile_deck.pop();
        self.game_state.remaining_tiles = self.tile_deck.len().try_into().unwrap();
        tile
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        if self.game_state.game_status != GameStatus::Staging {
            return Err(anyhow!("Cannot join game that is already in progress"));
        }
        self.game_state.players.push(Player {
            name,
            abilities: vec![],
        });
        Ok(())
    }

    pub fn has_player(&self, name: &str) -> bool {
        for p in self.game_state.players.iter() {
            if p.name == name {
                return true;
            }
        }
        false
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state.clone()
    }

    pub fn get_absolute_grid(&self) -> HashMap<MapPosition, Square> {
        let mut grid: HashMap<MapPosition, Square> = HashMap::new();
        for tile in self.game_state.tiles.iter() {
            // this is the top position for the tile - we can assign positions for this
            let tile_pos = &tile.position;
            for (i, square) in tile.squares.iter().enumerate() {
                let sq_x = (i % 4) as i32;
                let sq_y = (i / 4) as i32;
                let grid_x = tile_pos.x + sq_x;
                let grid_y = tile_pos.y + sq_y;
                trace!(
                    "{}: {:?} {:?} {:?} {:?}, {:?}",
                    i,
                    square.north_wall,
                    square.west_wall,
                    square.south_wall,
                    square.east_wall,
                    square.square_type
                );
                let mp = MapPosition {
                    x: grid_x,
                    y: grid_y,
                };
                grid.insert(mp, square.clone());
            }
        }
        grid
    }

    fn are_adjacent(my_pos: &MapPosition, other_pos: &MapPosition) -> bool {
        if my_pos.x == other_pos.x {
            let abs_distance = (my_pos.y - other_pos.y).abs();
            return abs_distance == 1;
        } else if my_pos.y == other_pos.y {
            let abs_distance = (my_pos.x - other_pos.x).abs();
            return abs_distance == 1;
        } else {
            return false;
        }
    }

    fn adjacent_move_direction(my_pos: &MapPosition, other_pos: &MapPosition) -> MoveDirection {
        // NOTE: I assume that the two positions are adjacent. Might not be relevant
        // Also suffers from NO VALIDATION AT ALL illness
        if my_pos.x > other_pos.x {
            return MoveDirection::West;
        } else if my_pos.x < other_pos.x {
            return MoveDirection::East;
        } else if my_pos.y > other_pos.y {
            return MoveDirection::North;
        } else {
            return MoveDirection::South;
        }
    }

    // NOTE: Would be nice if self.game_state.heisters was a map<color, heister>
    // or even <color, pos>
    fn get_mut_heister_from_vec(&mut self, hc: HeisterColor) -> Option<&mut Heister> {
        for h in self.game_state.heisters.iter_mut() {
            if h.heister_color == hc {
                return Some(h);
            }
        }
        return None;
    }

    fn get_heister_from_vec(&self, hc: HeisterColor) -> Option<&Heister> {
        for h in self.game_state.heisters.iter() {
            if h.heister_color == hc {
                return Some(h);
            }
        }
        return None;
    }

    fn move_blocked_by_wall(
        &self,
        heister_pos: &MapPosition,
        dest_pos: &MapPosition,
    ) -> MoveValidity {
        // Assumes that heister_pos & dest_pos are adjacent
        let grid = self.get_absolute_grid();
        let heister_square = match grid.get(&heister_pos) {
            Some(s) => s,
            None => {
                return MoveValidity::Invalid(format!(
                    "Heister square {:?} doesn't exist",
                    heister_pos
                ))
            }
        };
        let move_dir = Self::adjacent_move_direction(heister_pos, dest_pos);
        let blocking_wall = match move_dir {
            MoveDirection::North => heister_square.north_wall,
            MoveDirection::East => heister_square.east_wall,
            MoveDirection::South => heister_square.south_wall,
            MoveDirection::West => heister_square.west_wall,
        };

        match blocking_wall {
            WallType::Clear => MoveValidity::Valid,
            WallType::Impassable => {
                MoveValidity::Invalid("Can't pass through impassable wall".to_string())
            }
            // Wildcard matches each tile-discovery type (one per color)
            _wildcard => {
                MoveValidity::Invalid("Moving to un-placed tile not implemented yet".to_string())
            }
        }
    }

    fn position_is_occupied(&self, position: &MapPosition) -> MoveValidity {
        for h in &self.game_state.heisters {
            match &h.map_position == position {
                true => {
                    let msg = format!("Heister {:?} is on {:?}", h.heister_color, position);
                    return MoveValidity::Invalid(msg);
                }
                false => {}
            }
        }
        return MoveValidity::Valid;
    }

    fn get_door_wall(square: &Square) -> Option<WallType> {
        // Return the square's (exit) door, if it has one
        let walls = square.get_walls();
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

    fn get_door_direction(square: &Square) -> Option<MoveDirection> {
        // Return the direction of the square's (exit) door, if it has one
        match Self::get_door_wall(square) {
            Some(_) => (),
            None => return None,
        };
        for (dir, wall) in square.get_walls().iter() {
            if DOOR_TYPES.contains(&wall) {
                return Some(dir.clone());
            }
        }
        None
    }

    fn move_in_direction(position: MapPosition, direction: &MoveDirection) -> MapPosition {
        // Given a position and direction, return the position if you were to
        // "Move" in that direction (one square)
        match direction {
            MoveDirection::North => MapPosition {
                x: position.x,
                y: position.y - 1,
            },
            MoveDirection::East => MapPosition {
                x: position.x + 1,
                y: position.y,
            },
            MoveDirection::South => MapPosition {
                x: position.x,
                y: position.y + 1,
            },
            MoveDirection::West => MapPosition {
                x: position.x - 1,
                y: position.y,
            },
        }
    }

    fn heister_tile_placement_positions(
        &self,
        grid: &HashMap<MapPosition, Square>,
    ) -> Vec<MapPosition> {
        // Return the places from which you could draw a tile
        // AKA - squares where a matching heister is on a square with a HeisterColor door
        let mut placement_locations: Vec<MapPosition> = Vec::new();
        for heister in &self.game_state.heisters {
            let color = heister.heister_color;
            let square = grid
                .get(&heister.map_position)
                .expect("Heister on invalid square");
            let maybe_door = Self::get_door_wall(square);
            let door = match maybe_door {
                Some(d) => d,
                None => continue,
            };
            // TODO: put this in a helper?
            match door {
                WallType::PurpleDoor => {
                    if color == HeisterColor::Purple {
                        placement_locations.push(heister.map_position.clone());
                    }
                }
                WallType::OrangeDoor => {
                    if color == HeisterColor::Orange {
                        placement_locations.push(heister.map_position.clone());
                    }
                }
                WallType::GreenDoor => {
                    if color == HeisterColor::Green {
                        placement_locations.push(heister.map_position.clone());
                    }
                }
                WallType::YellowDoor => {
                    if color == HeisterColor::Purple {
                        placement_locations.push(heister.map_position.clone());
                    }
                }
                _wildcard => (),
            }
        }
        placement_locations
    }

    fn heister_to_tile_entrance_positions(
        &self,
        grid: &HashMap<MapPosition, Square>,
    ) -> HashMap<MapPosition, MapPosition> {
        // Returns a map from current heister positions to their prospective tile_entrance
        // positions, one tile away (if there are any such locations)
        let heister_door_positions = self.heister_tile_placement_positions(&grid);
        let mut tile_entrance_positions: HashMap<MapPosition, MapPosition> = HashMap::new();
        for heister_pos in heister_door_positions {
            let square = grid
                .get(&heister_pos)
                .expect("Heister must be on a valid square");
            let dir = &Self::get_door_direction(square)
                .expect("Square must have a door on it to be entered through");
            tile_entrance_positions.insert(
                heister_pos.clone(),
                Self::move_in_direction(heister_pos, dir),
            );
        }
        tile_entrance_positions
    }

    fn update_possible_placements(&mut self) -> () {
        let grid = self.get_absolute_grid();
        let heister_to_tile_entrance_locs = self.heister_to_tile_entrance_positions(&grid);

        let mut v = Vec::new();
        for val in heister_to_tile_entrance_locs.values() {
            v.push(val.clone());
        }
        self.game_state.possible_placements = v.clone();
    }

    fn new_tile_position(position: &MapPosition, dir: &MoveDirection) -> MapPosition {
        // From a tile entrance and move direction of the tile's orientation,
        // return the MapPosition for that new tile to place it in the absolute grid
        // This is doable since every tile has an entry square in some rotation of
        // (1, 3) - except for starting tiles
        match dir {
            MoveDirection::North => MapPosition {
                x: position.x - 1,
                y: position.y - 3,
            },
            MoveDirection::East => MapPosition {
                x: position.x,
                y: position.y - 1,
            },
            MoveDirection::South => MapPosition {
                x: position.x - 2,
                y: position.y,
            },
            MoveDirection::West => MapPosition {
                x: position.x - 3,
                y: position.y - 2,
            },
        }
    }

    fn process_move(&mut self, m: Move) -> MoveValidity {
        let heister_color = m.heister_color;
        let heister = self.get_heister_from_vec(heister_color).unwrap();
        let heister_pos = &heister.map_position;
        let dest_pos = m.position;

        let grid = self.get_absolute_grid();
        match grid.get(&dest_pos) {
            None => {
                return MoveValidity::Invalid(format!(
                    "Destination square {:?} doesn't exist",
                    dest_pos
                ))
            }
            Some(_wildcard) => (),
        };
        if Self::are_adjacent(heister_pos, &dest_pos) {
            let validity = self.move_blocked_by_wall(&heister_pos, &dest_pos);
            match validity {
                MoveValidity::Invalid(_) => return validity,
                _ => (),
            }
            let validity = self.position_is_occupied(&dest_pos);

            if validity == MoveValidity::Valid {
                // If the move is valid, actually move it
                let heister = self
                    .get_mut_heister_from_vec(heister_color.clone())
                    .unwrap();
                heister.map_position = dest_pos;
                self.update_possible_placements();
            }
            validity
        } else {
            // If they're not adjacent, then we can check whether the destination is a
            // matching teleport, and whether teleportation is allowed right now
            return MoveValidity::Invalid("Teleports & Escalators not implemented yet".to_string());
        }
    }

    fn place_tile(&mut self, position: &MapPosition, direction: &MoveDirection) -> MoveValidity {
        let tile = self.draw_tile();
        match tile {
            // TODO: figure out how to handle mismatched door case for {Color}Door
            // into TileEntrance (mismatched/asymmetric walls on tile bounds)
            Some(t) => {
                let new_pos = Self::new_tile_position(position, direction);
                let num_rotations = match direction {
                    MoveDirection::North => 0,
                    MoveDirection::East => 1,
                    MoveDirection::South => 2,
                    MoveDirection::West => 3,
                };
                let mut m: Vec<Vec<Square>> = t.to_matrix();
                for _ in 0..=num_rotations {
                    m = Tile::rotate_matrix_clockwise(&m);
                }

                let rotated_tile = &Tile::from_matrix(m, t.name.clone(), new_pos);
                self.game_state.tiles.push(rotated_tile.clone());

                info!(
                    "Added Tile {} at {:?} to Game map",
                    rotated_tile.name, rotated_tile.position
                );
                MoveValidity::Valid
            }
            None => MoveValidity::Invalid("No tiles left in deck to draw".to_string()),
        }
    }

    fn process_tile_placement(&mut self, pt: PlaceTile) -> MoveValidity {
        let grid = self.get_absolute_grid();
        let heister_to_tile_entrance_locs = self.heister_to_tile_entrance_positions(&grid);
        let maybe_heister_pos_tuple = heister_to_tile_entrance_locs
            .iter()
            .find(|&(_, te)| te == &pt.tile_entrance);
        let heister_pos = match maybe_heister_pos_tuple {
            Some(pos_tuple) => pos_tuple.0,
            None => {
                return MoveValidity::Invalid(
                    "When placing a tile, the heister must be at a tile-reveal door".to_string(),
                );
            }
        };

        let heister_square = grid
            .get(heister_pos)
            .expect("Heister must be on valid square");
        let dir = &Self::get_door_direction(heister_square)
            .expect("Heister must be on a square with a door");

        let validity = self.place_tile(&pt.tile_entrance, dir);
        self.update_possible_placements();
        validity
    }

    pub fn handle_message(&mut self, message: MainMessage) -> MoveValidity {
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        info!("Received message: {:?}", message);
        let body = message.body.unwrap();
        let validity = match body {
            Body::Move(m) => self.process_move(Move::from_proto(m)),
            Body::PlaceTile(pt) => self.process_tile_placement(PlaceTile::from_proto(pt)),
            Body::GameState(_gs) => {
                MoveValidity::Invalid("GameState Message is invalid from players".to_string())
            }
            Body::InvalidRequest(_ir) => {
                MoveValidity::Invalid("InvalidRequest Message is invalid from players".to_string())
            }
        };
        validity
    }
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
pub mod tests {
    use crate::manager::{GameHandle, GameOptions};
    use crate::types::{
        HeisterColor, Internal, MainMessage, MapPosition, Move, MoveDirection, Player, Square,
        WallType,
    };
    use log::{info, warn};
    use std::collections::HashMap;

    #[test]
    pub fn test_can_move_to_free_square() -> () {
        let _ = env_logger::builder().is_test(true).try_init();
        // Assuming that Yellow starts at 1, 1
        // This test tries to move it up (safe),
        // Then back down to its starting square
        // Checks that the moves are accepted as valid
        let game_handle = super::GameHandle("test_can_move_to_free_square".to_string());
        let game_options = super::GameOptions::default();
        let mut game = super::Game::new(game_handle, game_options);

        // Confirm yellow heister is where we expect it to be to begin with.
        let heister_color = super::HeisterColor::Yellow;
        assert_eq!(
            game.get_heister_from_vec(heister_color)
                .unwrap()
                .map_position
                .x,
            1
        );
        assert_eq!(
            game.get_heister_from_vec(heister_color)
                .unwrap()
                .map_position
                .y,
            1
        );

        let position = super::MapPosition { x: 1, y: 0 };
        let test_move = super::Move {
            heister_color,
            position: position.clone(),
        };
        let message = MainMessage {
            body: Some(super::Body::Move(test_move.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Valid);
        let mut curr_yellow_pos = game
            .get_heister_from_vec(super::HeisterColor::Yellow)
            .unwrap();
        assert_eq!(&curr_yellow_pos.map_position, &position);

        // THIS FOLLOWING PART *SHOULD* pass - but doesn't! Need unit tests on
        // tiles & tile loading - to ensure that tiles' walls are symmetric
        let next_position = super::MapPosition { x: 1, y: 1 };
        let test_move = super::Move {
            heister_color,
            position: next_position.clone(),
        };
        let message = MainMessage {
            body: Some(super::Body::Move(test_move.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Valid);
        curr_yellow_pos = game
            .get_heister_from_vec(super::HeisterColor::Yellow)
            .unwrap();
        assert_eq!(&curr_yellow_pos.map_position, &next_position);
    }

    #[test]
    pub fn heister_collision_is_invalid() -> () {
        let _ = env_logger::builder().is_test(true).try_init();
        // Assuming that Yellow starts at 1, 1
        // This test tries to move it up (safe),
        // Then back down to its starting square
        // Checks that the moves are accepted as valid
        let game_handle = super::GameHandle("test_can_move_to_free_square".to_string());
        let game_options = super::GameOptions::default();
        let mut game = super::Game::new(game_handle, game_options);

        // Confirm green heister is where we expect it to be to begin with.
        let src_position = super::MapPosition { x: 2, y: 2 };
        let heister_color = super::HeisterColor::Green;
        assert_eq!(
            game.get_heister_from_vec(heister_color)
                .unwrap()
                .map_position
                .x,
            src_position.x
        );
        assert_eq!(
            game.get_heister_from_vec(heister_color)
                .unwrap()
                .map_position
                .y,
            src_position.y
        );

        let dest_position = super::MapPosition { x: 2, y: 1 };
        // defined here to avoid borrow issues if used later
        let expected_msg = format!(
            "Heister {:?} is on {:?}",
            HeisterColor::Orange,
            dest_position
        );
        let test_move = super::Move {
            heister_color,
            position: dest_position,
        };
        let message = MainMessage {
            body: Some(super::Body::Move(test_move.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Invalid(expected_msg));
        let curr_green_pos = game
            .get_heister_from_vec(super::HeisterColor::Green)
            .unwrap();
        assert_eq!(&curr_green_pos.map_position, &src_position);
    }

    #[test]
    pub fn grid_walls_align() -> () {
        let _ = env_logger::builder().is_test(true).try_init();
        let game_handle = GameHandle("test_grid_walls_align".to_string());
        let game_options = GameOptions::default();
        let game = super::Game::new(game_handle, game_options);
        let grid: HashMap<MapPosition, Square> = game.get_absolute_grid();

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
        let _ = env_logger::builder().is_test(true).try_init();
        let game_handle = GameHandle("test_grid_walls_align".to_string());
        let game_options = GameOptions::default();
        let mut game = super::Game::new(game_handle, game_options);
        // game setup done

        // setup to move orange to its orange door (one move from starting pos)
        let dest_position = super::MapPosition { x: 2, y: 0 };
        let test_move = super::Move {
            heister_color: HeisterColor::Orange,
            position: dest_position,
        };
        let message = MainMessage {
            body: Some(super::Body::Move(test_move.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Valid);
        let expected_orange_door_loc = super::MapPosition { x: 2, y: 0 };
        let actual_orange = game
            .get_heister_from_vec(super::HeisterColor::Orange)
            .unwrap();
        let actual_orange_loc = &actual_orange.map_position;
        assert_eq!(&expected_orange_door_loc, actual_orange_loc);
        // orange movement done

        // Let's create & execute the tile placement move
        let tile_entrance = super::MapPosition { x: 2, y: -1 };
        let test_tile_placement = super::PlaceTile { tile_entrance };
        let message = MainMessage {
            body: Some(super::Body::PlaceTile(test_tile_placement.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Valid);

        for tile in game.game_state.tiles {
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
    }
}
