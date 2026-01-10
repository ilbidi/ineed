use actix_web::{web, App, HttpResponse, HttpServer, Result, HttpRequest, cookie::Cookie};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use serde::Deserialize;
use chrono::Local;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

// Login page
async fn login_page() -> Result<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Login</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                margin: 0;
                background-color: #f0f0f0;
            }
            .login-container {
                background: white;
                padding: 2rem;
                border-radius: 8px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                width: 300px;
            }
            h2 { margin-top: 0; }
            input {
                width: 100%;
                padding: 0.5rem;
                margin: 0.5rem 0;
                border: 1px solid #ddd;
                border-radius: 4px;
                box-sizing: border-box;
            }
            button {
                width: 100%;
                padding: 0.7rem;
                background-color: #007bff;
                color: white;
                border: none;
                border-radius: 4px;
                cursor: pointer;
                font-size: 1rem;
            }
            button:hover {
                background-color: #0056b3;
            }
        </style>
    </head>
    <body>
        <div class="login-container">
            <h2>Login</h2>
            <form method="post" action="/login">
                <input type="text" name="username" placeholder="Username" required>
                <input type="password" name="password" placeholder="Password" required>
                <button type="submit">Login</button>
            </form>
            <p style="font-size: 0.9rem; color: #666; margin-top: 1rem;">
                Use username: <b>admin</b>, password: <b>password</b>
            </p>
        </div>
    </body>
    </html>
    "#;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

// Handle login
async fn login(form: web::Form<LoginForm>, session: Session) -> Result<HttpResponse> {
    // Simple authentication (in production, use proper password hashing)
    if form.username == "admin" && form.password == "password" {
        session.insert("username", &form.username)?;
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/dashboard"))
            .finish())
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
    }
}

// Dashboard
async fn dashboard(session: Session) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Dashboard</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 0;
                    background-color: #f5f5f5;
                }}
                .header {{
                    background-color: #007bff;
                    color: white;
                    padding: 1rem 2rem;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }}
                .container {{
                    display: flex;
                    height: calc(100vh - 60px);
                }}
                .content {{
                    flex: 1;
                    padding: 2rem;
                }}
                .menu {{
                    width: 250px;
                    background-color: white;
                    border-left: 1px solid #ddd;
                    padding: 1rem;
                }}
                .menu h3 {{
                    margin-top: 0;
                    color: #333;
                }}
                .menu ul {{
                    list-style: none;
                    padding: 0;
                }}
                .menu li {{
                    margin: 0.5rem 0;
                }}
                .menu a {{
                    text-decoration: none;
                    color: #007bff;
                    display: block;
                    padding: 0.5rem;
                    border-radius: 4px;
                }}
                .menu a:hover {{
                    background-color: #f0f0f0;
                }}
                .logout {{
                    background-color: white;
                    color: #007bff;
                    border: none;
                    padding: 0.5rem 1rem;
                    border-radius: 4px;
                    cursor: pointer;
                }}
                .logout:hover {{
                    background-color: #f0f0f0;
                }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Dashboard</h1>
                <form method="post" action="/logout">
                    <button class="logout" type="submit">Logout</button>
                </form>
            </div>
            <div class="container">
                <div class="content">
                    <h2>Welcome, {}!</h2>
                    <p>You are now logged in to the dashboard.</p>
                </div>
                <div class="menu">
                    <h3>Apps</h3>
                    <ul>
                        <li><a href="/dashboard/time">Current Time</a></li>
                    </ul>
                </div>
            </div>
        </body>
        </html>
        "#, username);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish())
    }
}

// Time page
async fn time_page(session: Session) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Current Time</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 0;
                    background-color: #f5f5f5;
                }}
                .header {{
                    background-color: #007bff;
                    color: white;
                    padding: 1rem 2rem;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }}
                .container {{
                    display: flex;
                    height: calc(100vh - 60px);
                }}
                .content {{
                    flex: 1;
                    padding: 2rem;
                }}
                .menu {{
                    width: 250px;
                    background-color: white;
                    border-left: 1px solid #ddd;
                    padding: 1rem;
                }}
                .menu h3 {{
                    margin-top: 0;
                    color: #333;
                }}
                .menu ul {{
                    list-style: none;
                    padding: 0;
                }}
                .menu li {{
                    margin: 0.5rem 0;
                }}
                .menu a {{
                    text-decoration: none;
                    color: #007bff;
                    display: block;
                    padding: 0.5rem;
                    border-radius: 4px;
                }}
                .menu a:hover {{
                    background-color: #f0f0f0;
                }}
                .time-display {{
                    background-color: white;
                    padding: 2rem;
                    border-radius: 8px;
                    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                    text-align: center;
                }}
                .time {{
                    font-size: 2rem;
                    color: #007bff;
                    font-weight: bold;
                }}
                .back-link {{
                    margin-top: 1rem;
                    display: inline-block;
                    color: #007bff;
                    text-decoration: none;
                }}
                .back-link:hover {{
                    text-decoration: underline;
                }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Dashboard - Current Time</h1>
                <form method="post" action="/logout">
                    <button style="background-color: white; color: #007bff; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;" type="submit">Logout</button>
                </form>
            </div>
            <div class="container">
                <div class="content">
                    <div class="time-display">
                        <h2>Current Time</h2>
                        <div class="time">{}</div>
                        <a href="/dashboard" class="back-link">← Back to Dashboard</a>
                    </div>
                </div>
                <div class="menu">
                    <h3>Apps</h3>
                    <ul>
                        <li><a href="/dashboard/time">Current Time</a></li>
                    </ul>
                </div>
            </div>
        </body>
        </html>
        "#, current_time);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish())
    }
}

// Logout
async fn logout(session: Session) -> Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    let secret_key = Key::generate();
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
                .cookie_secure(false)
                .build()
            )
            .route("/", web::get().to(login_page))
            .route("/login", web::post().to(login))
            .route("/dashboard", web::get().to(dashboard))
            .route("/dashboard/time", web::get().to(time_page))
            .route("/logout", web::post().to(logout))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}