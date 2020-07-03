// Manages all the games.

use crate::endpoints::MyWs;
use crate::game::{Game, MoveValidity};
use crate::serializer::InternalMessage;
use crate::types::main_message::Body;
use crate::types::{MainMessage, PlayerName};

use actix::Addr;
use anyhow::{anyhow, Result};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GameHandle(pub String);

#[derive(Default)]
pub struct GameOptions {}

pub struct JoinOptions {
    pub name: String,
    pub handle: GameHandle,
}

#[derive(Debug)]
pub struct GameWrapper {
    game: Game,
    actors: Vec<Addr<MyWs>>,
}

// TODO Add the ability to pause and resume the game.
// TODO If any player disconnects, pause the game.
impl GameWrapper {
    pub fn new(game_handle: GameHandle, game_options: GameOptions) -> GameWrapper {
        let game = Game::new(game_handle, game_options);
        GameWrapper {
            game,
            actors: vec![],
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<()> {
        self.game.add_player(name)
    }

    pub fn add_actor(&mut self, actor: Addr<MyWs>) {
        self.drop_dead_actors();
        self.actors.push(actor);
    }

    pub fn drop_dead_actors(&mut self) {
        for a in self.actors.iter() {
            if !a.connected() {
                warn!(
                    "Dropping dead actor from {}: {:?}",
                    self.game.game_handle.0, a
                );
            }
        }
        self.actors.retain(|a| a.connected());
    }

    pub fn push_state(&self) -> Result<()> {
        let game_state = self.game.get_game_state();
        let internal_message = InternalMessage::from_game_state(game_state);
        for a in self.actors.iter() {
            // TODO Consider using send instead.
            a.do_send(internal_message.clone());
        }
        Ok(())
    }

    pub fn push_chat(&self, chat: String) -> Result<()> {
        let internal_message = InternalMessage::from_chat(chat);
        for a in self.actors.iter() {
            // TODO Consider using send instead.
            a.do_send(internal_message.clone());
        }
        Ok(())
    }

    pub fn handle_message(
        &mut self,
        message: MainMessage,
        player_name: &PlayerName,
    ) -> MoveValidity {
        if let Body::Chat(c) = message.clone().body.unwrap() {
            self.push_chat(c).unwrap();
            return MoveValidity::Valid;
        }
        self.game.handle_message(message, &player_name)
    }
}

pub struct GameManagerWrapper {
    pub game_manager: RwLock<GameManager>,
}

pub struct GameManager {
    pub games: HashMap<GameHandle, Arc<RwLock<GameWrapper>>>,
    pub words: HashSet<String>,
}

impl GameManager {
    pub fn new(
        games: HashMap<GameHandle, Arc<RwLock<GameWrapper>>>,
        words: HashSet<String>,
    ) -> Self {
        GameManager { games, words }
    }

    // TODO Just use GameHandle everywhere, including in the possible handle list.
    fn get_in_use_handles(&self) -> HashSet<String> {
        self.games.keys().map(|gh| gh.0.to_string()).collect()
    }

    pub fn new_game(
        &mut self,
        game_options: GameOptions,
        handle: Option<String>,
    ) -> Result<GameHandle> {
        let handle = match handle {
            Some(handle) => handle,
            None => {
                let in_use_handles: HashSet<String> = self.get_in_use_handles();
                let mut available_handles = self.words.difference(&in_use_handles);
                let handle = match available_handles.next() {
                    Some(handle) => handle,
                    None => return Err(anyhow!("Ran out of game handles")),
                };
                handle.to_string()
            }
        };
        let game_handle = GameHandle(handle.to_string());

        let game_wrapper = Arc::new(RwLock::new(GameWrapper::new(
            game_handle.clone(),
            game_options,
        )));

        self.games.insert(game_handle.clone(), game_wrapper);

        info!("Created game: {}", game_handle.0.to_string());

        Ok(game_handle)
    }

    pub fn join_game(&mut self, join_options: JoinOptions) -> Result<Arc<RwLock<GameWrapper>>> {
        let game_wrapper = match self.games.get_mut(&join_options.handle) {
            Some(game_wrapper) => game_wrapper,
            None => {
                return Err(anyhow!(format!(
                    "Game with handle \"{}\" does not exist",
                    join_options.handle.0
                )))
            }
        };

        {
            let mut game_wrapper = game_wrapper.write().unwrap();
            let player_name = join_options.name.to_string();
            let player_already_in = game_wrapper.game.has_player(&player_name);
            let join_prefix_str;
            if player_already_in {
                join_prefix_str = "RE-";
            } else {
                join_prefix_str = "";
                game_wrapper.add_player(player_name)?;
            }
            info!(
                "Player {} {}joined game {}",
                join_options.name.to_string(),
                join_prefix_str,
                join_options.handle.0
            );
        }

        Ok(game_wrapper.clone())
    }

    pub fn register_actor(&mut self, game_handle: GameHandle, actor: Addr<MyWs>) {
        let game_wrapper = match self.games.get_mut(&game_handle) {
            Some(game_wrapper) => game_wrapper,
            None => panic!("Game we just made doesn't exist"),
        };
        let mut game_wrapper = game_wrapper.write().unwrap();
        game_wrapper.add_actor(actor);
        // Push initial state / update other clients that there is a new player.
        match game_wrapper.push_state() {
            Ok(_) => (),
            Err(e) => error!("Failed to push state for {}: {:?}", game_handle.0, e),
        }
    }
}
