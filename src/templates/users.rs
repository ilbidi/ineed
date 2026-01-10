// ============================================================
// src/templates/users.rs
// ============================================================
use crate::models::User;

pub fn users_page(users: &[User]) -> String {
    let mut users_html = String::new();
    for user in users {
        users_html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            user.id, user.username, user.email
        ));
    }

    format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>User Management</title>
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
            table {{
                width: 100%;
                background-color: white;
                border-collapse: collapse;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }}
            th, td {{
                padding: 1rem;
                text-align: left;
                border-bottom: 1px solid #ddd;
            }}
            th {{
                background-color: #007bff;
                color: white;
            }}
            .back-link {{
                display: inline-block;
                margin-bottom: 1rem;
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
            <h1>Dashboard - User Management</h1>
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
                <a href="/dashboard" class="back-link">← Back to Dashboard</a>
                <h2>User Management</h2>
                <table>
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Username</th>
                            <th>Email</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
        </div>
    </body>
    </html>
    "#, users_html)
}
