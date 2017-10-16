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
use router::Router;
use routes;

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
                data.description.as_str(),
                req.extensions.get::<AuthenticatedUser>().unwrap().id
            );
        }

        Ok(Response::with((status::Found, Redirect(url_for!(req, "projects_ls")))))
    }

    pub fn get(req: &mut Request) -> IronResult<Response> {
        let id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("0").to_string();
        let project;

        // check whether the id is a number
        match id.parse::<i32>() {
            Ok(numeric_id) => {
                // if it is, throw it into our find or fail
                match Project::find_or_fail(numeric_id) {
                    Some(p) => {
                        project = p;
                    },
                    None => return Ok(routes::notfound::get_404_response("404", req))
                };
            },
            Err(_) => return Ok(routes::notfound::get_404_response("404", req))
        };

        let mut data = templating::get_base_template_data(req);
        data.insert("project".to_owned(), to_json(&project));

        let mut resp = Response::new();
        resp.set_mut(Template::new("projects/project", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
