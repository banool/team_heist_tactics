use crate::game::Game;
use crate::errors::MyError;
use crate::manager::{GameHandle, GameManagerWrapper, GameOptions, JoinOptions};

use log::debug;
use std::collections::HashSet;

use actix::{Actor, StreamHandler};
use actix_web::{http::header, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use askama::Template;
use serde::Deserialize;
use std::sync::{Arc, RwLock};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    adjective: &'a str,
}

#[derive(Template)]
#[template(path = "play.html")]
struct PlayTemplate<'a> {
    adjective: &'a str,
}

pub async fn index() -> impl Responder {
    // TODO Use random adjective from the adjective list on GameManager.
    let index = IndexTemplate { adjective: "lit" };
    let body = match index.render() {
        Ok(body) => body,
        Err(e) => return HttpResponse::from_error(MyError::from(e).into()),
    };
    HttpResponse::Ok().body(body)
}

pub async fn play() -> impl Responder {
    let play = PlayTemplate { adjective: "lit" };
    let body = match play.render() {
        Ok(body) => body,
        Err(e) => return HttpResponse::from_error(MyError::from(e).into()),
    };
    HttpResponse::Ok().body(body)
}

pub async fn create_game(
    _req: HttpRequest,
    game_manager_wrapper: web::Data<GameManagerWrapper>,
) -> impl Responder {
    let mut game_manager = game_manager_wrapper.game_manager.write().unwrap();

    // Register a new game.
    let game_options = GameOptions {};
    let game_handle = game_manager.new_game(game_options);
    let game_handle = match game_handle {
        Ok(game_handle) => game_handle,
        Err(e) => return HttpResponse::from_error(MyError::from(e).into()),
    };

    // Get the handle to the game and return a redirect to play/handle=<that page>.
    // The frontend will use the last part of the URL to build the join_game request.
    // TODO Use proper params builder for this, like url_for.
    let location = format!("play?handle={}", game_handle.0.to_string());

    HttpResponse::SeeOther()
        .header(header::LOCATION, location)
        .finish()
}

#[derive(Deserialize)]
pub struct JoinGameQuery {
    name: String,
    handle: String,
}

// TODO Make the input here a struct and use whatever actix offers for this purpose.
// TODO This is the one that returns the websocket client connection
pub async fn play_game(
    req: HttpRequest,
    info: web::Query<JoinGameQuery>,
    stream: web::Payload,
    game_manager_wrapper: web::Data<GameManagerWrapper>,
) -> impl Responder {
    let mut game_manager = game_manager_wrapper.game_manager.write().unwrap();
    let join_options = JoinOptions {
        name: info.name.to_string(),
        handle: GameHandle(info.handle.to_string()),
        // websocket: my_ws.clone(),
    };
    let game = game_manager.join_game(join_options);
    let game = match game {
        Ok(game) => game,
        Err(e) => return HttpResponse::from_error(MyError::from(e).into()),
    };
    let my_ws = MyWs { game };

    let resp = ws::start(my_ws, &req, stream);
    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => return HttpResponse::from_error(e),
    };
    debug!("Successfully upgraded to websocket for {}", info.handle);
    resp
}

/// Define http actor
pub struct MyWs {
    game: Arc<RwLock<Game>>
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                debug!("Echoing ping with {:?}", msg);
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                debug!("Echoing text with {:?}", text);
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => {
                debug!("Echoing binary with {:?}", bin);
                ctx.binary(bin)
            },
            _ => (),
        }
        // call the method on game. if it was a valid move, put an update back in to the channel
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
