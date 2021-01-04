#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_web::{App, Error, HttpResponse, HttpServer, Result, error, get, http, post, web};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::db::poo::*;
use crate::db::poo_bleeding::*;
use crate::db::poo_color::*;
use crate::db::poo_form::*;

mod db;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Serialize)]
struct PooInsertForm {
    pub form: i32,
    pub color: i32,
    pub bleeding: i32,
    pub required_time: String,
    pub published_at: String,
}

impl From<actix_web::web::Form<PooInsertForm>> for db::poo::RawPoo {
    // Uses dummy date/time if parse fails
    fn from(params: actix_web::web::Form<PooInsertForm>) -> Self {
        let default_rt = NaiveTime::from_hms(23, 59, 59);
        let required_time = match params.required_time.len() {
            5 => NaiveTime::parse_from_str(&params.required_time, "%H:%M").unwrap_or(default_rt),
            8 => NaiveTime::parse_from_str(&params.required_time, "%H:%M:%S").unwrap_or(default_rt),
            _ => default_rt,
        };

        let default_pa = NaiveDate::from_ymd(2099, 12, 31).and_hms(23, 59, 59);
        let published_at = match params.published_at.len() {
            16 => NaiveDateTime::parse_from_str(&params.published_at, "%Y-%m-%dT%H:%M")
                .unwrap_or(default_pa),
            _ => default_pa,
        };

        RawPoo {
            form: params.form,
            color: params.color,
            bleeding: params.bleeding,
            required_time,
            published_at,
        }
    }
}

#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    let poos = Poo::all(&conn);
    let poo_forms = PooForm::all(&conn);
    let poo_colors = PooColor::all(&conn);
    let poo_bleedings = PooBleeding::all(&conn);

    let mut context = tera::Context::new();
    context.insert("poos", &poos);
    context.insert("poo_forms", &poo_forms);
    context.insert("poo_colors", &poo_colors);
    context.insert("poo_bleedings", &poo_bleedings);

    let s = tmpl
        .render("index.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/poo")]
async fn insert_poo(
    params: web::Form<PooInsertForm>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    Poo::insert(&conn, params.into());

    // TODO: Show flush message
    Ok(redirect_to("/"))
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
            .service(insert_poo)
            .service(
                fs::Files::new("/css", concat!(env!("CARGO_MANIFEST_DIR"), "/static/css"))
                    .show_files_listing(),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
