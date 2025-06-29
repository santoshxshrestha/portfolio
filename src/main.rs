#![allow(unused)]
use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use sqlx::pool;
use sqlx::postgres::PgPoolOptions;
use std::env;
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
pub struct Project {
    repos: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>, // some repos may not have descriptions
    pub html_url: String,
    pub stargazers_count: i32,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommit {
    pub commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
pub struct CommitDetails {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectList {
    projects: Vec<ProjectName>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectName {
    name: String,
}

pub fn parsing_toml(path: &Path) -> Result<ProjectList, Box<dyn Error>> {
    let toml_str = fs::read_to_string(path)?;
    let data: ProjectList = toml::from_str(&toml_str)?;
    Ok(data)
}

#[derive(Debug, Deserialize, std::clone::Clone)]
pub struct RepoStats {
    pub name: String,
    pub description: Option<String>, // some repos may not have descriptions
    pub html_url: String,
    pub updated_at: String,
    pub stargazers_count: i32,
}

pub async fn get_project() -> Result<Vec<RepoStats>, reqwest::Error> {
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
        .json::<Vec<RepoStats>>()
        .await?;
    Ok(response)
}

pub async fn get_repo(matched_projects: Vec<RepoStats>) -> Result<Vec<Repo>, reqwest::Error> {
    let username = "santoshxshrestha";
    let mut repos: Vec<Repo> = Vec::new();

    dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");

    for project in matched_projects {
        let url = format!(
            "https://api.github.com/repos/{}/{}/commits?per_page=5",
            username, project.name
        );

        let client = reqwest::Client::new();

        let github_commits_result = client
            .get(&url)
            .header("User-Agent", "rust-script")
            .header("Authorization", format!("token {}", token))
            .send()
            .await;

        let commits = match github_commits_result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<GitHubCommit>>().await {
                        Ok(github_commits) => github_commits
                            .into_iter()
                            .map(|git_hub_commit| Commit {
                                message: git_hub_commit.commit.message,
                            })
                            .collect(),
                        Err(_) => {
                            println!("Failed to parse commits for repo: {}", project.name);
                            Vec::new()
                        }
                    }
                } else {
                    println!(
                        "API request failed for repo: {} with status: {}",
                        project.name,
                        response.status()
                    );
                    Vec::new()
                }
            }
            Err(e) => {
                println!("Network error for repo {}: {}", project.name, e);
                Vec::new()
            }
        };

        repos.push(Repo {
            name: project.name,
            description: project.description,
            html_url: project.html_url,
            stargazers_count: project.stargazers_count,
            commits,
        });
    }
    Ok(repos)
}

#[get("/projects")]
pub async fn projects() -> Result<impl Responder, actix_web::Error> {
    let response = get_project()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let project_list = parsing_toml(&Path::new("data/projects.toml"))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    ();
    let matched_projects: Vec<_> = response
        .into_iter()
        .filter_map(|repo| {
            project_list
                .projects
                .iter()
                .find(|project| repo.name == project.name)
                .map(|_| repo)
        })
        .collect();

    let repo = get_repo(matched_projects)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let template = Project { repos: repo };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct About;

#[get("/about")]
pub async fn about() -> impl Responder {
    let template = About;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct Blog {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    id: i32,
    content: String,
    time_stamp: NaiveDateTime,
}

#[get("/blog")]
async fn blog(pool: web::Data<sqlx::PgPool>) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query!(
        r#"
        select id,content 
        from blog
        order by id desc
        "#
    )
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let content: Vec<Message> = rows
        .into_iter()
        .map(|row| Message {
            id: row.id,
            content: row.content,
            time_stamp: row.time_stamp,
        })
        .collect();

    let template = Blog { messages: content };
    let body = template
        .render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(projects)
            .service(about)
            .service(blog)
            .service(Files::new("/static", "./static").show_files_listing())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
