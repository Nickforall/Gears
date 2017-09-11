extern crate router;

use self::router::Router;
use super::controllers::StaticController;
use hbs::{HandlebarsEngine, DirectorySource, Template};
use templating;

mod notfound;

pub use self::notfound::NotFound;

pub fn all() -> Router {
	let mut router = Router::new();
    router.get("/", StaticController::index, "index");
    router
}

pub fn templates() -> HandlebarsEngine {
	// create handlebars engine
	let mut hbse = HandlebarsEngine::new();
	// add a directory source, all files with .hbs suffix will be loaded as template
	hbse.add(Box::new(DirectorySource::new("./src/views", ".hbs")));

	// load templates from all registered sources
	if let Err(r) = hbse.reload() {
		panic!("{}", r);
	}

	return hbse;
}

pub fn get_404_handler(tpl_name: &str) -> NotFound {
	NotFound::new(Template::new(tpl_name,  templating::get_base_template_data()))
}
