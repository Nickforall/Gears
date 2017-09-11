pub struct AuthenticationController;

use middleware;
use models::user::User;

use iron::prelude::*;
use iron::{status, Url};
use iron::modifiers::RedirectRaw;
use params::{Params, Value};
use iron_sessionstorage::SessionRequestExt;

impl AuthenticationController {
    pub fn login(req: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();
        {
            //TODO: find a way that the scopes don't fuck up
            let input_map = req.get_ref::<Params>().unwrap();
            println!("{:?}", input_map);
        }

        if try!(req.session().get::<middleware::sessions::Login>()).is_some() {
            resp.set_mut(RedirectRaw("/?err=already_logged_in".to_string()));
        } else {
            if User::authenticate("hello@nickforall.nl", "password") {
                resp.set_mut(RedirectRaw("/?success=1".to_string()));
            } else {
                resp.set_mut(RedirectRaw("/?success=0".to_string()));
            }
        }

        resp.set_mut(status::TemporaryRedirect);

        Ok(resp)
    }
}
