// ============================================================
// src/handlers/users.rs
// ============================================================
use actix_web::{web, HttpResponse, Result};
use actix_session::Session;
use crate::{db::AppState, models::User, templates};

pub async fn users_page(session: Session, data: web::Data<AppState>) -> Result<HttpResponse> {
    if session.get::<String>("username")?.is_none() {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish());
    }

    let conn = data.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, username, email FROM users").unwrap();
    let users: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let html = templates::users::users_page(&users);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}