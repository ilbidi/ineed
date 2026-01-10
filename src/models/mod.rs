// ============================================================
// src/models/mod.rs
// ============================================================
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
