use anyhow::{anyhow, Result};

use crate::manager::{GameHandle, GameOptions};
use crate::types::{GameState, GameStatus, MainMessage};
use crate::utils::get_current_time_secs;

pub struct Player {
    name: String,
}

pub struct Game {
    pub game_handle: GameHandle,
    game_state: GameState,
    players: Vec<Player>,
}

pub enum MoveValidity {
    Valid,
    Invalid(String),
}

impl Game {
    pub fn new(game_handle: GameHandle, _game_options: GameOptions) -> Game {
        // TODO Make starting game state.
        let mut game_state = GameState::default();
        game_state.game_name = game_handle.0.to_string();
        game_state.game_started = get_current_time_secs();
        Game {
            game_handle,
            game_state,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        if self.game_state.game_status != GameStatus::Staging {
            return Err(anyhow!("Cannot join game that is already in progress"));
        }
        self.players.push(Player { name });
        Ok(())
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state.clone()
    }

    pub fn handle_message(&mut self, _main: MainMessage) -> MoveValidity {
        // TODO Match on main.body and influence the game state for each of the options.
        // If we receive GameState or InvalidRequest at this endpoint, panic, it should never happen.
        MoveValidity::Valid
    }
}
