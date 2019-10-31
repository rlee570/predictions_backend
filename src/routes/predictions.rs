use crate::db::predictions;
use crate::db::predictions::UpdatePrediction as update;
use crate::db::Conn as connection;
use crate::models::user::Payload;
use chrono::NaiveDate;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize)]
pub struct NewPrediction {
    owner: i32,
    statement: String,
    expiry: String,
    outcome: bool,
    votes: i32,
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
pub fn get_all_predictions(_auth: Payload, conn: connection) -> Option<JsonValue> {
    predictions::find_all(&conn).map(|prediction| json!(prediction))
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
) -> Result<JsonValue, JsonValue> {
    let split: Vec<&str> = new_prediction.expiry.split(".").collect();
    let year: i32 = split[0].parse::<i32>().unwrap();
    let slice = split[1..]
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    let datetime =
        NaiveDate::from_ymd(year, slice[0], slice[1]).and_hms(slice[2], slice[3], slice[4]);
    predictions::create(
        &conn,
        &new_prediction.owner,
        &new_prediction.statement,
        datetime,
        &new_prediction.outcome,
        &new_prediction.votes,
    )
    .map(|prediction| json!(prediction))
    .map_err(|_error| {
        json!({
            "status": "error",
            "reason":"Failed to create user"
        })
    })
}
