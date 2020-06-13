use anyhow::{anyhow, Result};

use crate::manager::{GameHandle, GameOptions};
use crate::types::{GameStatus, GameState};

pub struct Player {
    name: String,
}

pub struct Game {
    pub game_handle: GameHandle,
    game_state: GameState,
    players: Vec<Player>,
}

impl Game {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> Game {
        // TODO Make starting game state.
        let game_state = GameState::default();
        Game {
            game_handle,
            game_state,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        if GameStatus::from_i32(self.game_state.game_status) != Some(GameStatus::Staging) {
            return Err(anyhow!("Cannot join game that is already in progress"));
        }
        self.players.push(Player {name});
        Ok(())
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state.clone()
    }
}
