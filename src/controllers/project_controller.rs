use iron::{Request, Response, IronResult, status, Set};
use iron::modifiers::Redirect;
use templating;
use hbs::Template;
use hbs::handlebars::to_json;
use models::project::Project;
use middleware::authentication::AuthenticatedUser;
use middleware::authentication::IsAuthenticated;
use std::io::Read;
use helpers;

#[derive(Debug)]
pub struct NewProjectData {
    name: String,
    description: String
}

impl NewProjectData {
    pub fn parse(req: &mut Request) -> Self {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let data = helpers::decode_body(body);

        NewProjectData {
            name: data.get("description").unwrap().clone(),
            description: data.get("name").unwrap().clone(),
        }
    }
}


pub struct ProjectController;

impl ProjectController {
    pub fn list(req: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data(req);

        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            let projects = Project::find_all_by_owner(
                req.extensions.get::<AuthenticatedUser>().unwrap().id).unwrap();
            data.insert("projects".to_owned(), to_json(&projects));
        }

        let mut resp = Response::new();
        resp.set_mut(Template::new("projects", data)).set_mut(status::Ok);

        Ok(resp)
    }

    pub fn new(req: &mut Request) -> IronResult<Response> {
        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            let data = NewProjectData::parse(req);
            Project::create(
                data.name.as_str(),
                data.name.as_str(),
                req.extensions.get::<AuthenticatedUser>().unwrap().id
            );
        }

        Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
    }
}
