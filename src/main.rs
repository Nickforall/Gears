extern crate iron;
extern crate handlebars_iron as hbs;
extern crate serde_json;
extern crate mount;
extern crate staticfile;

mod routes;
mod controllers;

use mount::Mount;
use iron::prelude::*;
use std::path::Path;
use staticfile::Static;
use std::sync::Arc;
use hbs::Watchable;

#[cfg(feature = "watch")]
fn main() {
    let address = "localhost:3000";

    let mut router_mount = Mount::new();

    router_mount
        .mount("/", routes::all())
        .mount("/static/", Static::new(Path::new("src/static")));

    let mut chain = Chain::new(router_mount);

    let hbse_ref = Arc::new(routes::templates());
    hbse_ref.watch("./src/views/");

    chain.link_after(hbse_ref);

    println!("Running your server on \"{}\" <3", address);

    Iron::new(chain).http(address).unwrap();
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("WARNING, you are running in debugging mode without the watcher!!!");
    let address = "localhost:3000";

    let mut router_mount = Mount::new();

    router_mount
        .mount("/", routes::all())
        .mount("/static/", Static::new(Path::new("src/static")));

    let mut chain = Chain::new(router_mount);
    chain.link_after(routes::templates());

    println!("Running your server on \"{}\" <3", address);

    Iron::new(chain).http(address).unwrap();
}
