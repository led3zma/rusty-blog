pub mod model;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;

use crate::model::{models::*, queries};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_simple_connection() -> PgConnection {
    dotenv().ok();
    PgConnection::establish(&env::var("DATABASE_URL").expect("DB URL NOT FOUND"))
        .expect("COULD NOT ESTABLISH DB CONNECTION")
}

fn establish_connection() -> ConnectionManager<PgConnection> {
    dotenv().ok();
    ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DB URL NOT FOUND"))
}

pub fn get_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    Pool::builder()
        .build(establish_connection())
        .expect("DB Pool err")
}

pub fn full_example() {
    let conn = &mut establish_simple_connection();
    let posts = queries::select_all(conn);
    println!("All posts: {:?}", posts);

    let created_post = queries::create(
        conn,
        NewPost {
            title: "Mi primer post",
            body: "LOREM IMPOSU",
            slug: "testing-post",
        },
    );

    println!("Created: {:?}", created_post);

    let updated_post = queries::update(
        conn,
        created_post.id,
        NewPost {
            title: &created_post.title,
            body: "WOWZERS",
            slug: &created_post.slug,
        },
    );

    println!("Updated: {:?}", updated_post);

    let posts = queries::select_all(conn);
    println!("All posts after new: {:#?}", posts);

    let deleted_posts = queries::delete_by_slug(conn, "%testing-%");
    println!("Deleted post: {deleted_posts}");
}
