use diesel::prelude::*;
use models::schema::user_projects;
use models::user::User;
use models::project::Project;
use models::database;
use models::schema::users;
use models::schema::projects;

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
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

        UserProject::belonging_to(proj)
            .inner_join(users::table)
            .load::<(UserProject, User)>(&connection)
            .expect("Error loading users")
    }

    pub fn get_projects(user: &User) -> Vec<(UserProject, Project)> {
        let connection = database::connect();
        joinable!(user_projects -> projects (project_id));

        UserProject::belonging_to(user)
            .inner_join(projects::table)
            .load::<(UserProject, Project)>(&connection)
            .expect("Error loading projects")
    }

    pub fn add_user(user: &User, proj: &Project) {
        use diesel::insert;
        use models;

        let connection = database::connect();

        let up = &NewUserProject {
            user_id: user.id,
            project_id: proj.id
        };

        insert(up)
            .into(models::schema::user_projects::table)
            .execute(&connection)
            .expect("Removing user from project failed");
    }

    pub fn remove_user(user: &User, proj: &Project) {
        use diesel::delete;
        use models::schema::user_projects::dsl::{user_id, project_id, user_projects};

        let connection = database::connect();

        delete(user_projects.filter(user_id.eq(user.id)).filter(project_id.eq(proj.id)))
            .execute(&connection).expect("Removing user from project failed");
    }
}

#[derive(Insertable)]
#[table_name="user_projects"]
pub struct NewUserProject {
    pub user_id: i32,
    pub project_id: i32,
}
