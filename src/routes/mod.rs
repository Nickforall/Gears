use router::Router;
use controllers::{ StaticController, AuthenticationController, ProjectController, PostController };

use hbs::{HandlebarsEngine, DirectorySource};
use templating;

pub mod notfound;

pub use self::notfound::NotFound;

pub fn all() -> Router {
	router! {
		projects_new: post "/projects/new" => ProjectController::new,
		projects_ls: get "/projects" => ProjectController::list,
		projects_detail: get "/projects/:id" => ProjectController::get,
		projects_edit: post "/projects/:id/edit" => ProjectController::edit,
		projects_posts: get "/projects/:id/posts" => PostController::ls,
		projects_posts_new: get "/projects/:id/posts/new" => PostController::create_form,
		projects_posts_post: post "/projects/:id/posts/new" => PostController::post_form,
		projects_search: get "/projects/search/:query" => ProjectController::search,
		projects_toggle_user: post "/projects/:id/users/toggle" => ProjectController::toggle_user,
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

  	hbse.handlebars_mut().register_helper("notEq", Box::new(templating::helpers::not_eq_helper));
	hbse.handlebars_mut().register_helper("debug", Box::new(templating::helpers::debug));
	hbse.handlebars_mut().register_helper("toString", Box::new(templating::helpers::to_string));
	hbse.handlebars_mut().register_helper("equals", Box::new(templating::helpers::eq_helper));
	hbse.handlebars_mut().register_helper("nequals", Box::new(templating::helpers::not_eq_helper));
	hbse.handlebars_mut().register_helper("time", Box::new(templating::helpers::fmt_time));

	// load templates from all registered sources
	if let Err(r) = hbse.reload() {
		panic!("{}", r);
	}

	// Return the Handlebars Engine
	hbse
}

/// Gets the names of routes that require authentication
pub fn get_public_routes() -> Vec<String> {
	vec![
		"".to_owned(),
		"auth/login".to_owned(),
		"auth/signup".to_owned()
	]
}

pub fn get_404_handler(tpl_name: &str) -> NotFound {
	// Create and return a 404 handler with the given template
	NotFound::new(tpl_name)
}
