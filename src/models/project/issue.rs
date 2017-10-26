use diesel::prelude::*;

use models::database;

use models::user::User;
use models::project::Project;
use models::schema::issues;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User)]
#[belongs_to(Project)]
struct Issue {
    id: i32,
    user_id: i32,
    project_id: i32,
    posted_at: i32,
    content: Option<String>,
    is_resolved: bool,
    assignee: i32,
}

impl Issue {
    pub fn all() -> QueryResult<Vec<Issue>> {
        use models::schema::issues::dsl::issues;

        let connection = database::connect();
        issues
            .load::<Issue>(&connection)
    }
}
