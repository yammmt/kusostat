#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use tera::Tera;

use crate::db::poo::Poo;

mod db;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    let poos = Poo::all(&conn);

    let mut context = tera::Context::new();
    context.insert("poos", &poos);

    let s = tmpl
        .render("index.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read `.env` file");
    let connspec = std::env::var("DATABASE_URL").expect("env `DATABASE_URL` is empty");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        // No `unwrap()` error because there is the `static/` directory
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/*")).unwrap();

        App::new()
            .data(pool.clone())
            .data(tera)
            .service(index)
            .service(
                fs::Files::new("/css", concat!(env!("CARGO_MANIFEST_DIR"), "/static/css"))
                    .show_files_listing(),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
