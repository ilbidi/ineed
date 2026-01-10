// ============================================================
// src/main.rs
// ============================================================
mod db;
mod models;
mod handlers;
mod templates;

use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Initializing database...");
    let conn = db::init_db().expect("Failed to initialize database");
    
    let app_state = web::Data::new(db::AppState {
        db: std::sync::Mutex::new(conn),
    });
    
    println!("Starting server at http://127.0.0.1:8080");
    
    let secret_key = Key::generate();
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
                .cookie_secure(false)
                .build()
            )
            .route("/", web::get().to(handlers::auth::login_page))
            .route("/register", web::get().to(handlers::auth::register_page))
            .route("/register", web::post().to(handlers::auth::register))
            .route("/login", web::post().to(handlers::auth::login))
            .route("/dashboard", web::get().to(handlers::dashboard::dashboard))
            .route("/dashboard/time", web::get().to(handlers::dashboard::time_page))
            .route("/dashboard/users", web::get().to(handlers::users::users_page))
            .route("/logout", web::post().to(handlers::auth::logout))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
