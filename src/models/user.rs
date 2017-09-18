use diesel::prelude::*;

use models::database;
use models::schema::users;

pub enum AuthenticationError {
    NonExistent,
    Invalid
}

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub displayname: String,
    pub email: String,
    pub password: String,
}

impl User {
    /// Finds a user by its id
    pub fn find_by_id(id: i32) -> QueryResult<Vec<User>> {
        use models::schema::users::dsl::{id, users};

        let connection = database::connect();
        users
            .filter(id.eq(id))
            .limit(1)
            .load::<User>(&connection)
    }

    /// Finds a user by its email address
    pub fn find_by_email(uemail: String) -> QueryResult<Vec<User>> {
        use models::schema::users::dsl::{email, users};

        let connection = database::connect();
        users
            .filter(email.eq(uemail))
            .limit(1)
            .load::<User>(&connection)
    }

    /// Returns an option that tells you whether the authentication was succesfull, if it was
    /// `Ok` contains the authenticated user.
    pub fn authenticate(inusername: String, inpassword: String) -> Result<User, AuthenticationError> {
        let user_vec = User::find_by_email(inusername).unwrap();

        match user_vec.first() {
            Some(u) => {
                if u.password == inpassword {
                    return Ok(u.clone())
                } else {
                    return Err(AuthenticationError::Invalid)
                }
            },
            None =>  return Err(AuthenticationError::NonExistent),
        };
    }

    /// Creates a new user
    pub fn create(mail: &str, dname: &str, pass: &str) -> QueryResult<Vec<User>> {
        use diesel::insert;

        let connection = database::connect();
        let new_user = &NewUser {
            password: pass,
            displayname: dname,
            email: mail,
        };

        insert(new_user)
            .into(users::table)
            .execute(&connection);

        User::find_by_email(mail.to_owned())
    }
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    password: &'a str,
    displayname: &'a str,
    email: &'a str,
}
