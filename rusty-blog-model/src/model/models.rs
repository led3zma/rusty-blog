use crate::model::schema::posts;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct PostSimple {
    pub title: String,
    pub body: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub slug: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

impl NewPost {
    pub fn new(title: String, body: String) -> NewPost {
        let slug = Self::slugify(title.clone());
        NewPost { title, slug, body }
    }

    fn slugify(title: String) -> String {
        title.replace(" ", "-").to_lowercase()
    }
}
