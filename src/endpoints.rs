use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize)]
struct TemplateContext {
    handle: String,
}

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

pub async fn play() -> impl Responder {
    // TODO Make this function take in a reference to the server object
    // and use a word from the word list in that server object that it
    // loaded at startup.
    let context = TemplateContext {
        handle: "play".to_string(),
    };
    HttpResponse::Ok().body("Play!")
}
