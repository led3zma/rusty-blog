pub mod controller;
pub mod model;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use std::{
    env::{self, VarError},
    error::Error,
};

fn establish_connection() -> Result<ConnectionManager<PgConnection>, VarError> {
    dotenv().ok();
    Ok(ConnectionManager::<PgConnection>::new(env::var(
        "DATABASE_URL",
    )?))
}

pub fn get_db_pool() -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn Error>> {
    Ok(Pool::builder().build(establish_connection().expect("err"))?)
}
