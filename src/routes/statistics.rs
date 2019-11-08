use crate::db::statistics;
use crate::db::Conn as connection;
use crate::models::user::Payload;
use rocket_contrib::json::JsonValue;

#[get("/statistics/<id>")]
pub fn get_statistics_by_id(
    id: i32,
    _auth: Payload,
    conn: connection,
) -> Result<JsonValue, JsonValue> {
    statistics::create(&conn, id)
        .map(|statistics| json!(statistics))
        .map_err(|_error| {
            json!({
                "status":"error",
                "reason":"Failed to create statistics object"
            })
        })
}
