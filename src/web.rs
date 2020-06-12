use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    handle: String,
}

#[get("/")]
pub fn index() -> Template {
    // TODO Make this function take in a reference to the server object
    // and use a word from the word list in that server object that it
    // loaded at startup.
    let context = TemplateContext {
        handle: "hey".to_string(),
    };
    Template::render("index", &context)
}

#[get("/play")]
pub fn play() -> Template {
    // TODO Make this function take in a reference to the server object
    // and use a word from the word list in that server object that it
    // loaded at startup.
    let context = TemplateContext {
        handle: "play".to_string(),
    };
    Template::render("play", &context)
}
