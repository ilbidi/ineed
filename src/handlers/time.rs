// ============================================================
// src/handlers/time.rs
// ============================================================
use actix_web::{HttpResponse, Result};
use actix_session::Session;
use chrono::Local;
use crate::templates;

pub async fn time_page(session: Session) -> Result<HttpResponse> {
    if session.get::<String>("username")?.is_none() {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish());
    }

    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let html = templates::time::time_page(&current_time);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
