use rocket_contrib::databases::diesel;
use diesel::PgConnection;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
