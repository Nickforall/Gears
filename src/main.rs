extern crate iron;
extern crate handlebars_iron as hbs;
extern crate serde_json;
extern crate mount;
extern crate staticfile;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate lazy_static;

mod routes;
mod controllers;
mod templating;
mod models;

use mount::Mount;
use iron::prelude::*;
use std::path::Path;
use staticfile::Static;

fn init() -> Chain {
    let mut router_mount = Mount::new();
    router_mount
        .mount("/", routes::all())
        .mount("/static/", Static::new(Path::new("src/static")));

    let mut chain = Chain::new(router_mount);

    chain.link_after(routes::get_404_handler("404"));

    #[cfg(not(feature = "watch"))]
    chain.link_after(routes::templates());

    chain
}

#[cfg(feature = "watch")]
fn main() {
    use std::sync::Arc;
    use hbs::Watchable;

    let mut chain = init();

    // Link the watcher
    let hbse_ref = Arc::new(routes::templates());
    hbse_ref.watch("./src/views/");

    chain.link_after(hbse_ref);

    println!("Running your server on \"{}\" <3", ADDRESS);

    Iron::new(chain).http(ADDRESS).unwrap();
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("WARNING, you are running in debugging mode without the watcher!!!");
    let chain = init();

    println!("Running your server on \"{}\" <3", ADDRESS);

    Iron::new(chain).http(ADDRESS).unwrap();
}

const ADDRESS: &'static str = "localhost:3000";
