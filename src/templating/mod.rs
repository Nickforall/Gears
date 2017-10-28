use serde_json::value::{Map};
use serde_json::value::Value;
use hbs::handlebars::to_json;
use iron::{Request};
use iron::prelude::*;
use middleware;
use helpers::error::ErrorBag;
use params;
use params::Params;

pub mod helpers;

pub fn get_base_template_data(req: &mut Request) -> Map<String, Value> {
    let mut map = Map::new();

    let has_errors: bool;
    let mut errors: Vec<String> = Vec::new();

    match req.get_ref::<Params>().unwrap().get("errors") {
        Some(&params::Value::String(ref b64)) => {
            has_errors = true;
            errors = ErrorBag::decode(b64).as_vec();
        },
        _ => has_errors = false,
    }

    let authenticated = req.extensions.get::<middleware::authentication::IsAuthenticated>().unwrap();
    map.insert("authenticated".to_owned(), to_json(&authenticated));
    if *authenticated {
        let user = req.extensions.get::<middleware::authentication::AuthenticatedUser>().unwrap();
        map.insert("user".to_owned(), to_json(user));
    }
    map.insert("has_errors".to_string(), to_json(&has_errors));
    map.insert("errors".to_string(), to_json(&errors));
    map.insert("parent".to_string(), to_json(&"template".to_owned()));

    map
}
