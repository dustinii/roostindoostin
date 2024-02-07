#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix::{prelude::*, Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use bcrypt::{hash, verify};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

// Use the User struct from models
use crate::models::{NewUser, User};
use crate::schema::users;

// Function to establish a connection to the PostgreSQL database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
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
async fn register_user(user_data: web::Json<NewUser>) -> impl Responder {
    let conn = establish_connection();

    // Hash the password
    let hashed_password = hash_password(&user_data.password_hash);

    // Create a new user
    let new_user = NewUser {
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
    };

    // Insert new user into the database
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
    {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating user")
        }
    }
}

// Login handler
async fn login_user(_credentials: web::Json<User>) -> impl Responder {
    let _conn = establish_connection();
    // Add logic to verify the user credentials from the database
    HttpResponse::Ok().body("Logged in successfully")
}

// Handler for GET request to fetch all users
async fn get_all_users() -> impl Responder {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let user_list = users
        .load::<User>(&connection)
        .expect("Error loading users");

    let user_info: Vec<_> = user_list
        .iter()
        .map(|user| (user.username.clone(), user.email.clone()))
        .collect();

    HttpResponse::Ok().json(user_info)
}

// WebSocket route handler
async fn chat_ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ChatWebSocket {}, &req, stream)
}

// Define a WebSocket actor for handling incoming WebSocket messages
struct ChatWebSocket;

// Implement Actor trait for ChatWebSocket
impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

// Implement StreamHandler to handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => (),
        }
    }
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connected");
    }
    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket disconnected");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_all_users))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/ws/", web::get().to(chat_ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
