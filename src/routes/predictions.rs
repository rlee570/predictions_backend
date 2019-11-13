use crate::db::predictions;
use crate::db::predictions::UpdatePrediction as update;
use crate::db::users;
use crate::db::votes;
use crate::db::Conn as connection;
use crate::models::user::{Payload, User};
use chrono::DateTime;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize)]
pub struct NewPrediction {
    owner: i32,
    statement: String,
    expiry: String,
    outcome: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdatePrediction {
    prediction: update,
}

#[get("/prediction/<id>")]
pub fn get_prediction_by_id(id: i32, _auth: Payload, conn: connection) -> Option<JsonValue> {
    predictions::find_by_id(&conn, id).map(|prediction| json!(prediction))
}

#[get("/predictions")]
pub fn get_all_predictions(conn: connection) -> Option<JsonValue> {
    predictions::find_all(&conn).map(|predictions| json!(predictions))
}

#[put("/prediction/<id>", format = "json", data = "<prediction>")]
pub fn put_prediction(
    id: i32,
    prediction: Json<UpdatePrediction>,
    _auth: Payload,
    conn: connection,
) -> Option<JsonValue> {
    predictions::update(&conn, id, &prediction.prediction).map(|prediction| json!(prediction))
}

#[post("/prediction", format = "json", data = "<new_prediction>")]
pub fn post_create_prediction(
    new_prediction: Json<NewPrediction>,
    _auth: Payload,
    conn: connection,
) -> Result<JsonValue, Custom<Json<JsonValue>>> {
    let datetime = DateTime::parse_from_rfc3339(&new_prediction.expiry)
        .unwrap()
        .naive_utc();
    predictions::create(
        &conn,
        &new_prediction.owner,
        &new_prediction.statement,
        datetime,
        &new_prediction.outcome,
    )
    .map(|prediction| json!(prediction))
    .map_err(|_error| {
        status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "reason":"Failed to create prediction"
            })),
        )
    })
}

#[derive(Deserialize)]
pub struct Outcome {
    outcome: bool,
}

#[put("/prediction/outcome/<id>", format = "json", data = "<outcome>")]
pub fn put_prediction_outcome(
    id: i32,
    outcome: Json<Outcome>,
    _auth: Payload,
    conn: connection,
) -> Option<JsonValue> {
    predictions::update_outcome(&conn, id, outcome.outcome);
    let result: Vec<User> = votes::find_by_prediction_id_and_outcome(&conn, id, outcome.outcome)
        .unwrap()
        .iter()
        .map(|vote| users::update_points(&conn, vote.user_id, vote.points).unwrap())
        .collect();
    Some(json!(result))
}
