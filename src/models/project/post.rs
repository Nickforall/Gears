use diesel::prelude::*;

use models::database;

use models::user::User;
use models::project::Project;
use models::schema::posts;
use chrono::prelude::*;

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
    pub fn all_by_project(project: &Project) -> QueryResult<Vec<(Post, User)>> {
        use models::schema::posts::dsl::id;
        use models::schema::users;

        joinable!(posts -> users (user_id));

        let connection = database::connect();
        Post::belonging_to(project)
            .inner_join(users::table)
            .order(id.desc())
            .load::<(Post, User)>(&connection)
    }

    pub fn create(project: i32, user: i32, pcontent: String) {
        use diesel::insert;
        use models;

        let connection = database::connect();
        let local: DateTime<Local> = Local::now();

        let new_post = &NewPost {
            user_id: user,
            project_id: project,
            posted_at: local.timestamp() as i32,
            content: pcontent.as_str(),
        };

        insert(new_post)
            .into(models::schema::posts::table)
            .execute(&connection)
            .expect("Removing user from project failed");
    }
}

#[derive(Insertable)]
#[table_name="posts"]
struct NewPost<'a> {
    user_id: i32,
    project_id: i32,
    posted_at: i32,
    content: &'a str,
}
