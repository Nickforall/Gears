use diesel::prelude::*;

use models::database;
use models::schema::projects;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
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
            .unwrap();

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
            .unwrap();

        projects.order(id.desc()).first(&connection).unwrap()
    }
}
