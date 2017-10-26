use diesel::prelude::*;

use models::database;

use models::user::User;
use models::project::Project;
use models::schema::posts;

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Project)]
pub struct Post {
    id: i32,
    user_id: i32,
    project_id: i32,
    posted_at: i32,
    content: Option<String>
}

impl Post {
    pub fn all_by_project(project: &Project) -> QueryResult<Vec<Post>> {
        let connection = database::connect();
        Post::belonging_to(project)
            .load::<Post>(&connection)
    }
}
