pub struct AuthenticationController;

use middleware;
use models::user::User;

use iron::prelude::*;
use iron::{status, Url};
use iron::modifiers::Redirect;
use iron_sessionstorage::SessionRequestExt;
use std::io::Read;
use helpers;

#[derive(Debug)]
pub struct LoginData {
    email: String,
    password: String
}

impl LoginData {
    /// Parses a request into LoginData
    pub fn parse(req: &mut Request) -> Self {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let data = helpers::decode_body(body);

        LoginData {
            email: data.get("email").unwrap().clone(),
            password: data.get("password").unwrap().clone(),
        }
    }
}

#[derive(Debug)]
pub struct SignupData {
    email: String,
    displayname: String,
    password: String
}

impl SignupData {
    /// Parses a request into LoginData
    pub fn parse(req: &mut Request) -> Self {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let data = helpers::decode_body(body);

        // TODO: validate

        SignupData {
            email: data.get("email").unwrap().clone(),
            displayname: data.get("displayname").unwrap().clone(),
            password: data.get("password").unwrap().clone(),
        }
    }
}

impl AuthenticationController {
    pub fn login(req: &mut Request) -> IronResult<Response> {
        use middleware::sessions::Login;
        use iron_sessionstorage::Value;

        let url: Url;

        // read the body from a string and receive
        let login_data = LoginData::parse(req);

        if try!(req.session().get::<middleware::sessions::Login>()).is_some() {
            url = url_for!(req, "index", "status" => "already_loggedin");
        } else {
            match User::authenticate(login_data.email, login_data.password) {
                Ok(user) => {
                    url = url_for!(req, "index", "status" => "success");
                    req.session().set(Login::from_raw(user.id.to_string()).unwrap());
                },
                Err(_) => url = url_for!(req, "index", "status" => "failure"),
            }
        }

        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }

    pub fn signup(req: &mut Request) -> IronResult<Response> {
        use middleware::sessions::Login;
        use iron_sessionstorage::Value;

        let url: Url = url_for!(req, "index", "status" => "successful");

        let signup_data = SignupData::parse(req);

        let user = User::create(
            signup_data.email.as_str(),
            signup_data.displayname.as_str(),
            signup_data.password.as_str()
        ).unwrap().first().unwrap().clone();

        // Authenticate the new user
        req.session().set(Login::from_raw(user.id.to_string()).unwrap());

        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }
}
