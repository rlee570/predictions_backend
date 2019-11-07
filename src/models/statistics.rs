#[derive(Serialize)]
pub struct Statistics {
    pub prediction_id: i32,
    pub total_votes: i32,
    pub yes_votes: i32,
    pub no_votes: i32,
}
