pub mod model;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;

fn establish_connection() -> ConnectionManager<PgConnection> {
    dotenv().ok();
    ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DB URL NOT FOUND"))
}

pub fn get_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    Pool::builder()
        .build(establish_connection())
        .expect("DB Pool err")
}
