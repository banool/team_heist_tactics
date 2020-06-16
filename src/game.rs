use anyhow::{anyhow, Result};

use crate::manager::{GameHandle, GameOptions};
use crate::types::{GameState, GameStatus, MainMessage, Player, Move, MapPosition, InvalidRequest};
use crate::types::main_message::Body;
use crate::types::Internal;
use crate::types;

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

    pub fn process_move(&mut self, m: Move) -> MoveValidity {
        info!("{:#?}", m);
        // let m = b as Move;
        // let hc = m.heister_color;
        // let mpos = m.position;
        // info!("heister color: {:?}, map pos: {:?}", hc, mpos);

        MoveValidity::Valid
    }

    pub fn handle_message(&mut self, message: MainMessage) -> MoveValidity {
        // TODO Match on main.body and influence the game state for each of the options.
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        info!("Received message: {:#?}", message);
        let body = message.body.unwrap();
        let mv = match body {
            Body::Move ( m ) => self.process_move(Move::from_proto(m)),
            Body::GameState ( gs ) => MoveValidity::Invalid("invalid".to_string()),
            Body::InvalidRequest ( ir ) => MoveValidity::Invalid("invalid".to_string()),
        };
        mv
    }
}
