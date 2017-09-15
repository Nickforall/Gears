use router::Router;
use controllers::StaticController;
use controllers::AuthenticationController;
use hbs::{HandlebarsEngine, DirectorySource, Template};
use templating;

mod notfound;

pub use self::notfound::NotFound;

pub fn all() -> Router {
	router!{
        auth_login: post "/auth/login" => AuthenticationController::login,
        index: get "/" => StaticController::index
    }
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

	// Return the Handlebars Engine
	hbse
}

pub fn get_404_handler(tpl_name: &str) -> NotFound {
	// Create and return a 404 handler with the given template
	NotFound::new(Template::new(tpl_name,
		templating::get_base_template_data()))
}
