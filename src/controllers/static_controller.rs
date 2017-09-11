use iron::prelude::*;
use iron::status;
use hbs::handlebars::to_json;
use hbs::Template;
use iron_sessionstorage::SessionRequestExt;

use templating;
use middleware;

pub struct StaticController;

impl StaticController {
    pub fn index(req: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data();

        if try!(req.session().get::<middleware::sessions::Login>()).is_some() {
            // Already logged in
            data.insert("year".to_string(), to_json(&"2018".to_owned()));
        } else {
            data.insert("year".to_string(), to_json(&"2017".to_owned()));
        }

        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
