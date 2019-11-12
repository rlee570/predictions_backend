use crate::db::statistics;
use crate::db::Conn as connection;
use crate::models::user::Payload;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket_contrib::json::{Json, JsonValue};

#[get("/statistics/<id>")]
pub fn get_statistics_by_id(
    id: i32,
    _auth: Payload,
    conn: connection,
) -> Result<JsonValue, Custom<Json<JsonValue>>> {
    statistics::create(&conn, id)
        .map(|statistics| json!(statistics))
        .map_err(|_error| {
            status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "status":"error",
                    "reason":"Failed to create statistics object"
                })),
            )
        })
}
