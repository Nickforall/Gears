use hbs::Template;
use router::NoRoute;
use iron::AfterMiddleware;
use iron::status;
use iron::prelude::*;

pub struct NotFound {
	template: Template
}

impl NotFound {
	pub fn new(tpl: Template) -> Self {
		// Set a template for the returned 404 page struct
		return NotFound { template: tpl }
	}
}

impl AfterMiddleware for NotFound {
	fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
		// Check if a no route error occured
		if let Some(_) = err.error.downcast::<NoRoute>() {
			// Create a new route with our template
			let mut resp = Response::new();
	        resp.set_mut(self.template.clone()).set_mut(status::Ok);

			// Tell the chain we caught something and return a response
			Ok(resp)
		} else {
			// Tell the chain all is ok
			Err(err)
		}
	}
}
