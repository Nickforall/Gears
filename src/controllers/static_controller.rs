use iron::prelude::*;
use iron::status;
use hbs::Template;

use templating;
use middleware::authentication::AuthenticatedUser;
use middleware::authentication::IsAuthenticated;
use models::project::Project;
use hbs::handlebars::to_json;

pub struct StaticController;

impl StaticController {
    pub fn index(req: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data(req);

        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            let projects = Project::find_all_by_owner(
                req.extensions.get::<AuthenticatedUser>().unwrap().id).unwrap();
            data.insert("projects".to_owned(), to_json(&projects));
        }

        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
