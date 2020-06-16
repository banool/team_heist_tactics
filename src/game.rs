use anyhow::{anyhow, Result};

use crate::manager::{GameHandle, GameOptions};
use crate::types::{GameState, GameStatus, MainMessage, Player, Move, Internal, Square, MapPosition};
use crate::types::main_message::Body;
use std::collections::HashMap;

use log::info;

#[derive(Debug)]
pub struct Game {
    pub game_handle: GameHandle,
    game_state: GameState,
}

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
            for (i,square) in tile.squares.iter().enumerate() {
                let sq_x = (i / 4) as i32;
                let sq_y = (sq_x % 4) as i32;
                let grid_x = tile_pos.x + sq_x;
                let grid_y = tile_pos.y + sq_y;
                info!("{}: {:?} {:?} {:?} {:?}, {:?}", i, square.north_wall, square.west_wall, square.south_wall, square.east_wall, square.square_type);
                let mp = MapPosition {
                    x: grid_x,
                    y: grid_y,
                };
                grid.insert(mp, square.clone());
            }
        }
        info!("{:#?}", grid);
        grid
    }

    pub fn process_move(&self, m: Move) -> MoveValidity {
        let heister = m.heister_color;
        let pos = m.position;
        let grid = self.get_absolute_grid();
        let my_square = match grid.get(&pos) {
            Some(my_square) => my_square,
            None => return MoveValidity::Invalid("Square {:?} doesn't exist".to_string()),
        };
        MoveValidity::Valid
    }

    pub fn handle_message(&mut self, message: MainMessage) -> MoveValidity {
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        info!("Received message: {:?}", message);
        let body = message.body.unwrap();
        let validity = match body {
            Body::Move(m) => self.process_move(Move::from_proto(m)),
            Body::GameState(_gs) => MoveValidity::Invalid("GameState is invalid from players".to_string()),
            Body::InvalidRequest(_ir) => MoveValidity::Invalid("InvalidRequest is invalid from players".to_string()),
        };
        validity
    }
}
