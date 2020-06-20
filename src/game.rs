use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::manager::{GameHandle, GameOptions};
use crate::types::main_message::Body;
use crate::types::{
    GameState, GameStatus, Heister, HeisterColor, Internal, MainMessage, MapPosition, Move,
    MoveDirection, Player, Square, WallType,
};

use log::{debug, info, trace};

#[derive(Debug)]
pub struct Game {
    pub game_handle: GameHandle,
    game_state: GameState,
}

#[derive(PartialEq, Debug)]
pub enum MoveValidity {
    Valid,
    Invalid(String),
}

impl Game {
    pub fn new(game_handle: GameHandle, _game_options: GameOptions) -> Game {
        let game_state = GameState::new(game_handle.clone());
        Game {
            game_handle,
            game_state,
        }
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
    fn get_heister_from_vec(&mut self, hc: HeisterColor) -> Option<&mut Heister> {
        for h in self.game_state.heisters.iter_mut() {
            if h.heister_color == hc {
                return Some(h);
            }
        }
        return None;
    }

    fn door_matches_heister(wall: WallType, color: &HeisterColor) -> MoveValidity {
        // Assumption: wall is one of the color-door types
        // Treating MoveValidity like a bool here, since it's the result type
        // I want anyways
        if (wall == WallType::PurpleDoor && color == &HeisterColor::Purple)
            || (wall == WallType::OrangeDoor && color == &HeisterColor::Orange)
            || (wall == WallType::YellowDoor && color == &HeisterColor::Yellow)
            || (wall == WallType::GreenDoor && color == &HeisterColor::Green)
        {
            return MoveValidity::Valid;
        } else {
            MoveValidity::Invalid("Can't move heister through wrong-colored door".to_string())
        }
    }

    fn process_move(&mut self, m: Move) -> MoveValidity {
        let grid = self.get_absolute_grid();
        // debug!("{:#?}", grid);

        let heister_color = m.heister_color;
        let heister = self.get_heister_from_vec(heister_color.clone()).unwrap();
        let heister_pos = &heister.map_position;
        debug!("Heister pos:");
        // debug!("{}", grid.get(&heister_pos).unwrap().pp());

        let dest_pos = m.position;
        debug!("Dest pos:");
        // debug!("{}", grid.get(&dest_pos).unwrap().pp());
        match grid.get(&dest_pos) {
            None => {
                return MoveValidity::Invalid(format!(
                    "Destination square {:?} doesn't exist",
                    dest_pos
                ))
            }
            Some(_wildcard) => (),
        };
        // OK - if the squares are adjacent, then we can assume they're trying to
        // move to an adjacent square, and can check for doors/walls
        if Self::are_adjacent(heister_pos, &dest_pos) {
            // Is the move valid for the wall between these two squares?
            // NOTE: I'm only going to check the wall of the source square -
            // edge cases where dest square wall may not match, but for now, don't are
            let heister_square = match grid.get(&heister_pos) {
                Some(s) => s,
                None => {
                    return MoveValidity::Invalid(format!(
                        "Heister square {:?} doesn't exist",
                        heister_pos
                    ))
                }
            };
            let move_dir = Self::adjacent_move_direction(&heister_pos, &dest_pos);
            info!("heister square: {:?}", heister_square);
            let blocking_wall = match move_dir {
                MoveDirection::North => heister_square.north_wall,
                MoveDirection::East => heister_square.east_wall,
                MoveDirection::South => heister_square.south_wall,
                MoveDirection::West => heister_square.west_wall,
            };
            let validity = match blocking_wall {
                WallType::Clear => MoveValidity::Valid,
                WallType::Impassable => {
                    MoveValidity::Invalid("Can't pass through impassable wall".to_string())
                }
                _wildcard => MoveValidity::Invalid(
                    "Moving to un-placed tile not implemented yet".to_string(),
                ),
            };
            // TODO - also check if there is another heister in the way

            if validity == MoveValidity::Valid {
                // move the heister
                heister.map_position = dest_pos;
            }
            return validity;
        } else {
            // If they're not adjacent, then we can check whether the destination is a
            // matching teleport, and whether teleportation is allowed right now
            return MoveValidity::Invalid("Teleports & Escalators not implemented yet".to_string());
        }
    }

    pub fn handle_message(&mut self, message: MainMessage) -> MoveValidity {
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        info!("Received message: {:?}", message);
        let body = message.body.unwrap();
        let validity = match body {
            Body::Move(m) => self.process_move(Move::from_proto(m)),
            Body::GameState(_gs) => {
                MoveValidity::Invalid("GameState is invalid from players".to_string())
            }
            Body::InvalidRequest(_ir) => {
                MoveValidity::Invalid("InvalidRequest is invalid from players".to_string())
            }
        };
        validity
    }
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
pub mod tests {
    use std::collections::HashMap;
    use log::{warn, info};
    use crate::manager::{GameHandle, GameOptions};
    use crate::types::{
        Internal, MainMessage, MapPosition, Move,
        MoveDirection, Player, Square, WallType,
    };

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
        assert_eq!(game.get_heister_from_vec(heister_color).unwrap().map_position.x, 1);
        assert_eq!(game.get_heister_from_vec(heister_color).unwrap().map_position.y, 1);

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
        let mut curr_yellow_pos = game.get_heister_from_vec(super::HeisterColor::Yellow).unwrap();
        assert_eq!(&curr_yellow_pos.map_position, &position);

        // THIS FOLLOWING PART *SHOULD* pass - but doesn't! Need unit tests on
        // tiles & tile loading - to ensure that tiles' walls are symmetric
        let next_position = super::MapPosition {
            x: 1, y: 1
        };
        let test_move = super::Move {
            heister_color,
            position: next_position.clone(),
        };
        let message = MainMessage {
            body: Some(super::Body::Move(test_move.to_proto())),
        };
        let validity = game.handle_message(message);
        assert_eq!(validity, super::MoveValidity::Valid);
        curr_yellow_pos = game.get_heister_from_vec(super::HeisterColor::Yellow).unwrap();
        assert_eq!(&curr_yellow_pos.map_position, &next_position);
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
                let index = MapPosition { x: mp.x - 1, y: mp.y};
                let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
                let left = grid.get(&index).expect(&msg);
                assert_eq!(square.west_wall, left.east_wall);
            }
            // Check right wall lines up.
            if mp.x < 3 {
                let index = MapPosition { x: mp.x + 1, y: mp.y};
                let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
                let right = grid.get(&index).expect(&msg);
                assert_eq!(square.east_wall, right.west_wall);
            }
            // Check top wall lines up.
            if mp.y > 0 {
                let index = MapPosition { x: mp.x, y: mp.y - 1};
                let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
                let above = grid.get(&index).expect(&msg);
                assert_eq!(square.north_wall, above.south_wall);
            }
            // Check bottom wall lines up.
            if mp.y < 3 {
                let index = MapPosition { x: mp.x, y: mp.y + 1};
                let msg = format!("Map tile {},{} not found", &mp.x, &mp.y);
                let below = grid.get(&index).expect(&msg);
                assert_eq!(square.south_wall, below.north_wall);
            }
        }
        info!("All walls line up");
    }
}
