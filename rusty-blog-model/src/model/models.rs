use crate::model::schema::posts;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Debug, Queryable)]
pub struct PostSimple {
    pub title: String,
    pub body: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}