use crate::db::users;
use crate::db::users::UpdateUser as update;
use crate::db::votes;
use crate::db::Conn as connection;
use crate::models::user::Payload;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize)]
pub struct NewVote {
    prediction: i32,
    user_id: i32,
    points: i32,
    outcome: bool,
}

#[get("/vote/<id>")]
pub fn get_vote_by_id(id: i32, _auth: Payload, conn: connection) -> Option<JsonValue> {
    votes::find_by_id(&conn, id).map(|vote| json!(vote))
}

#[get("/votes")]
pub fn get_all_votes(_auth: Payload, conn: connection) -> Option<JsonValue> {
    votes::find_all(&conn).map(|votes| json!(votes))
}

#[post("/vote", format = "json", data = "<new_vote>")]
pub fn post_create_vote(
    new_vote: Json<NewVote>,
    _auth: Payload,
    conn: connection,
) -> Result<JsonValue, Custom<Json<JsonValue>>> {
    let user = users::find_by_id(&conn, new_vote.user_id).unwrap();
    if user.points < new_vote.points {
        return Err(status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status":"error",
                "reason":"Not enough points",
            })),
        ));
    } else {
        let new_points = user.points - new_vote.points;
        let update = &update {
            email: None,
            first_name: None,
            last_name: None,
            points: Some(new_points),
            role: None,
            hash: None,
        };
        users::update(&conn, user.id, update);
    }
    //Create User point check.
    votes::create(
        &conn,
        &new_vote.prediction,
        &new_vote.user_id,
        &new_vote.points,
        &new_vote.outcome,
    )
    .map(|vote| json!(vote))
    .map_err(|_error| {
        status::Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "reason":"Failed to create vote"
            })),
        )
    })
}
