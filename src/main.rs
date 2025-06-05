use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware::Logger, post, web::route,
};

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
    title: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let template = HelloTemplate {
        name: "Santosh".to_string(),
        title: "kai hola".to_lowercase(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{req_body}");
    HttpResponse::Ok().body(req_body)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Oops! The page you are looking . That shit does not exist.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .default_service(route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
