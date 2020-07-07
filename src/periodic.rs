use log::info;
use std::collections::HashMap;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::game::GameHandle;
use crate::manager::{GameManagerWrapper, GameWrapper, TEST_HANDLE};
use crate::utils::get_current_time_secs;

const REAP_DURATION: u64 = 3600; // 1hr from creation, games are reaped
const REAP_INTERVAL: u64 = 600; // 10m between reap calls

pub fn reaper(game_manager_wrapper: &GameManagerWrapper) {
    thread::sleep(Duration::from_secs(REAP_INTERVAL));
    let test_handle = GameHandle(TEST_HANDLE.to_string());

    let mut game_manager = game_manager_wrapper.game_manager.write().unwrap();

    let num_games = game_manager.games.len();
    let mut unreaped_games: HashMap<GameHandle, std::sync::Arc<RwLock<GameWrapper>>> =
        HashMap::new();
    let mut reaped_games: Vec<GameHandle> = Vec::new();
    for (handle, game_wrapper) in game_manager.games.iter() {
        let creation_time = game_wrapper.read().unwrap().get_creation_time();
        if handle == &test_handle || (creation_time + REAP_DURATION) > get_current_time_secs() {
            unreaped_games.insert(handle.clone(), game_wrapper.clone());
        } else {
            reaped_games.push(handle.clone());
        }
    }
    let num_reaped_games = num_games - unreaped_games.len();
    info!(
        "Reaper: num games: {}, num games reaped this cycle: {}, handles reaped: {:?}",
        num_games, num_reaped_games, reaped_games
    );

    game_manager.games = unreaped_games;
}
