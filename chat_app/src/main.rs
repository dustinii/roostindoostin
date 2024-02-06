use actix_web::{web, App, HttpServer, Responder};
use bcrypt::{hash, verify};

struct User {
    username: String,
    password: String,
    password_hash: String,
}

// Dummy function to mimic database save
async fn save_user(user: User) -> Result<(), ()> {
    // Save user logic here
    Ok(())
}

async fn register_user(username: String, email: String, password: String) -> impl Responder {
    let password_hash = hash(password, 4).unwrap();
    let user = User { username, email, password_hash };

    match save_user(user).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
    }
}

async fn login_user(username: String, password: String) -> impl Responder {
    // Logic to verify user and password
    //"Login logic here"
}

async fn greet() -> impl Responder {
    "Hello, RUSTin!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}