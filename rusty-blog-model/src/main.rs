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

    let post_records = posts
        .filter(id.le(4))
        .load::<Post>(connection)
        .expect("DB query error");

    post_records.iter().for_each(|post| println!("{:?}", post))
}
