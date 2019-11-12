use crate::db;
use crate::db::Conn;
use crate::models::user::Payload;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize)]
pub struct NewUser {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
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
pub fn get_user_by_id(id: i32, _auth: Payload, conn: db::Conn) -> Option<JsonValue> {
    db::users::find_by_id(&conn, id).map(|user| json!(user))
}

#[get("/user/email?<email>")]
pub fn get_user_by_email(email: String, _auth: Payload, conn: db::Conn) -> Option<JsonValue> {
    db::users::find_by_email(&conn, email.as_ref()).map(|user| json!(user))
}

#[post("/user", format = "json", data = "<new_user>")]
pub fn post_create_user(
    new_user: Json<NewUser>,
    conn: Conn,
) -> Result<JsonValue, Custom<Json<JsonValue>>> {
    db::users::create(
        &conn,
        new_user.email.as_ref(),
        new_user.password.as_ref(),
        new_user.first_name.as_ref(),
        new_user.last_name.as_ref(),
        "USER",
    )
    .map(|user| json!({ "user": user }))
    .map_err(|_error| {
        status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status":"error",
                "reason":"email already in use"
            })),
        )
    })
}

#[post("/user/login", format = "json", data = "<user>")]
pub fn post_user_login(
    user: Json<LoginData>,
    conn: Conn,
) -> Result<JsonValue, Custom<Json<JsonValue>>> {
    db::users::login(&conn, user.email.as_ref(), user.password.as_ref())
        .map(|user| json!({ "user": user.to_user_jwt() }))
        .ok_or_else(|| {
            status::Custom(
                Status::InternalServerError,
                Json(json!({"status":"error","reason":"username or password is invalid"})),
            )
        })
}
