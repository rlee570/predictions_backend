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
    pub outcome: Option<bool>,
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
    outcome: &Option<bool>,
) -> Result<Prediction, PredictionError> {
    let new_prediction = &NewPrediction {
        owner: *owner,
        statement: statement.to_string(),
        expiry,
        outcome: *outcome,
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
        .map_err(|err| println!("find_all_predictions: {}", err))
        .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "predictions"]
pub struct UpdatePrediction {
    pub owner: Option<i32>,
    pub statement: Option<String>,
    pub expiry: Option<NaiveDateTime>,
    pub outcome: Option<bool>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdatePrediction) -> Option<Prediction> {
    let data = &UpdatePrediction {
        owner: None,
        expiry: None,
        ..data.clone()
    };
    diesel::update(predictions::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}

pub fn update_outcome(conn: &PgConnection, id: i32, outcome: bool) -> Option<Prediction> {
    let update = &UpdatePrediction {
        owner: None,
        statement: None,
        expiry: None,
        outcome: Some(outcome),
    };
    diesel::update(predictions::table.find(id))
        .set(update)
        .get_result(conn)
        .ok()
}
