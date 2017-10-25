use diesel::prelude::*;

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
    content: String
}
