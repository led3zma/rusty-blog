use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use rusty_blog_model::{get_db_pool, model::schema::posts::dsl::*};
use rusty_blog_model::{model::models::Post, DbPool};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hola desde Actix")
}

#[get("/posts")]
async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("DB get error");

    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => HttpResponse::Ok().body(format!("Hola desde Actix: \n{:?}", data)),
        Err(err) => HttpResponse::NotFound().body(format!("No consiguió: \n{:?}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = get_db_pool();

    HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .service(get_posts)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 9900))?
    .run()
    .await
}
