#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;

pub mod endpoints;
pub mod errors;
pub mod game;
pub mod manager;
pub mod types;
