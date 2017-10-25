use serde_json::value::{Map};
use serde_json::value::Value;
use hbs::handlebars::to_json;
use iron::Request;
use middleware;

pub mod helpers;

pub fn get_base_template_data(req: &mut Request) -> Map<String, Value> {
    let mut map = Map::new();

    let authenticated = req.extensions.get::<middleware::authentication::IsAuthenticated>().unwrap();
    map.insert("authenticated".to_owned(), to_json(&authenticated));
    if *authenticated {
        let user = req.extensions.get::<middleware::authentication::AuthenticatedUser>().unwrap();
        map.insert("user".to_owned(), to_json(user));
    }
    map.insert("parent".to_string(), to_json(&"template".to_owned()));

    map
}
