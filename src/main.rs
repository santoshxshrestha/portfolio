use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
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
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(Files::new("/static", "./static").show_files_listing())
            .app_data(cache_manager.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
