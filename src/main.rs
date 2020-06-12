#[macro_use]
extern crate rocket;

// Generic imports.
use log::{error, info};
use std::env;

// Rocket imports.
use rocket_contrib::templates::Template;

// My imports.
use team_heist_tactics::manager::GameManager;
use team_heist_tactics::web;

const REQUIRED_ENV_VARS: &'static [&'static str] = &["TODO"];

fn validate_env() -> bool {
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            error!("This env var must be set: {}", s);
            return false;
        }
    }
    return true;
}

fn main() {
    env_logger::init();
    if !validate_env() {
        std::process::exit(69);
    }
    info!("All environment variables set");
    let game_manager = GameManager {};
    rocket::ignite()
        .manage(game_manager)
        .mount("/", routes![web::index])
        .mount("/play", routes![web::play])
        .attach(Template::fairing())
        .launch();
}
