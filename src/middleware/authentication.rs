use iron::prelude::*;
use typemap;
use middleware;
use iron::{Handler, AroundMiddleware};
use models::user::User;
use middleware::sessions::SessionKey;

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
            Some(mut session) => {
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
