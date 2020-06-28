use actix_web;
use anyhow;
use std::fmt;

// The purpose of this error is to consume errors like the actix
// error or the askama error and convert them into anyhow errors.
// It can then convert back into the actix error.
// This allows us to use anyhow internally wherever possible.
#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl actix_web::error::ResponseError for MyError {}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}

impl From<actix_web::error::UrlGenerationError> for MyError {
    fn from(err: actix_web::error::UrlGenerationError) -> MyError {
        MyError {
            err: anyhow::anyhow!(format!("{:?}", err)),
        }
    }
}
