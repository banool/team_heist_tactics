use crate::manager::{GameHandle, GameManagerWrapper, GameOptions};
use crate::errors::MyError;

use std::collections::HashSet;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

#[derive(Serialize)]
struct TemplateContext {
    handle: String,
}

// TODO Have a thing here that calls create_game()
pub async fn index() -> impl Responder {
    // TODO Make this function take in a reference to the server object
    // and use a word from the word list in that server object that it
    // loaded at startup.
    let context = TemplateContext {
        handle: "hey".to_string(),
    };
    // Template::render("index", &context)
    HttpResponse::Ok().body("Index!")
}

// TODO have a thing here that calls play()
pub async fn play() -> impl Responder {
    // TODO Make this function take in a reference to the server object
    // and use a word from the word list in that server object that it
    // loaded at startup.
    let context = TemplateContext {
        handle: "play".to_string(),
    };
    HttpResponse::Ok().body("Play!")
}

// TODO POST that redirects to play()
pub async fn create_game(game_manager_wrapper: web::Data<GameManagerWrapper>) -> impl Responder {
    let mut game_manager = game_manager_wrapper.game_manager.write().unwrap();

    // Register a new game.
    let game_options = GameOptions {};
    let game_handle = game_manager.new_game(game_options);
    let game_handle = match game_handle {
        Ok(game_handle) => game_handle,
        Err(e) => return HttpResponse::from_error(MyError::from(e).into()),
    };

    // Get the handle to the game and return a redirect to play/<that page>.
    // The frontend will use the last part of the URL to build the join_game request.
    let body = format!("play/{}", game_handle.0.to_string());

    HttpResponse::SeeOther().body(body)
}

// TODO This is the one that returns the websocket client connection
pub async fn join_game(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

/// Define http actor
pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
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
