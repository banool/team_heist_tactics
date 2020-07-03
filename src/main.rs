// Generic imports.
use log::{error, info};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::RwLock;

use std::thread;
use std::time::Duration;
use team_heist_tactics::utils::get_current_time_secs;

// Other crate imports.
use actix_files as fs;
use actix_web::{web, App, HttpServer};

// My imports.
use team_heist_tactics::endpoints;
use team_heist_tactics::game::GameOptions;
use team_heist_tactics::manager::{GameManager, GameManagerWrapper};

const REAP_DURATION: u64 = 3600; // 1hr from creation, games are reaped
const REAP_INTERVAL: u64 = 600; // 10m between reap calls

const REQUIRED_ENV_VARS: &'static [&'static str] = &[
    "THT_IP_ADDRESS",
    "THT_PORT",
    "THT_DEPLOYMENT_MODE",
    "HANDLES_FILE",
];

fn validate_env() -> bool {
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            error!("This env var must be set: {}", s);
            return false;
        }
    }
    return true;
}

#[derive(Clone, Debug, PartialEq)]
enum DeploymentMode {
    Dev,
    Prod,
}

impl FromStr for DeploymentMode {
    type Err = ();

    fn from_str(input: &str) -> Result<DeploymentMode, Self::Err> {
        match input {
            "dev" => Ok(DeploymentMode::Dev),
            "prod" => Ok(DeploymentMode::Prod),
            _ => Err(()),
        }
    }
}

fn get_possible_handles() -> HashSet<String> {
    let filename = env::var("HANDLES_FILE").unwrap();
    let file = File::open(filename).expect("Could not read handles file");
    let buf_reader = BufReader::new(file);
    let mut words = HashSet::new();
    for line in buf_reader.lines() {
        let w = line.unwrap();
        let w = w.replace(" ", "");
        let w = w.replace("-", "");
        words.insert(w);
    }
    words
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if !validate_env() {
        std::process::exit(69);
    }
    info!("All environment variables set");

    let possible_handles = get_possible_handles();

    let games = HashMap::new();
    let game_manager = GameManager::new(games, possible_handles);
    let game_manager = RwLock::new(game_manager);
    let game_manager_wrapper = GameManagerWrapper { game_manager };
    let game_manager_wrapper = web::Data::new(game_manager_wrapper);

    let ip = env::var("THT_IP_ADDRESS").unwrap();
    let port = env::var("THT_PORT").unwrap();
    let ip_port = format!("{}:{}", ip, port);
    let deployment_mode = DeploymentMode::from_str(&env::var("THT_DEPLOYMENT_MODE").unwrap())
        .expect("Invalid deployment mode");

    // For testing.
    game_manager_wrapper
        .game_manager
        .write()
        .unwrap()
        .new_game(
            GameOptions {
                shuffle_tiles: false,
            },
            Some("test".to_string()),
        )
        .unwrap();

    let reaper_manager_ref = game_manager_wrapper.clone();

    // Game Reaper thread
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(REAP_INTERVAL));
        let mut game_manager_mut = reaper_manager_ref.game_manager.write().unwrap();
        let games = game_manager_mut.games.clone();
        let mut unreaped_games: HashMap<GameHandle, std::sync::Arc<RwLock<GameWrapper>>> =
            HashMap::new();
        for (handle, game_wrapper) in games {
            let creation_time = game_wrapper.read().unwrap().game.game_created;
            if creation_time + REAP_DURATION < get_current_time_secs() {
                unreaped_games.insert(handle, game_wrapper);
            }
        }
        let num_games = reaper_manager_ref.game_manager.read().unwrap().games.len();
        let num_reaped_games = num_games - unreaped_games.len();
        println!(
            "Reaper: time {}, num games: {:?}, num games reaped: {:?}",
            get_current_time_secs(),
            num_games,
            num_reaped_games
        );
        game_manager_mut.games = unreaped_games;
    });

    HttpServer::new(move || {
        let app = App::new()
            .app_data(game_manager_wrapper.clone())
            .route("/", web::get().to(endpoints::index))
            .route("/play", web::get().to(endpoints::play))
            .route("/create_game", web::post().to(endpoints::create_game))
            .route("/play_game", web::get().to(endpoints::play_game));
        let app = match deployment_mode {
            DeploymentMode::Dev => app.service(fs::Files::new("/static", "static")),
            _ => app,
        };
        app
    })
    .bind(ip_port)?
    .run()
    .await
}
