// diesel needs this, because the macros of model generation got a bit too big
#![recursion_limit="128"]

extern crate iron;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate serde_json;
extern crate mount;
extern crate staticfile;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate params;
#[macro_use] extern crate router;
extern crate serde_urlencoded;
#[macro_use] extern crate serde_derive;
extern crate pwhash;
extern crate chrono;
extern crate base64;
extern crate iron_csrf;
extern crate secure_session;
extern crate typemap;
extern crate rand;

mod routes;
mod controllers;
mod templating;
mod models;
mod middleware;
mod helpers;
mod macros;

use mount::Mount;
use iron::prelude::*;
use std::path::Path;
use staticfile::Static;
use dotenv::dotenv;
use std::env;
use iron::AroundMiddleware;

fn init() -> Chain {
    dotenv().ok();
    let cryptokey = env::var("CRYPTO_KEY").expect("CRYPTO_KEY must be set");

    let mut router_mount = Mount::new();
    // Mount the static folder on the static route
    router_mount
        .mount("/", routes::all())
        .mount("/static/", Static::new(Path::new("src/static")));

    // initialize the chain
    let mut chain = Chain::new(router_mount);

    let sessions = middleware::sessions::get_session_middleware(cryptokey.clone());
    let csrf = middleware::csrf::get_csrf_middleware(cryptokey.clone());
    chain.link_around(middleware::authentication::AuthMiddleware);

    let handler = sessions.around(Box::new(chain));
    let mut chain = Chain::new(handler);

    // 404 middleware that serves a 404 on non-existent routes
    chain.link_after(routes::get_404_handler("404"));
    // load the templates if we're not using the watch module
    #[cfg(not(feature = "watch"))]
    chain.link_after(routes::templates());


    chain
}

#[cfg(feature = "watch")]
fn main() {
    use std::sync::Arc;
    use hbs::Watchable;

    // Initialize the chain
    let mut chain = init();

    // Link the watcher for development purposes
    let hbse_ref = Arc::new(routes::templates());
    hbse_ref.watch("./src/views/");
    chain.link_after(hbse_ref);

    // Tell on which address we're running
    println!("Running your server on \"{}\" <3", ADDRESS);

    // Start the iron loop
    Iron::new(chain).http(ADDRESS).unwrap();
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("WARNING, you are running in debugging mode without the watcher!!!");
    // Initialize the chain
    let chain = init();

    // Tell on which address server we're running
    println!("Running your server on \"{}\" <3", ADDRESS);

    // Start the iron loop
    Iron::new(chain).http(ADDRESS).unwrap();
}

const ADDRESS: &'static str = "localhost:3000";
