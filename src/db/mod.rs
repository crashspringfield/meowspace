use rocket_contrib::databases::diesel;

pub mod cats;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
