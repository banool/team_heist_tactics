// Manages all the games.

use crate::endpoints::MyWs;
use crate::game::Game;

use anyhow::{anyhow, Result};
use log::info;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct GameHandle(pub String);

pub struct GameOptions {}

pub struct JoinOptions {
    pub name: String,
    pub handle: GameHandle,
    //websocket: Arc<MyWs>,
}

pub struct GameWrapper {
    game: Game,
    websockets: Vec<Arc<MyWs>>,
}

impl GameWrapper {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> GameWrapper {
        let game = Game::new(game_handle, game_options);
        GameWrapper {
            game,
            websockets: vec![],
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        // add_websocket
        self.game.add_player(name)
    }

    pub fn add_websocket(&mut self, my_ws: Arc<MyWs>) {
        self.websockets.push(my_ws);
    }

    // TODO a function that pushes the game state to all websockets
}

pub struct GameManagerWrapper {
    pub game_manager: RwLock<GameManager>,
}

pub struct GameManager {
    pub games: HashMap<GameHandle, Arc<RwLock<GameWrapper>>>,
    pub words: HashSet<String>,
}

impl GameManager {
    pub fn new(games: HashMap<GameHandle, Arc<RwLock<GameWrapper>>>, words: HashSet<String>) -> Self {
        GameManager { games, words }
    }

    // TODO Just use GameHandle everywhere, including in the possible handle list.
    fn get_in_use_handles(&self) -> HashSet<String> {
        self.games.keys().map(|gh| gh.0.to_string()).collect()
    }

    pub fn new_game(&mut self, game_options: GameOptions) -> Result<GameHandle> {
        let in_use_handles: HashSet<String> = self.get_in_use_handles();
        let mut available_handles = self.words.difference(&in_use_handles);
        let handle = match available_handles.next() {
            Some(handle) => handle,
            None => return Err(anyhow!("Ran out of game handles")),
        };
        let game_handle = GameHandle(handle.to_string());

        let game_wrapper = Arc::new(RwLock::new(GameWrapper::new(game_handle.clone(), game_options)));

        self.games.insert(game_handle.clone(), game_wrapper);

        info!("Created game: {}", game_handle.0.to_string());

        Ok(game_handle)
    }

    pub fn join_game(&mut self, join_options: JoinOptions) -> Result<Arc<RwLock<GameWrapper>>> {
        let game_wrapper = match self.games.get_mut(&join_options.handle) {
            Some(game_wrapper) => game_wrapper,
            None => return Err(anyhow!(format!("Game with handle \"{}\" does not exist", join_options.handle.0))),
        };

        game_wrapper.write().unwrap().add_player(join_options.name.to_string())?;

        info!("Player {} joined game {}", join_options.name.to_string(), join_options.handle.0);

        Ok(game_wrapper.clone())
    }

    pub fn register_websocket(&mut self, game_handle: GameHandle, websocket: Arc<MyWs>) {
        let game_wrapper = match self.games.get_mut(&game_handle) {
            Some(game_wrapper) => game_wrapper,
            None => panic!("Game we just made doesn't exist"),
        };
        game_wrapper.write().unwrap().add_websocket(websocket);
    }
}
