use crate::models::statistics::Statistics;
use crate::models::vote::Vote;
use crate::schema::votes;
use crate::schema::votes::columns::prediction;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::PgConnection;

pub enum StatisticsError {
    SerializationFailure,
}

pub fn create(conn: &PgConnection, id: i32) -> Result<Statistics, StatisticsError> {
    let result = votes::table
        .filter(prediction.eq(id))
        .load::<Vote>(conn)
        .expect("error loading votes");
    let total_votes:i32 =result.iter().map(|x|x.points).sum();
    let yes_votes:i32 = result.iter().filter(|x|x.outcome == true).map(|x|x.points).sum();
    let no_votes:i32 = result.iter().filter(|x|x.outcome == false).map(|x|x.points).sum();
    let statistics = Statistics {
        prediction_id: id,
        total_votes,
        yes_votes,
        no_votes,
    };
    Ok(statistics)
}
