extern crate iron;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate serde_json;
extern crate mount;
extern crate staticfile;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate iron_sessionstorage;
extern crate params;
#[macro_use] extern crate router;
extern crate serde_urlencoded;
#[macro_use] extern crate serde_derive;

mod routes;
mod controllers;
mod templating;
mod models;
mod middleware;
mod helpers;

use mount::Mount;
use iron::prelude::*;
use std::path::Path;
use staticfile::Static;

fn init() -> Chain {
    let mut router_mount = Mount::new();
    // Mount the static folder on the static route
    router_mount
        .mount("/", routes::all())
        .mount("/static/", Static::new(Path::new("src/static")));

    // initialize the chain
    let mut chain = Chain::new(router_mount);
    let session_middleware = middleware::sessions::get_session_middleware("deadbeef");

    // Authentication middleware
    chain.link_around(middleware::authentication::AuthMiddleware);
    // Session middleware that handles a key-value like session storage
    chain.link_around(session_middleware);


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
