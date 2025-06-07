use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, post, web};
use askama::Template;
use dotenvy::dotenv;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use serde::Deserialize;
use std::env;

// --- TEMPLATES AND DATA STRUCTURES (Unchanged) ---

// Home page template
#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate {}

// Projects page template
#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectTemplate {}

// Contact page template
#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {
    title: String,
    request: RequestContext,
}

// Request context for handling query parameters
#[derive(Debug)]
struct RequestContext {
    query: QueryParams,
}

#[derive(Debug, Default)]
struct QueryParams {
    success: bool,
}

// Contact form data structure
#[derive(Debug, Deserialize, Clone)] // Added Clone
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

impl ContactForm {
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name is required".to_string());
        }

        if self.email.trim().is_empty() {
            return Err("Email is required".to_string());
        }

        if !self.email.contains('@') {
            return Err("Please enter a valid email address".to_string());
        }

        if self.message.trim().len() < 10 {
            return Err("Message must be at least 10 characters long".to_string());
        }

        Ok(())
    }

    fn sanitize(&mut self) {
        self.name = self.name.trim().to_string();
        self.email = self.email.trim().to_lowercase();
        self.message = self.message.trim().to_string();
    }
}

// --- ROUTE HANDLERS ---

#[get("/")]
async fn home() -> impl Responder {
    let template = HelloTemplate {};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[get("/projects")]
async fn projects() -> impl Responder {
    let template = ProjectTemplate {};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[get("/contact")]
async fn contact(req: HttpRequest) -> impl Responder {
    let query_string = req.query_string();
    let success = query_string.contains("success=true");

    let template = ContactTemplate {
        title: "Contact".to_string(),
        request: RequestContext {
            query: QueryParams { success },
        },
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering template")
        }
    }
}

#[post("/contact")]
async fn contact_submit(form: web::Form<ContactForm>) -> Result<HttpResponse> {
    let mut contact_form = form.into_inner();

    // Validate form data
    if let Err(error_msg) = contact_form.validate() {
        println!("Form validation failed: {}", error_msg);
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body(format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Form Error</title>
                     <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
                    <style>
                        body {{ font-family: 'JetBrains Mono', monospace; background: #0d1117; color: #fff; padding: 2rem; display: flex; justify-content: center; align-items: center; min-height: 100vh; }}
                        .error-container {{ max-width: 600px; text-align: center; border: 1px solid #30363d; background: #161b22; padding: 2rem; border-radius: 12px; }}
                        .error-title {{ color: #CE422B; font-size: 2rem; margin-bottom: 1rem; }}
                        .error-message {{ color: #E85A4F; margin-bottom: 2rem; padding: 1rem; background: #21262d; border-radius: 8px; }}
                        .back-link {{ color: #6A5ACD; text-decoration: none; padding: 0.75rem 1.5rem; border: 1px solid #6A5ACD; border-radius: 8px; transition: all 0.3s ease; }}
                        .back-link:hover {{ background: #6A5ACD; color: white; }}
                    </style>
                </head>
                <body>
                    <div class="error-container">
                        <h1 class="error-title">❌ Form Submission Error</h1>
                        <div class="error-message">{}</div>
                        <a href="/contact" class="back-link">← Go Back</a>
                    </div>
                </body>
                </html>
                "#,
                error_msg
            )));
    }

    // Sanitize input data
    contact_form.sanitize();

    // Clone form data to move into the blocking thread
    let form_data = contact_form.clone();

    // Send email in a blocking thread to avoid blocking the server
    let email_result = web::block(move || send_email_notification(&form_data)).await;

    match email_result {
        Ok(Ok(_)) => println!("✅ Email sending process completed successfully."),
        Ok(Err(e)) => eprintln!("🔥 Email sending failed within the thread: {}", e),
        Err(e) => eprintln!("🔥 Failed to execute email sending thread: {}", e),
    }

    // Redirect to success page regardless of email outcome to ensure good UX
    // In a real-world app, you might queue the email and retry on failure
    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/contact?success=true"))
        .finish())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("Received request body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().content_type("text/html").body(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
    <style>
        body { 
            font-family: 'JetBrains Mono', monospace; 
            background: #0d1117; 
            color: #fff; 
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center; 
            min-height: 100vh;
            margin: 0;
        }
        .container { max-width: 600px; }
        h1 { color: #CE422B; font-size: 3rem; margin-bottom: 1rem; }
        p { color: #8b949e; margin-bottom: 2rem; font-size: 1.1rem; }
        .home-link { 
            color: #6A5ACD; 
            text-decoration: none; 
            padding: 0.75rem 1.5rem; 
            border: 1px solid #6A5ACD; 
            border-radius: 8px; 
            transition: all 0.3s ease;
        }
        .home-link:hover { background: #6A5ACD; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀 404</h1>
        <p>Oops! The page you're looking for doesn't exist in this Rust-powered universe.</p>
        <a href="/" class="home-link">← Back to Home</a>
    </div>
</body>
</html>
        "#,
    )
}

// --- EMAIL SENDING LOGIC (UPDATED) ---

fn send_email_notification(
    form: &ContactForm,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Attempting to send email...");
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER environment variable not set")?;
    let smtp_password =
        env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD environment variable not set")?;
    let to_email = env::var("TO_EMAIL").unwrap_or_else(|_| smtp_user.clone());

    let email = Message::builder()
        .from(smtp_user.parse()?)
        .reply_to(format!("{} <{}>", form.name, form.email).parse()?)
        .to(to_email.parse()?)
        .subject(format!("Portfolio Contact: {}", form.name))
        .body(format!(
            "New contact form submission:\n\nName: {}\nEmail: {}\n\nMessage:\n{}",
            form.name, form.email, form.message
        ))?;

    let creds = Credentials::new(smtp_user, smtp_password);

    // Use Gmail's SMTP server with TLS
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)?;
    println!("✅ Email notification dispatched successfully!");
    Ok(())
}

// --- MAIN FUNCTION (UPDATED) ---

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    println!("🚀 Starting Santosh Shrestha's Portfolio Server...");
    println!("🌐 Server running at: http://127.0.0.1:8080");
    println!("🦀 Built with Rust + Actix Web + Askama");

    HttpServer::new(|| {
        App::new()
            // Main routes
            .service(home)
            .service(projects)
            .service(contact)
            .service(contact_submit)
            .service(echo)
            // Static files
            .service(Files::new("/static", "./static"))
            // 404 handler
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
