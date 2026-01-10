// ============================================================
// src/templates/auth.rs
// ============================================================
pub fn login_page() -> String {
    r#"
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
                margin-top: 0.5rem;
            }
            button:hover {
                background-color: #0056b3;
            }
            .register-link {
                text-align: center;
                margin-top: 1rem;
            }
            .register-link a {
                color: #007bff;
                text-decoration: none;
            }
            .register-link a:hover {
                text-decoration: underline;
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
            <div class="register-link">
                Don't have an account? <a href="/register">Register</a>
            </div>
        </div>
    </body>
    </html>
    "#.to_string()
}

pub fn register_page() -> String {
    r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Register</title>
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
            .register-container {
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
                background-color: #28a745;
                color: white;
                border: none;
                border-radius: 4px;
                cursor: pointer;
                font-size: 1rem;
                margin-top: 0.5rem;
            }
            button:hover {
                background-color: #218838;
            }
            .login-link {
                text-align: center;
                margin-top: 1rem;
            }
            .login-link a {
                color: #007bff;
                text-decoration: none;
            }
            .login-link a:hover {
                text-decoration: underline;
            }
        </style>
    </head>
    <body>
        <div class="register-container">
            <h2>Register</h2>
            <form method="post" action="/register">
                <input type="text" name="username" placeholder="Username" required>
                <input type="email" name="email" placeholder="Email" required>
                <input type="password" name="password" placeholder="Password" required>
                <button type="submit">Register</button>
            </form>
            <div class="login-link">
                Already have an account? <a href="/">Login</a>
            </div>
        </div>
    </body>
    </html>
    "#.to_string()
}