#[derive(Queryable, Serialize, Debug)]
pub struct Vote {
    pub id: i32,
    pub prediction: i32,
    pub user_id: i32,
    pub points: i32,
    pub outcome: bool,
}
