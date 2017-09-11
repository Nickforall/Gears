use serde_json::value::{Map};
use serde_json::value::Value;
use hbs::handlebars::to_json;

pub fn get_base_template_data() -> Map<String, Value> {
    let mut map = Map::new();

    map.insert("parent".to_string(), to_json(&"template".to_owned()));

    map
}
