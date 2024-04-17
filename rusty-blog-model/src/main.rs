use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hola desde Actix")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db =
        ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DB URL NOT FOUND"));

    let pool = Pool::builder().build(db).expect("DB Pool err");

    HttpServer::new(move || App::new().service(hello_world).app_data(pool.clone()))
        .bind(("0.0.0.0", 9900))?
        .run()
        .await
}
