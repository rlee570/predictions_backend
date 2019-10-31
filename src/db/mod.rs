use rocket_contrib::databases::diesel;

pub mod predictions;
pub mod users;
pub mod votes;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
