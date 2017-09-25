use hbs::Template;
use router::NoRoute;
use iron::AfterMiddleware;
use iron::status;
use iron::prelude::*;
use templating;

pub struct NotFound {
	template: String
}

impl NotFound {
	pub fn new(tpl: &str) -> Self {
		// Set a template for the returned 404 page struct
		return NotFound { template: tpl.to_owned() }
	}
}

impl AfterMiddleware for NotFound {
	fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
		// Check if a no route error occured
		if let Some(_) = err.error.downcast::<NoRoute>() {
			// Create a new route with our template
			let mut resp = Response::new();
			let template = Template::new(self.template.as_str(),
				templating::get_base_template_data(req));

			// set the response
	        resp.set_mut(template).set_mut(status::Ok);

			// Tell the chain we caught something and return a response
			Ok(resp)
		} else {
			// Tell the chain all is ok
			Err(err)
		}
	}
}
