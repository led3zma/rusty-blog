pub mod routes {
    use actix_web::{get, post, web, HttpResponse, Responder};

    use crate::model::models::NewPostHandler;
    use crate::model::queries;

    use diesel::r2d2::ConnectionManager;
    use diesel::{prelude::*, r2d2};

    pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

    #[get("/")]
    pub async fn index(template: web::Data<tera::Tera>) -> impl Responder {
        let ctx = tera::Context::new();
        HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render("index.html", &ctx).expect("Template Err"))
    }

    #[get("/posts")]
    pub async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
        let conn = pool.get().expect("DB get error");

        match web::block(move || queries::select_all(conn)).await {
            Ok(data) => HttpResponse::Ok().body(format!("Hola desde Actix: \n{:?}", data)),
            Err(err) => HttpResponse::NotFound().body(format!("No consiguió: \n{:?}", err)),
        }
    }

    #[post("/posts/new")]
    pub async fn new_post(
        pool: web::Data<DbPool>,
        item: web::Json<NewPostHandler>,
    ) -> impl Responder {
        let conn = pool.get().expect("DB get error");

        match web::block(move || queries::create(conn, &item)).await {
            Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
            Err(err) => HttpResponse::NotFound().body(format!("No consiguió: \n{:?}", err)),
        }
    }
}
