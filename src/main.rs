use actix_files as fs;
use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer, Result};
use tera::Tera;

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // No `unwrap()` error because there is the `static/` directory
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/*")).unwrap();

        App::new().data(tera).service(index).service(
            fs::Files::new("/css", concat!(env!("CARGO_MANIFEST_DIR"), "/static/css"))
                .show_files_listing(),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
