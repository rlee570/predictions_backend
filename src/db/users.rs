use crate::models::user::User;
use crate::schema::users;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub enum UserCreationError {
    DuplicatedEmail,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub points: i32,
    pub role: &'a str,
    pub hash: &'a str,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    email: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    role: &str,
) -> Result<User, UserCreationError> {
    // see https://blog.filippo.io/the-scrypt-parameters
    let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

    let new_user = &NewUser {
        email,
        first_name,
        last_name,
        points: 100,
        role,
        hash,
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn login(conn: &PgConnection, email: &str, password: &str) -> Option<User> {
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let password_matches = scrypt_check(password, &user.hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .ok()?;

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "Login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub fn find_by_id(conn: &PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user:{}", err))
        .ok()
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> Option<User> {
    users::table
        .filter(users::email.eq(email))
        .get_result(conn)
        .map_err(|err| eprintln!("find_by_email:{}", err))
        .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub points: Option<i32>,
    pub role: Option<String>,
    pub hash: Option<String>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateUser) -> Option<User> {
    let data = &UpdateUser {
        first_name: None,
        last_name: None,
        hash: None,
        ..data.clone()
    };
    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
