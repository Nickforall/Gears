use iron::prelude::*;
use iron::status;
use hbs::Template;

use templating;
use middleware::authentication::AuthenticatedUser;
use middleware::authentication::IsAuthenticated;
use models::project::Project;
use models::project::user_to_project::UserProject;
use hbs::handlebars::to_json;

pub struct StaticController;

impl StaticController {
    pub fn index(req: &mut Request) -> IronResult<Response> {
        let mut data = templating::get_base_template_data(req);

        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            let user = req.extensions.get::<AuthenticatedUser>().unwrap();
            let projects = Project::find_all_by_owner(user.id).unwrap();
            data.insert("projects".to_owned(), to_json(&projects));

            let mut assigned_projects = Vec::new();
            for p in UserProject::get_projects(user) {
                assigned_projects.push(p.1)
            }
            data.insert("memberprojects".to_owned(), to_json(&assigned_projects));
        }

        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
