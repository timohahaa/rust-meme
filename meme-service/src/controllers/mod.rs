use actix_web::web;

mod memes;

pub fn memes_service(c: &mut web::ServiceConfig) {
    c.route("/", web::get().to(memes::list));
    c.route("/{meme_id}", web::get().to(memes::get));
    c.route("/", web::post().to(memes::create));
    c.route("/{meme_id}", web::put().to(memes::update));
    c.route("/{meme_id}", web::delete().to(memes::delete));
}
