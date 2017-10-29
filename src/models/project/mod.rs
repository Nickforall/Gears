pub mod user_to_project;
pub mod issue;
pub mod post;

use diesel;
use diesel::prelude::*;
use iron::Response;
use iron::prelude::*;

use models::database;
use models::schema::projects;

#[derive(Queryable, Associations, Identifiable, Clone, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[table_name="projects"]
struct NewProject<'a> {
    name: &'a str,
    description: &'a str,
    owner_id: i32,
}

impl Project {
    pub fn all() -> Vec<Project> {
        use models::schema::projects::dsl::projects;

        let connection = database::connect();
        projects
            .load::<Project>(&connection)
            .unwrap()
    }

    /// Finds all projects by owner id
    pub fn find_all_by_owner(in_owner_id: i32) -> QueryResult<Vec<Project>> {
        use models::schema::projects::dsl::{owner_id, projects};

        let connection = database::connect();
        projects
            .filter(owner_id.eq(in_owner_id))
            .load::<Project>(&connection)
    }

    pub fn find_or_fail(project_id: i32) -> Option<Project> {
        use models::schema::projects::dsl::{id, projects};

        let connection = database::connect();
        let proj = projects
            .filter(id.eq(project_id))
            .load::<Project>(&connection)
            .expect("Removing user from project failed");

        if proj.len() < 1 {
            return None
        }

        match proj.first() {
            Some(p) => Some(p.clone()),
            None => None
        }
    }

    pub fn create(project_name: &str, project_desc: &str, project_ownerid: i32) -> Project {
        use diesel::insert;
        use models;
        use models::schema::projects::dsl::{projects, id};

        let connection = database::connect();

        let new_project = &NewProject {
            name: project_name,
            description: project_desc,
            owner_id: project_ownerid,
        };

        insert(new_project)
            .into(models::schema::projects::table)
            .execute(&connection)
            .expect("Removing user from project failed");

        projects.order(id.desc()).first(&connection).unwrap()
    }

    pub fn update(&self, project_name: &str, project_desc: &str) {
        use models::schema::projects::dsl::{description, name};

        let connection = database::connect();

        diesel::update(self)
            .set((
                name.eq(project_name),
                description.eq(project_desc)
            ))
            .execute(&connection)
            .unwrap();
    }

    pub fn from_request(parameter: &str, req: &mut Request) -> Result<Project, Response> {
        use routes;
        use router::Router;

        let id = req.extensions.get::<Router>().unwrap().find(parameter).unwrap_or("0").to_string();
        let project;

        // check whether the id is a number
        match id.parse::<i32>() {
            Ok(numeric_id) => {
                // if it is, throw it into our find or fail
                match Project::find_or_fail(numeric_id) {
                    Some(p) => {
                        project = p;
                    },
                    None => return Err(routes::notfound::get_404_response("404", req))
                };
            },
            Err(_) => return Err(routes::notfound::get_404_response("404", req))
        };

        Ok(project)
    }

    pub fn search_by_name(query: &str) -> QueryResult<Vec<Project>> {
        use models::schema::projects::dsl::{projects, name};

        let connection = database::connect();
        let pattern = format!("%{}%", query);

        projects
            .filter(name.like(pattern))
            .load::<Project>(&connection)
    }

    pub fn search_by_description(query: &str) -> QueryResult<Vec<Project>> {
        use models::schema::projects::dsl::{projects, description};

        let connection = database::connect();
        let pattern = format!("%{}%", query);

        projects
            .filter(description.like(pattern))
            .load::<Project>(&connection)
    }
}
