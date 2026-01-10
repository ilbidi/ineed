// ============================================================
// src/handlers/dashboard.rs
// ============================================================
use actix_web::{HttpResponse, Result};
use actix_session::Session;
use crate::templates;

pub async fn dashboard(session: Session) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {
        let html = templates::dashboard::dashboard_page(&username);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish())
    }
}