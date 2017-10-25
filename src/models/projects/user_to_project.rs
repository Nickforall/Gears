use diesel::prelude::*;
use models::schema::user_projects;
use models::user::User;
use models::project::Project;
use models::database;
use models::schema::users;
use models::schema::projects;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User)]
#[belongs_to(Project)]
pub struct UserProject {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
}

impl UserProject {
    pub fn get_users(proj: &Project) -> Vec<(UserProject, User)> {
        let connection = database::connect();
        joinable!(user_projects -> users (user_id));

        user_projects::table
            .inner_join(users::table)
            .load::<(UserProject, User)>(&connection)
            .expect("Error loading users")
    }

    pub fn get_projects(user: &User) -> Vec<(UserProject, User)> {
        let connection = database::connect();
        joinable!(user_projects -> projects (project_id));

        user_projects::table
            .inner_join(users::table)
            .load::<(UserProject, User)>(&connection)
            .expect("Error loading users")
    }
}

#[derive(Insertable)]
#[table_name="user_projects"]
pub struct NewUserProject {
    pub user_id: i32,
    pub project_id: i32,
}
