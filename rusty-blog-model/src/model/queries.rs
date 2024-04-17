use crate::model::{
    models::{NewPost, Post, PostSimple},
    schema::posts::{self, dsl::*},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};

use super::models::NewPostHandler;

pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create(mut connection: DbConn, new_post: &NewPostHandler) -> Result<Post, Error> {
    diesel::insert_into(posts::table)
        .values(NewPost::new(new_post.title.clone(), new_post.body.clone()))
        .get_result(&mut connection)
}

pub fn select_all(mut connection: DbConn) -> Result<Vec<Post>, Error> {
    posts.load::<Post>(&mut connection)
}

pub fn select_one_post(connection: &mut PgConnection) -> Vec<Post> {
    posts
        .limit(1)
        .load::<Post>(connection)
        .expect("DB query error")
}

pub fn select_simple(connection: &mut PgConnection) -> Vec<PostSimple> {
    posts
        .select((title, body))
        .load::<PostSimple>(connection)
        .expect("DB query err")
}

pub fn select_by_id(connection: &mut PgConnection, search_id: i32) -> Vec<Post> {
    posts
        .filter(id.eq(search_id))
        .load::<Post>(connection)
        .expect("DB query err")
}

pub fn update(connection: &mut PgConnection, search_id: i32, updated_post: NewPost) -> Post {
    diesel::update(posts.filter(id.eq(search_id)))
        .set(updated_post)
        .get_result::<Post>(connection)
        .expect("DB update error")
}

pub fn delete_by_id(connection: &mut PgConnection, search_id: i32) -> usize {
    diesel::delete(posts.filter(id.eq(search_id)))
        .execute(connection)
        .expect("DB delete error")
}

pub fn delete_by_slug(connection: &mut PgConnection, search_slug: &str) -> usize {
    diesel::delete(posts.filter(slug.like(search_slug)))
        .execute(connection)
        .expect("DB delete error")
}
