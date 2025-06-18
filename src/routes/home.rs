use actix_web::{HttpResponse, Responder, get};
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    // rustacean: String,
}

#[get("/")]
pub async fn home() -> impl Responder {
    let template = Home {
        // rustacean: "Santosh".to_string(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
