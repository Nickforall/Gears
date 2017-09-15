pub struct AuthenticationController;

use middleware;
use models::user::User;

use iron::prelude::*;
use iron::{status, Url};
use iron::modifiers::Redirect;
use params::{Params, Value};
use iron_sessionstorage::SessionRequestExt;

impl AuthenticationController {
    pub fn login(req: &mut Request) -> IronResult<Response> {
        let url: Url;

        use std::io::Read;

        let mut body = vec![];
        req.body.read_to_end(&mut body).unwrap();

        if try!(req.session().get::<middleware::sessions::Login>()).is_some() {
            url = url_for!(req, "index", "status" => "already_loggedin");
        } else {
            if User::authenticate("hello@nickforall.nl", "password") {
                url = url_for!(req, "index", "status" => "success");
            } else {
                url = url_for!(req, "index", "status" => "failure");
            }
        }

        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }
}
