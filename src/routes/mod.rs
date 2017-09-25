use router::Router;
use controllers::StaticController;
use controllers::AuthenticationController;
use hbs::{HandlebarsEngine, DirectorySource};

mod notfound;

pub use self::notfound::NotFound;

pub fn all() -> Router {
	router! {
        auth_login: post "/auth/login" => AuthenticationController::login,
		auth_signup: post "/auth/signup" => AuthenticationController::signup,
		// must be at root level (iron-sessionstorage#8)
		auth_logout: get "/logout" => AuthenticationController::logout,
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
	NotFound::new(tpl_name)
}
