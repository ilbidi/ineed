// ============================================================
// src/templates/dashboard.rs
// ============================================================
pub fn dashboard_page(username: &str) -> String {
    format!(r#"
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
            .menu {{
                width: 250px;
                background-color: white;
                border-right: 1px solid #ddd;
                padding: 1rem;
            }}
            .content {{
                flex: 1;
                padding: 2rem;
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
            <div class="menu">
                <h3>Apps</h3>
                <ul>
                    <li><a href="/dashboard/time">Current Time</a></li>
                    <li><a href="/dashboard/users">User Management</a></li>
                </ul>
            </div>
            <div class="content">
                <h2>Welcome, {}!</h2>
                <p>You are now logged in to the dashboard.</p>
            </div>
        </div>
    </body>
    </html>
    "#, username)
}

pub fn time_page(current_time: &str) -> String {
    format!(r#"
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
            .menu {{
                width: 250px;
                background-color: white;
                border-right: 1px solid #ddd;
                padding: 1rem;
            }}
            .content {{
                flex: 1;
                padding: 2rem;
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
            <div class="menu">
                <h3>Apps</h3>
                <ul>
                    <li><a href="/dashboard/time">Current Time</a></li>
                    <li><a href="/dashboard/users">User Management</a></li>
                </ul>
            </div>
            <div class="content">
                <div class="time-display">
                    <h2>Current Time</h2>
                    <div class="time">{}</div>
                    <a href="/dashboard" class="back-link">← Back to Dashboard</a>
                </div>
            </div>
        </div>
    </body>
    </html>
    "#, current_time)
}