use hbs::Template;
use super::router::NoRoute;
use iron::AfterMiddleware;
use iron::status;
use iron::prelude::*;

pub struct NotFound {
	template: Template
}

impl NotFound {
	pub fn new(tpl: Template) -> Self {
		return NotFound { template: tpl }
	}
}

impl AfterMiddleware for NotFound {
	fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
		println!("Hitting custom 404 middleware");

		if let Some(_) = err.error.downcast::<NoRoute>() {
			let mut resp = Response::new();
	        resp.set_mut(self.template.clone()).set_mut(status::Ok);

			Ok(resp)
		} else {
			Err(err)
		}
	}
}
