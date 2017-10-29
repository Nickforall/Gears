pub struct AuthenticationController;

use models::user::User;

use iron::prelude::*;
use iron::{status, Url};
use iron::modifiers::Redirect;
use std::io::Read;
use helpers;
use middleware::sessions::{Session, SessionKey};
use middleware::authentication::IsAuthenticated;
use helpers::error::ErrorBag;

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
    pub fn validate(body: String) -> ErrorBag {
        use gild::ValidationChain;
        use gild::validators;

        let data = helpers::decode_body(body);
        let mut errors = ErrorBag::new();

        if data.get("email").unwrap().is_empty() {
            errors.add("An email Is Required");
        } else {
            if ValidationChain::new()
               .add(validators::IsEmail::new())
               .validate(data.get("email").unwrap().clone())
               .is_err() {
                   errors.add("Email Address isn't valid");
            }
        }

        if data.get("displayname").unwrap().is_empty() {
            errors.add("A displayname Is Required");
        }

        if data.get("password").unwrap().is_empty() {
            errors.add("A password Is Required");
        } else {
            if ValidationChain::new()
               .add(validators::MinSize::new(6))
               .validate(data.get("password").unwrap().clone())
               .is_err() {
                   errors.add("Your password should be at least 6 characters");
            }
        }

        errors
    }

    /// Parses a request into SignupData
    pub fn parse(body: String) -> Self {
        let data = helpers::decode_body(body);

        SignupData {
            email: data.get("email").unwrap().clone(),
            displayname: data.get("displayname").unwrap().clone(),
            password: data.get("password").unwrap().clone(),
        }
    }
}

impl AuthenticationController {
    /// Called when logging in
    pub fn login(req: &mut Request) -> IronResult<Response> {
        let url: Url;

        // read the body from a string and receive
        let login_data = LoginData::parse(req);

        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            url = url_for!(req, "index", "status" => "already_loggedin");
        } else {
            // Check whether the passed credentials
            match User::authenticate(login_data.email, login_data.password) {
                Ok(user) => {
                    url = url_for!(req, "index", "status" => "success");
                    // Authenticate the new user
                    req.extensions.insert::<SessionKey>(Session {
                        id: user.id
                    });
                },
                Err(_) => url = url_for!(req, "index", "status" => "failure"),
            }
        }

        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }

    /// Called when signing up
    pub fn signup(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let errors = SignupData::validate(body.clone());
        if errors.has_errors() {
            return Ok(Response::with((
                status::Found,
                Redirect(url_for!(req, "index", "errors" => errors.encode()))
            )));
        }

        let signup_data = SignupData::parse(body);
        let mut url = url_for!(req, "index", "status" => "successful");

        if *req.extensions.get::<IsAuthenticated>().unwrap() {
            // if the user is already authenticated, don't do this..
            url = url_for!(req, "index", "status" => "already_loggedin");
        } else {
            // create a new user with the given credentials
            let user = User::create(
                signup_data.email.as_str(),
                signup_data.displayname.as_str(),
                signup_data.password.as_str()
            ).unwrap().first().unwrap().clone();

            // Authenticate the new user
            req.extensions.insert::<SessionKey>(Session {
                id: user.id
            });
        }

        Ok(Response::with((status::Found,
            Redirect(url)
        )))
    }

    /// Called when logging out
    pub fn logout(req: &mut Request) -> IronResult<Response> {
        // Empty the session
        req.extensions.remove::<SessionKey>();

        Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
    }
}
