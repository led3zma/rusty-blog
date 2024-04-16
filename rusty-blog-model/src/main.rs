use dotenvy::dotenv;
use std::env;

use diesel::{pg::PgConnection, prelude::*};

use rusty_blog_model::models::*;
use rusty_blog_model::schema::posts::{self, dsl::*};

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB url not found");

    let connection = &mut PgConnection::establish(&db_url).expect("DB connection err");

    let new_post = NewPost {
        title: "Test",
        slug: "first-test",
        body: "This is a test",
    };

    let _: Post = diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(connection)
        .expect("DB insert err");

    // Select All
    println!("Select ALL posts");
    let post_records = posts.load::<Post>(connection).expect("DB query error");

    post_records.iter().for_each(|post| println!("{:?}", post));

    // Select with Limit 1
    println!("Select only 1 post");
    let limited_post_records = posts
        .limit(1)
        .load::<Post>(connection)
        .expect("DB query error");

    limited_post_records
        .iter()
        .for_each(|post| println!("{:?}", post));

    // Select with specific columns
    println!("Select ALL posts but only \'title\' and \'body\' columns");
    let simple_post_records = posts
        .select((title, body))
        .load::<PostSimple>(connection)
        .expect("DB query err");

    simple_post_records
        .iter()
        .for_each(|post| println!("{:?}", post));

    // Select with condition
    println!("Select posts using WHERE clause");
    let where_post_records = posts
        .filter(id.eq(6))
        .select((title, body))
        .load::<PostSimple>(connection)
        .expect("DB query err");

    where_post_records
        .iter()
        .for_each(|post| println!("{:?}", post));
}
