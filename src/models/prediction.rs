use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Prediction {
    id: i32,
    owner: i32,
    statement: String,
    expiry: NaiveDateTime,
    outcome: Option<bool>,
}
