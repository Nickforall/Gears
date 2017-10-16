use diesel::prelude::*;

use models::database;
use models::schema::users;
use pwhash::bcrypt;

pub enum AuthenticationError {
    NonExistent,
    Invalid
}

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub displayname: String,
    pub password: String,
}

impl User {
    /// Finds a user by its id
    pub fn find_by_id(uid: i32) -> QueryResult<Vec<User>> {
        use models::schema::users::dsl::{id, users};

        let connection = database::connect();
        users
            .filter(id.eq(uid))
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

                if bcrypt::verify(inpassword.as_str(), u.password.as_str()) {
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
        let hashed_pass = bcrypt::hash(pass).unwrap();

        let new_user = &NewUser {
            password: hashed_pass.as_str(),
            displayname: dname,
            email: mail,
        };

        insert(new_user)
            .into(users::table)
            .execute(&connection)
            .unwrap();

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
