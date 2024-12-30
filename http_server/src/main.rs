use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::sync::Mutex;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

// This struct represents state
struct AppState {
    app_name: String,
    counter: Mutex<i32>, // Mutex is necessary to mutate safely across threads
}

// POST handler
#[post("/{domain}/{model}/{action}")]
async fn echo(
    path: web::Path<(String, String, String)>, // Match three path parameters
    data: web::Data<AppState>,                // Shared application state
    req_body: String,                         // Extract raw request body
) -> impl Responder {
    let app_name = &data.app_name; // Access application state
    let (domain, model, action) = path.into_inner(); // Deconstruct path parameters

    HttpResponse::Ok().body(format!(
        "{app_name} - Domain: {domain} | Model: {model} | Action: {action} | Body: {req_body}"
    ))
}

// GET handler
#[get("/{domain}/{model}/{action}")]
async fn index(path: web::Path<(String, String, String)>) -> impl Responder {
    let (domain, model, action) = path.into_inner(); // Deconstruct path parameters

    HttpResponse::Ok().body(format!("Domain: {domain} | Model: {model} | Action: {action}"))
}


// Main application
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created outside HttpServer::new closure
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: Mutex::new(0),
    });

    // Print the server address
    println!("################################################################################");
    println!("Server running at — http://{}:{}", HOST, PORT);
    println!("################################################################################");

    HttpServer::new(move || {
        // Move app_state into the closure
        App::new()
            .app_data(app_state.clone()) // Register the created data
            .service(echo)
            .service(index)
    })
    .bind((HOST, PORT))? // Bind the server to the address
    .run()
    .await
}
