use crate::models::prediction::Prediction;
use crate::schema::predictions;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::PgConnection;

pub enum PredictionError {
    ForeignKeyViolation,
}

#[derive(Insertable)]
#[table_name = "predictions"]
pub struct NewPrediction {
    pub owner: i32,
    pub statement: String,
    pub expiry: NaiveDateTime,
    pub outcome: bool,
    pub votes: i32,
}

impl From<Error> for PredictionError {
    fn from(err: Error) -> Self {
        if let Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) = &err {
            match info.constraint_name() {
                Some("predictions_owner_fkey") => return PredictionError::ForeignKeyViolation,
                _ => {}
            }
        }
        panic!("Error creating prediction: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    owner: &i32,
    statement: &String,
    expiry: NaiveDateTime,
    outcome: &bool,
    votes: &i32,
) -> Result<Prediction, PredictionError> {
    let new_prediction = &NewPrediction {
        owner:*owner,
        statement:statement.to_string(),
        expiry:expiry,
        outcome:*outcome,
        votes:*votes,
    };
    diesel::insert_into(predictions::table)
        .values(new_prediction)
        .get_result::<Prediction>(conn)
        .map_err(Into::into)
}

pub fn find_by_id(conn: &PgConnection, id: i32) -> Option<Prediction> {
    predictions::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_prediction_by_id: {}", err))
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Option<Vec<Prediction>> {
    predictions::table
        .load::<Prediction>(conn)
        .map_err(|err| println!("find_all: {}", err))
        .ok()
}
