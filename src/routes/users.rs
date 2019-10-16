use crate::models::user::User;
use rocket_contrib::json::{Json, JsonValue};
//use crate::db::users::NewUser;
use crate::db;
use crate::db::Conn;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewUser {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

//#[derive(Deserialize, Validate)]
//struct NewUserData {
//    #[validate(length(min="1"))]
//    first_name:Option<String>,
//    #[validate(length(min="1"))]
//    last_name:Option<String>,
//    #[validate(email)]
//    email:Option<String>,
//    #[validate(length(min="8"))]
//    password: Option<String>
//
//}

#[get("/user/<id>")]
pub fn get_user(id: i32,conn: db::Conn) -> Option<JsonValue> {
    db::users::find(&conn,id).map(|user|json!({"user":user}))
}

#[post("/user", format = "json", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>, conn: Conn) -> Result<JsonValue, JsonValue> {
    db::users::create(
        &conn,
        new_user.email.as_ref(),
        new_user.password.as_ref(),
        new_user.first_name.as_ref(),
        new_user.last_name.as_ref(),
        "USER",
    )
    .map(|user| json!(user))
    .map_err(|error| {
        let field = match error {
            db => "email",
        };
        json!({
            "status":"error",
            "reason":"email already in use"
        })
    })
}
