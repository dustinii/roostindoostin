use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web_actors::ws;

use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

// User model
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password_hash: String,
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

// Registration handler
async fn register_user(user: web::Json<User>) -> impl Responder {
    let password_hash = hash_password(&user.password_hash);
    let new_user = User {
        username: user.username.clone(),
        email: user.email.clone(),
        password_hash,
    };

    let mut users = USERS.lock().unwrap();
    users.insert(new_user.username.clone(), new_user);

    HttpResponse::Ok().body("User registered successfully")
}

// Login handler
async fn login_user(credentials: web::Json<User>) -> impl Responder {
    let users = USERS.lock().unwrap();
    match users.get(&credentials.username) {
        Some(user) if verify_password(&user.password_hash, &credentials.password_hash) => {
            HttpResponse::Ok().body("Logged in successfully")
        },
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

// Define a WebSocket actor for handling incoming WebSocket messages
struct ChatWebSocket;

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

// Implement StreamHandler to handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text), // Echo the text received
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin), // Echo binary data
            Ok(ws::Message::Close(_)) => ctx.stop(), // Stop the actor
            _ => (),
        }
    }
}

// WebSocket route handler
async fn chat_ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ChatWebSocket {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/ws/", web::get().to(chat_ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}