use rocket::config::Datetime;

pub struct Prediction {
    id: i32,
    //change to foreign key
    owner: i32,
    statement: String,
    expiry: Datetime,
    outcome: bool,
    votes: i32,
}
