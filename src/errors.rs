use std::fmt;
use anyhow;
use actix_web;

#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl actix_web::error::ResponseError for MyError { }

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}
