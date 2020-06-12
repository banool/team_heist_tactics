// Manages all the games.

use crate::game::Game;
use crate::endpoints::MyWs;

use anyhow::{anyhow, Result};
use log::info;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct GameHandle(pub String);

pub struct GameOptions {

}

pub struct GameWrapper {
    game: Game,
    websockets: Vec<MyWs>,
}

impl GameWrapper {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> GameWrapper {
        let game = Game::new(game_handle, game_options);
        GameWrapper { game, websockets: vec![] }
    }
}

pub struct GameManagerWrapper {
    pub game_manager: RwLock<GameManager>,
}

pub struct GameManager {
    pub games: HashMap<GameHandle, GameWrapper>,
    pub words: HashSet<String>,
}

impl GameManager {
    pub fn new(games: HashMap<GameHandle, GameWrapper>, words: HashSet<String>) -> Self {
        GameManager { games, words }
    }

    pub fn new_game(&mut self, game_options: GameOptions) -> Result<GameHandle> {
        let in_use_handles: HashSet<String> = self.games.keys().map(|gh| gh.0.to_string()).collect();
        let mut available_handles = self.words.difference(&in_use_handles);
        let handle = match available_handles.next() {
            Some(handle) => handle,
            None => return Err(anyhow!("Ran out of game handles")),
        };
        let game_handle = GameHandle(handle.to_string());

        let game_wrapper = GameWrapper::new(game_handle.clone(), game_options);

        self.games.insert(game_handle.clone(), game_wrapper);

        info!("Created game: {}", game_handle.0.to_string());

        Ok(game_handle)
    }
}
