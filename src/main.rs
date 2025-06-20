#![allow(unused)]
use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;
use dotenv::dotenv;
use reqwest::{self, Client};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

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

#[derive(Template, Deserialize, Debug)]
#[template(path = "projects.html")]
pub struct ProjectList {
    projects: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
    description: String,
    github_link: String,
    readme_link: String,
}

pub fn parsing_toml(path: &Path) -> Result<ProjectList, Box<dyn Error>> {
    let toml_str = fs::read_to_string(path)?;
    let data: ProjectList = toml::from_str(&toml_str)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>, // some repos may not have descriptions
    pub html_url: String,
    pub updated_at: String,
    pub stargazers_count: String,
}

pub async fn get_project() -> Result<Vec<Repo>, reqwest::Error> {
    let username = "santoshxshrestha";
    let url = format!(
        "https://api.github.com/users/{}/repos?per_page=100",
        username
    );

    dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-script")
        .header("Authorization", format!("token {}", token))
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;
    Ok(response)
}

#[get("/projects")]
pub async fn projects() -> Result<impl Responder, actix_web::Error> {
    let template = parsing_toml(&Path::new("data/projects.toml"))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
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
