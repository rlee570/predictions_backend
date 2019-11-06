#[derive(Queryable, Serialize)]
pub struct Vote {
    id: i32,
    prediction: i32,
    user_id: i32,
    points: i32,
    outcome: bool,
}
