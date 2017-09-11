use diesel::prelude::*;

use models::database;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub displayname: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn find_by_id(id: i32) -> QueryResult<Vec<User>> {
        use models::schema::users::dsl::*;

        let connection = database::connect();
        users.filter(id.eq(id))
            .limit(1)
            .load::<User>(&connection)
    }

    pub fn authenticate(username: &str, password: &str) -> bool {
        true
    }
}
