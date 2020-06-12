use crate::types::GameState;
use crate::manager::{GameHandle, GameOptions};

pub struct Game {
    pub game_handle: GameHandle,
    game_state: GameState,
}

impl Game {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> Game {
        // TODO Make starting game state.
        let game_state = GameState::default();
        Game { game_handle, game_state }
    }
}
