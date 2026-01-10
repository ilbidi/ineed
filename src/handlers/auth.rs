// ============================================================
// src/handlers/auth.rs
// ============================================================
use actix_web::{web, HttpResponse, Result};
use actix_session::Session;
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::params;
use crate::{db::AppState, models::{LoginForm, RegisterForm}, templates};

pub async fn login_page() -> Result<HttpResponse> {
    let html = templates::auth::login_page();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn register_page() -> Result<HttpResponse> {
    let html = templates::auth::register_page();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn register(
    form: web::Form<RegisterForm>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let password_hash = match hash(&form.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Error hashing password")),
    };

    let conn = data.db.lock().unwrap();
    match conn.execute(
        "INSERT INTO users (username, email, password_hash) VALUES (?1, ?2, ?3)",
        params![&form.username, &form.email, &password_hash],
    ) {
        Ok(_) => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish()),
        Err(_) => Ok(HttpResponse::BadRequest()
            .body("Username or email already exists")),
    }
}

pub async fn login(
    form: web::Form<LoginForm>,
    session: Session,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let conn = data.db.lock().unwrap();
    
    let result: Result<(String, String), rusqlite::Error> = conn.query_row(
        "SELECT username, password_hash FROM users WHERE username = ?1",
        params![&form.username],
        |row| Ok((row.get(0)?, row.get(1)?)),
    );

    match result {
        Ok((username, password_hash)) => {
            if verify(&form.password, &password_hash).unwrap_or(false) {
                session.insert("username", &username)?;
                Ok(HttpResponse::SeeOther()
                    .append_header(("Location", "/dashboard"))
                    .finish())
            } else {
                Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().body("Invalid credentials")),
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}
