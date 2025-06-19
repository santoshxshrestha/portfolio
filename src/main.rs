use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home;

#[get("/")]
pub async fn home() -> impl Responder {
    let template = Home;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct Projects;

#[get("/projects")]
pub async fn projects() -> impl Responder {
    let template = Projects;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(projects)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
