use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::route};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
    title: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let template = HelloTemplate {
        name: "Santosh".to_string(),
        title: "Santosh".to_lowercase(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Oops! The page you are looking . That shit does not exist.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/static", "./static"))
            .default_service(route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
