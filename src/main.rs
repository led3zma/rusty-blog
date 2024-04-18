use std::env;
use std::process::exit;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use tera::Tera;

use rusty_blog::controller::*;
use rusty_blog::get_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_db_pool().unwrap_or_else(|err| {
        eprintln!("Error en DB pool: {err}");
        exit(1);
    });

    HttpServer::new(move || {
        let tera = Tera::new(&format!(
            "{}/templates/**/*",
            env::var("TEMPLATES_DIR").unwrap_or(env!("CARGO_MANIFEST_DIR").to_string())
        ))
        .expect("Tera Err");
        App::new()
            .service(routes::index)
            .service(routes::get_posts)
            .service(routes::new_post)
            .service(routes::get_post)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("0.0.0.0", 9900))?
    .run()
    .await
}
