#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use serde::Serialize;

#[derive(Serialize)]
struct Project {
    title: String,
    description: String,
    image_path: String,
}

#[derive(FromForm)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Home",
        name: "Santosh",
    })
}

#[get("/projects")]
fn projects() -> Template {
    let projects = vec![
        Project {
            title: "Project One".to_string(),
            description: "Description of your first project".to_string(),
            image_path: "/static/images/project1.jpg".to_string(),
        },
        Project {
            title: "Project Two".to_string(),
            description: "Description of your second project".to_string(),
            image_path: "/static/images/project2.jpg".to_string(),
        },
    ];

    Template::render("projects", context! {
        title: "My Projects",
        projects: projects,
    })
}

#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", context! {
        title: "Contact Me",
    })
}

#[post("/contact", data = "<form>")]
fn submit_contact(form: Form<ContactForm>) -> Redirect {
    println!("Contact form submitted: {} - {}", form.name, form.email);
    
    Redirect::to("/contact?success=true")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, projects, contact, submit_contact])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}

