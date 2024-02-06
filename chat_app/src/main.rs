use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};


// User Model
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String, // Changed from password_hash to password
}

// Dummy in-memory database for demonstration
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

// Function to hash password
fn hash_password(password: &str) -> String {
    hash(password, 4).unwrap()
}

// Function to verify password
fn verify_password(hash: &str, password: &str) -> bool {
    verify(password, hash).unwrap()
}

async fn greet() -> impl Responder {
    "Hello, RUSTin!"
}

// Registration handler
async fn register_user(user: web::Json<User>) -> impl Responder {
    let password_hash = hash_password(&user.password); // Hashing the plain password
    let new_user = User {
        username: user.username.clone(),
        email: user.email.clone(),
        password: password_hash, // Use 'password' instead of 'password_hash'
    };

    let mut users = USERS.lock().unwrap();
    users.insert(new_user.username.clone(), new_user);

    HttpResponse::Ok().body("User registered successfully")
}

// Login handler
async fn login_user(credentials: web::Json<User>) -> impl Responder {
    let users = USERS.lock().unwrap();
    match users.get(&credentials.username) {
        Some(user) if verify_password(&user.password, &credentials.password) => { // Use 'password' here
            HttpResponse::Ok().body("Logged in successfully")
        },
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}