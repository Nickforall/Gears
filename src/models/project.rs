use diesel::prelude::*;

use models::database;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub owner_id: i32,
}

impl Project {
    /// Finds all projects by owner id
    pub fn find_all_by_owner(_: i32) -> QueryResult<Vec<Project>> {
        use models::schema::projects::dsl::{owner_id, projects};

        let connection = database::connect();
        projects
            .filter(owner_id.eq(owner_id))
            .load::<Project>(&connection)
    }
}
