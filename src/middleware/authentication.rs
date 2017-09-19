use iron::prelude::*;
use iron::typemap;
use iron_sessionstorage::SessionRequestExt;
use middleware;
use iron::{Handler, AroundMiddleware};
use models::user::User;

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

        // Execute the original handler
        let res = self.handler.handle(req);

        // Check whether the login session is set.
        let is_authenticated = try!(req.session().get::<middleware::sessions::Login>()).is_some();
        req.extensions.insert::<IsAuthenticated>(is_authenticated);

        // If the user is authenticated, we want to get their user info
        if is_authenticated {
            let logindata = req.session().get::<middleware::sessions::Login>().unwrap().unwrap();
            let user = User::find_by_id(logindata.id.parse::<i32>().unwrap()).unwrap().first().unwrap().clone();

            // Put the user in an extensions, so we can reach it in any controller
            req.extensions.insert::<AuthenticatedUser>(user);
        }

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
