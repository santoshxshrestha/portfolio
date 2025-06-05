use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    middleware::Logger,
    post,
    web::{self, route},
};
use env_logger;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Santosh")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{req_body}");
    HttpResponse::Ok().body(req_body)
}

#[get("/lol")]
async fn index() -> impl Responder {
    HttpResponse::Created().body("Item created lol")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
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
            .route("/hey", web::get().to(manual_hello))
            .service(hello)
            .service(echo)
            .service(index)
            .default_service(route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
