use crate::models::vote::Vote;
use crate::schema::votes;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::PgConnection;

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote {
    prediction: i32,
    user_id: i32,
    points: i32,
    outcome: bool,
}

pub enum VoteError {
    ForeignKeyViolation,
}

impl From<Error> for VoteError {
    fn from(err: Error) -> Self {
        if let Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) = &err {
            match info.constraint_name() {
                Some("predictions_prediction_fkey") => return VoteError::ForeignKeyViolation,
                Some("predictions_user_id_fkey") => return VoteError::ForeignKeyViolation,
                _ => {}
            }
        }
        panic!("Error creating prediction: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    prediction: &i32,
    used_id: &i32,
    points: &i32,
    outcome: &bool,
) -> Result<Vote, VoteError> {
    let new_vote = &NewVote {
        prediction: *prediction,
        user_id: *used_id,
        points: *points,
        outcome: *outcome,
    };
    diesel::insert_into(votes::table)
        .values(new_vote)
        .get_result::<Vote>(conn)
        .map_err(Into::into)
}

pub fn find_by_id(conn: &PgConnection, id: i32) -> Option<Vote> {
    votes::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_vote_by_id: {}", err))
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Option<Vec<Vote>> {
    votes::table
        .load::<Vote>(conn)
        .map_err(|err| println!("find_all_votes: {}", err))
        .ok()
}