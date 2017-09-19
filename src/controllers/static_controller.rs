use iron::prelude::*;
use iron::status;
use hbs::Template;

use templating;

pub struct StaticController;

impl StaticController {
    pub fn index(_: &mut Request) -> IronResult<Response> {
        let data = templating::get_base_template_data();

        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
