#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::middleware::Logger;
use actix_web::{error, get, http, post, web, App, Error, HttpResponse, HttpServer, Result};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use session::FlashMessage;
use tera::Tera;

use crate::db::poo::*;
use crate::db::poo_bleeding::*;
use crate::db::poo_color::*;
use crate::db::poo_form::*;

mod db;
mod schema;
mod session;
#[cfg(test)]
mod tests;

static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

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
    session: Session,
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

    if let Some(flash) = session::get_flash(&session)? {
        context.insert("msg", &(flash.kind, flash.message));
        session::clear_flash(&session);
    }

    let s = tmpl
        .render("index.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/poo")]
async fn insert_poo(
    params: web::Form<PooInsertForm>,
    pool: web::Data<DbPool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    info!("{:?}", params);
    session::set_flash(
        &session,
        if Poo::insert(&conn, params.into()) {
            FlashMessage::success("New poo data was added")
        } else {
            FlashMessage::error("Failed to add new poo data")
        },
    )?;

    Ok(redirect_to("/"))
}

// Use `post` method instead of `delete` one because
// - Dealing with `delete` method requires manual implement
// - There are no plans to support the other requests
#[post("/poo/{pooid}")]
async fn delete_poo(
    web::Path(pooid): web::Path<i32>,
    pool: web::Data<DbPool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    info!("delete ID {}", pooid);
    // TODO: Even if no record is deleted, `delete_with_id` returns `true`
    session::set_flash(
        &session,
        if Poo::delete_with_id(&conn, pooid) {
            FlashMessage::success("Your poo data was deleted")
        } else {
            FlashMessage::error("Failed to delete you poo data")
        },
    )?;

    Ok(redirect_to("/"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || App::new().configure(app_config).wrap(Logger::default()))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

fn app_config(config: &mut web::ServiceConfig) {
    dotenv().ok();
    let mut connspec = std::env::var("DATABASE_URL").expect("env `DATABASE_URL` is empty");
    if cfg!(test) {
        connspec.push_str("_test");
    }
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    // No `unwrap()` error because there is the `static/` directory
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/*")).unwrap();
    let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);

    config.service(
        web::scope("")
            .data(pool)
            .data(tera)
            .wrap(session_store)
            .service(index)
            .service(insert_poo)
            .service(delete_poo)
            .service(
                fs::Files::new("/css", concat!(env!("CARGO_MANIFEST_DIR"), "/static/css"))
                    .show_files_listing(),
            ),
    );
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
