#![allow(unused)]
use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::web::Form;
use actix_web::{HttpResponse, Responder, get, post};
use askama::Template;
use dotenv::dotenv;
use pulldown_cmark::{Options, Parser, html};
use reqwest;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::PrimitiveDateTime;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;
use tokio::time::Instant;

#[derive(Debug, Clone)]
struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            timestamp: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
}

type Cache<T> = Arc<Mutex<HashMap<String, CacheEntry<T>>>>;

pub struct CacheManager {
    repo_stats_cache: Cache<Vec<RepoStats>>,
    repo_cache: Cache<Vec<Repo>>,
}

impl CacheManager {
    fn new() -> Self {
        Self {
            repo_stats_cache: Arc::new(Mutex::new(HashMap::new())),
            repo_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_repo_stats(&self, key: &str) -> Option<Vec<RepoStats>> {
        let cache = self.repo_stats_cache.lock().unwrap();
        cache.get(key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(entry.data.clone())
            }
        })
    }

    fn set_repo_stats(&self, key: String, data: Vec<RepoStats>, ttl: Duration) {
        let mut cache = self.repo_stats_cache.lock().unwrap();
        cache.insert(key, CacheEntry::new(data, ttl));
    }

    fn get_repos(&self, key: &str) -> Option<Vec<Repo>> {
        let cache = self.repo_cache.lock().unwrap();
        cache.get(key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(entry.data.clone())
            }
        })
    }

    fn set_repos(&self, key: String, data: Vec<Repo>, ttl: Duration) {
        let mut cache = self.repo_cache.lock().unwrap();
        cache.insert(key, CacheEntry::new(data, ttl));
    }

    fn cleanup(&self) {
        {
            let mut cache = self.repo_stats_cache.lock().unwrap();
            cache.retain(|_, entry| !entry.is_expired());
        }
        {
            let mut cache = self.repo_cache.lock().unwrap();
            cache.retain(|_, entry| !entry.is_expired());
        }
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home;

#[derive(Template, Deserialize, Debug)]
#[template(path = "projects.html")]
pub struct Project {
    repos: Vec<Repo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: i32,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct RepoStats {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub updated_at: String,
    pub stargazers_count: i32,
}

pub async fn get_project_cached(cache: &CacheManager) -> Result<Vec<RepoStats>, reqwest::Error> {
    let cache_key = "user_repos".to_string();

    if let Some(cached_data) = cache.get_repo_stats(&cache_key) {
        println!("Cache hit for repository stats");
        return Ok(cached_data);
    }

    println!("Cache miss for repository stats, fetching from API");

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

    cache.set_repo_stats(cache_key, response.clone(), Duration::from_secs(600));

    Ok(response)
}

pub async fn get_repo_cached(
    matched_projects: Vec<RepoStats>,
    cache: &CacheManager,
) -> Result<Vec<Repo>, reqwest::Error> {
    let cache_key = format!(
        "repos_{}",
        matched_projects
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join("_")
    );

    if let Some(cached_data) = cache.get_repos(&cache_key) {
        println!("Cache hit for repository details");
        return Ok(cached_data);
    }

    println!("Cache miss for repository details, fetching from API");

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

    cache.set_repos(cache_key, repos.clone(), Duration::from_secs(300));

    Ok(repos)
}

#[get("/")]
pub async fn home() -> impl Responder {
    let template = Home;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub fn parsing_toml(path: &Path) -> Result<ProjectList, Box<dyn Error>> {
    let toml_str = fs::read_to_string(path)?;
    let data: ProjectList = toml::from_str(&toml_str)?;
    Ok(data)
}

#[get("/projects")]
pub async fn projects(cache: web::Data<CacheManager>) -> Result<impl Responder, actix_web::Error> {
    let response = get_project_cached(&cache)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let project_list = parsing_toml(&Path::new("data/projects.toml"))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

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

    let repo = get_repo_cached(matched_projects, &cache)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let template = Project { repos: repo };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

async fn cache_cleanup_task(cache: web::Data<CacheManager>) {
    let mut interval = tokio::time::interval(Duration::from_secs(300)); // Run every 5 minutes
    loop {
        interval.tick().await;
        cache.cleanup();
        println!("Cache cleanup completed");
    }
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
    messages: Vec<BlogCard>,
}

#[derive(Debug)]
struct BlogCard {
    id: i32,
    title: String,
    excerpt: String,
    created_at: PrimitiveDateTime,
    views: i32,
}

#[get("/blog")]
async fn blog(pool: web::Data<sqlx::PgPool>) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query!(
        r#"
        select id,title, excerpt,views, created_at
        from blog
        order by id desc
        "#
    )
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let content: Vec<BlogCard> = rows
        .into_iter()
        .map(|row| BlogCard {
            id: row.id,
            title: row.title,
            excerpt: row.excerpt,
            created_at: row.created_at,
            views: row.views,
        })
        .collect();

    let template = Blog { messages: content };
    let body = template
        .render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    id: i32,
    title: String,
    content: String,
    excerpt: String,
}

#[get("/admin")]
async fn admin(pool: web::Data<sqlx::PgPool>) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query!(
        r#"
        select id,title, content, excerpt, created_at 
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
            title: row.title,
            content: row.content,
            excerpt: row.excerpt,
        })
        .collect();

    let template = AdminTemplate { messages: content };
    let body = template
        .render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
struct NewMessage {
    title: String,
    content: String,
    excerpt: String,
}

#[post("/send")]
async fn send_message(
    pool: web::Data<sqlx::PgPool>,
    form: Form<NewMessage>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!(
        "INSERT INTO blog (title,  content, excerpt) VALUES ($1, $2, $3 )",
        form.title,
        form.content,
        form.excerpt,
    )
    .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin"))
        .finish())
}

#[derive(Deserialize)]
struct DeleteForm {
    id: i32,
}

#[post("/delete")]
async fn delete_message(
    pool: web::Data<sqlx::PgPool>,
    form: Form<DeleteForm>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!("DELETE FROM blog WHERE id = $1", form.id)
        .execute(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin"))
        .finish())
}

#[derive(Template)]
#[template(path = "blog_detail.html")]
struct BlogDetailTemplate {
    title: String,
    created_at: String,
    content: String,
    excerpt: String,
}

#[get("/blog/{id}")]
async fn blog_detail(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    sqlx::query!("update blog set views = views + 1 where id = $1", id)
        .execute(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let row = sqlx::query!(
        "SELECT title, content, created_at,excerpt FROM blog WHERE id = $1",
        id
    )
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let parser = Parser::new_ext(&row.content, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let template = BlogDetailTemplate {
        title: row.title,
        created_at: row.created_at.to_string(),
        content: html_output,
        excerpt: row.excerpt,
    };

    let body = template.render().unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
struct Views {
    id: i32,
}

#[post("/blog/views")]
async fn views(
    pool: web::Data<sqlx::PgPool>,
    form: Form<DeleteForm>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!("update blog set views = views + 1 where id = $1", form.id)
        .execute(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/blog/{id}"))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let cache_manager = web::Data::new(CacheManager::new());

    let cache_for_cleanup = cache_manager.clone();
    tokio::spawn(async move {
        cache_cleanup_task(cache_for_cleanup).await;
    });

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(projects)
            .service(about)
            .service(blog)
            .service(blog_detail)
            .service(Files::new("/static", "./static").show_files_listing())
            .app_data(web::Data::new(pool.clone()))
            .app_data(cache_manager.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
