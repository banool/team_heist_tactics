// Generic imports.
use anyhow::{Error, Result};
use log::{error, info};
use std::env;
use std::sync::Arc;

// Other crate imports.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// My imports.
use team_heist_tactics::manager::GameManager;
use team_heist_tactics::endpoints;

const REQUIRED_ENV_VARS: &'static [&'static str] = &["THT_IP_ADDRESS", "THT_PORT"];

fn validate_env() -> bool {
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            error!("This env var must be set: {}", s);
            return false;
        }
    }
    return true;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if !validate_env() {
        std::process::exit(69);
    }
    info!("All environment variables set");

    let game_manager = GameManager {};
    let game_manager = web::Data::new(game_manager);
    let ip = env::var("THT_IP_ADDRESS").unwrap();
    let port = env::var("THT_PORT").unwrap();
    let ip_port = format!("{}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(game_manager.clone())
            .route("/", web::get().to(endpoints::index))
            .route("/play", web::get().to(endpoints::play))
    })
    .bind(ip_port)?
    .run()
    .await
}
