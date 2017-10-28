use iron::{Request, Response, IronResult, status, Set};
use templating;
use hbs::Template;
use hbs::handlebars::to_json;
use models::project::Project;
use models::project::post::Post;
use iron::modifiers::Redirect;
use helpers;
use std::io::Read;
use middleware::authentication::AuthenticatedUser;

struct PostData {
    post: String
}

impl PostData {
    pub fn parse(req: &mut Request) -> Self {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let data = helpers::decode_body(body);

        PostData {
            post: data.get("content").unwrap().clone()
        }
    }
}

pub struct PostController;

impl PostController {
    //TODO: inner join user.
    pub fn ls(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        let mut data = templating::get_base_template_data(req);
        let posts = Post::all_by_project(&project).unwrap();

        data.insert("project".to_owned(), to_json(&project));
        data.insert("posts".to_owned(), to_json(&posts));

        let mut resp = Response::new();
        resp.set_mut(Template::new("projects/posts", data)).set_mut(status::Ok);

        Ok(resp)
    }

    pub fn create_form(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        let mut data = templating::get_base_template_data(req);
        data.insert("project".to_owned(), to_json(&project));

        let mut resp = Response::new();
        resp.set_mut(Template::new("posts/new", data)).set_mut(status::Ok);

        Ok(resp)
    }

    pub fn post_form(req: &mut Request) -> IronResult<Response> {
        let project;
        match Project::from_request("id", req) {
            Ok(p) => project = p,
            Err(r) => return Ok(r)
        };

        //TODO; check if in project

        let form = PostData::parse(req);

        Post::create(
            project.id,
            req.extensions.get::<AuthenticatedUser>().unwrap().id,
            form.post.to_string()
        );

        Ok(Response::with((status::Found, Redirect(url_for!(req, "projects_posts",
            "id" => project.id.to_string()
        )))))
    }
}
