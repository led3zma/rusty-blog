use std::process::exit;

use actix_web::{middleware::Logger, web, App, HttpServer};
use tera::Tera;

use rusty_blog::controller::*;
use rusty_blog::get_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_db_pool().unwrap_or_else(|err| {
        eprintln!("Error en DB pool: {err}");
        exit(1);
    });

    HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).expect("Tera Err");
        App::new()
            .service(routes::index)
            .service(routes::get_posts)
            .service(routes::new_post)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("0.0.0.0", 9900))?
    .run()
    .await
}
