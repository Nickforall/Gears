use iron::{Request, Response, IronResult, status, Set};
use iron::modifiers::Redirect;
use templating;
use hbs::Template;
use hbs::handlebars::to_json;
use models::project::Project;
use middleware::authentication::AuthenticatedUser;
use std::io::Read;
use std::collections::HashMap;
use helpers;
use routes;
use router::Router;
use models::user::User;
use helpers::error::ErrorBag;
use models::project::user_to_project::UserProject;

#[derive(Debug)]
pub struct ProjectData {
    name: String,
    description: String
}

impl ProjectData {
    pub fn validate(body: String) -> ErrorBag {
        let data = helpers::decode_body(body);
        let mut errors = ErrorBag::new();

        if data.get("name").unwrap().is_empty() {
            errors.add("A project Name Is Required");
        }

        if data.get("description").unwrap().is_empty() {
            errors.add("A project Description Is Required");
        }

        return errors
    }

    pub fn parse(body: String) -> Self {
        let data = helpers::decode_body(body);

        ProjectData {
            name: data.get("name").unwrap().clone(),
            description: data.get("description").unwrap().clone(),
        }
    }
}

pub struct ProjectController;

impl ProjectController {
    // Lists all projects
    pub fn list(req: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data(req);

        let projects = Project::all();
        data.insert("projects".to_owned(), to_json(&projects));

        let mut resp = Response::new();
        resp.set_mut(Template::new("projects", data)).set_mut(status::Ok);

        Ok(resp)
    }

    // Creates a new project
    pub fn new(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let errors = ProjectData::validate(body.clone());
        if errors.has_errors() {
            return Ok(Response::with((
                status::Found,
                Redirect(url_for!(req, "index", "errors" => errors.encode()))
            )));
        }

        let data = ProjectData::parse(body);

        Project::create(
            data.name.as_str(),
            data.description.as_str(),
            req.extensions.get::<AuthenticatedUser>().unwrap().id
        );

        Ok(Response::with((status::Found, Redirect(url_for!(req, "projects_ls")))))
    }

    /// Gets a specific project's details
    pub fn get(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        let mut data = templating::get_base_template_data(req);

        // get all users and their project permissions
        let users = User::all().unwrap();
        let mut project_user_perms = HashMap::new();
        // clone the users (because borrowing issues in template engine) and iterate
        for u in users.clone() {
            project_user_perms.insert(u.id, u.is_in_project(&project));
        }

        // insert data into map
        data.insert("project".to_owned(), to_json(&project));
        data.insert("users".to_owned(), to_json(&users));
        data.insert("user_project_data".to_owned(), to_json(&project_user_perms));

        // make response with the data
        let mut resp = Response::new();
        resp.set_mut(Template::new("projects/project", data)).set_mut(status::Ok);

        Ok(resp)
    }

    /// Edit a project's details
    pub fn edit(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        // check whether we can edit this project
        if req.extensions.get::<AuthenticatedUser>().unwrap().id != project.owner_id {
            return Ok(routes::notfound::get_404_response("404", req))
        }

        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let errors = ProjectData::validate(body.clone());
        if errors.has_errors() {
            return Ok(Response::with((
                status::Found,
                Redirect(url_for!(req, "projects_detail",
                    "errors" => errors.encode(),
                    "id" => project.id.to_string()
                ))
            )));
        }

        // parse and validate data
        let form = ProjectData::parse(body);

        // update
        project.update(
            form.name.as_str(),
            form.description.as_str(),
        );

        Ok(Response::with((status::Found, Redirect(url_for!(req, "projects_detail",
            "id" => project.id.to_string()
        )))))
    }

    /// Performs a search
    pub fn search(req: &mut Request) -> IronResult<Response> {
        // unwrap the query
        let q = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("").to_string();
        let mut data = templating::get_base_template_data(req);

        // make the search in the database
        let projects = Project::search_by_name(q.as_str()).unwrap();
        data.insert("projects".to_owned(), to_json(&projects));
        data.insert("query".to_owned(), to_json(&q));

        // make response with the data
        let mut resp = Response::new();
        resp.set_mut(Template::new("projects/search", data)).set_mut(status::Ok);

        Ok(resp)
    }

    /// Toggles a user
    pub fn toggle_user(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        // parse body and get id
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let data = helpers::decode_body(body);
        let id = data.get("userid").unwrap().parse::<i32>().unwrap();

        // check whether we can add/remove users
        if req.extensions.get::<AuthenticatedUser>().unwrap().id != project.owner_id {
            return Ok(routes::notfound::get_404_response("404", req))
        }

        // no "model not found" handling, because this is an underlying endpoint, it should 500 when
        // something goes wrong
        let _users_by_id = User::find_by_id(id).unwrap();
        // temporary value because the borrow checker doesn't like it when I get a value from
        // a vec in the same variable, ( the vec doesn't live long enough )
        let toggle_user = _users_by_id.first().unwrap();
        if toggle_user.is_in_project(&project) {
            UserProject::remove_user(&toggle_user, &project);
        } else {
            UserProject::add_user(&toggle_user, &project);
        }

        // redirect back to the project page
        Ok(Response::with((status::Found, Redirect(url_for!(req, "projects_detail",
            "id" => project.id.to_string()
        )))))
    }
}
