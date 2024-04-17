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
        .values(NewPost::new_by_handler(new_post))
        .get_result(&mut connection)
}

pub fn select_all(mut connection: DbConn) -> Result<Vec<Post>, Error> {
    posts.load::<Post>(&mut connection)
}

pub fn select_one_post(mut connection: DbConn) -> Result<Vec<Post>, Error> {
    posts.limit(1).load::<Post>(&mut connection)
}

pub fn select_simple(mut connection: DbConn) -> Result<Vec<PostSimple>, Error> {
    posts
        .select((title, body))
        .load::<PostSimple>(&mut connection)
}

pub fn select_by_id(mut connection: DbConn, search_id: i32) -> Result<Vec<Post>, Error> {
    posts.filter(id.eq(search_id)).load::<Post>(&mut connection)
}

pub fn select_by_slug(mut connection: DbConn, search_slug: &str) -> Result<Vec<Post>, Error> {
    posts
        .filter(slug.like(search_slug))
        .load::<Post>(&mut connection)
}

pub fn update_by_id(
    mut connection: DbConn,
    search_id: i32,
    updated_post: &NewPostHandler,
) -> Result<Post, Error> {
    diesel::update(posts.filter(id.eq(search_id)))
        .set(NewPost::new_by_handler(updated_post))
        .get_result::<Post>(&mut connection)
}

pub fn update_by_slug(
    mut connection: DbConn,
    search_slug: &str,
    updated_post: &NewPostHandler,
) -> Result<Post, Error> {
    diesel::update(posts.filter(slug.like(search_slug)))
        .set(NewPost::new_by_handler(updated_post))
        .get_result::<Post>(&mut connection)
}

pub fn delete_by_id(mut connection: DbConn, search_id: i32) -> Result<usize, Error> {
    diesel::delete(posts.filter(id.eq(search_id))).execute(&mut connection)
}

pub fn delete_by_slug(mut connection: DbConn, search_slug: &str) -> Result<usize, Error> {
    diesel::delete(posts.filter(slug.like(search_slug))).execute(&mut connection)
}
