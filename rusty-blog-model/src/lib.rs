mod model;

use diesel::{prelude::*, PgConnection};
use dotenvy::dotenv;
use std::env;

use crate::model::{models::*, queries};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB url not found");

    PgConnection::establish(&db_url).expect("DB connection err")
}

pub fn full_example() {
    let conn = &mut establish_connection();
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
