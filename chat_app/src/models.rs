use super::schema::{users, messages};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub user_id: i32,
    pub content: &'a str,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}