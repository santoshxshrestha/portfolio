#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn hello(tera: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();

    match tera.render("contact.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            println!("Template  error: {}", err);
            HttpResponse::InternalServerError().body("Error rendering template")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
