use iron::prelude::*;
use typemap;
use iron::{Handler, AroundMiddleware, status};
use models::user::User;
use middleware::sessions::{SessionKey, Session};
use routes;
use iron::modifiers::RedirectRaw;

pub struct AuthMiddleware;
pub struct IsAuthenticated;
pub struct AuthenticatedUser;

impl typemap::Key for IsAuthenticated {
    type Value = bool;
}

impl typemap::Key for AuthenticatedUser {
    type Value = User;
}

pub struct AuthHandler<H: Handler> {
    handler: H
}

impl<H: Handler> Handler for AuthHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let is_authenticated;
        let id = match req.extensions.remove::<SessionKey>() {
            Some(session) => {
                is_authenticated = true;
                session.id
            },
            None => {
                is_authenticated = false;
                0
            }
        };

        // Check whether the login session is set.
        req.extensions.insert::<IsAuthenticated>(is_authenticated);

        // If the user is authenticated, we want to get their user info
        if is_authenticated {
            let user = User::find_by_id(id).unwrap().first().unwrap().clone();

            // Put the user in an extensions, so we can reach it in any controller
            req.extensions.insert::<AuthenticatedUser>(user);

            // Re-insert session if everything went alright
            req.extensions.insert::<SessionKey>(Session {
                id: id
            });
        }

        // only check auth for routed requests
        if !req.url.path().join("/").starts_with("static/") {
            // if the route isn't public and you aren't authenticated, redirect to login
            if !routes::get_public_routes().contains(&req.url.path().join("/")) && !is_authenticated {
                return Ok(Response::with((status::Found, RedirectRaw("/".to_owned()))))
            }
        }

        // Execute the original handler
        let res = self.handler.handle(req);

        // Return an IronResult
        res
    }
}

impl AroundMiddleware for AuthMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(AuthHandler {
            handler: handler
        }) as Box<Handler>
    }
}
