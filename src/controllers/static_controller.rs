use iron::prelude::*;
use iron::status;
use hbs::handlebars::to_json;
use hbs::Template;

use templating;

pub struct StaticController;

impl StaticController {
    pub fn index(_: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data();
        data.insert("year".to_string(), to_json(&"2017".to_owned()));

        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
