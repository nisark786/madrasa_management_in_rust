use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    let port = std::env::var("WEB_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!(%addr, "web server listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind web listener");
    axum::serve(listener, app)
        .await
        .expect("web server failure");
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn health() -> &'static str {
    "ok"
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Madrasa Student Management</title>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
    <link href="https://cdn.jsdelivr.net/npm/tailwindcss@3.3.0/tailwind.min.css" rel="stylesheet" />
    <style>
        * {
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
        }
        
        body {
            background: linear-gradient(to bottom right, #f8fafc, #f0f9ff, #f8fafc);
            color: #0f172a;
            margin: 0;
            padding: 0;
        }
        
        #app {
            min-height: 100vh;
        }
        
        button, a, [role="button"] {
            transition: all 200ms ease-in-out;
        }
        
        :focus-visible {
            outline: 2px solid #3b82f6;
            outline-offset: 2px;
        }
        
        ::-webkit-scrollbar {
            width: 8px;
            height: 8px;
        }
        
        ::-webkit-scrollbar-track {
            background: #f1f5f9;
        }
        
        ::-webkit-scrollbar-thumb {
            background: #cbd5e1;
            border-radius: 4px;
        }
        
        ::-webkit-scrollbar-thumb:hover {
            background: #94a3b8;
        }
        
        .animate-spin {
            animation: spin 1s linear infinite;
        }
        
        @keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }
    </style>
</head>
<body>
    <div id="app"></div>
    
    <script type="module">
        import init, { App } from '/js/madrasa_web.js';
        
        async function main() {
            await init();
            const root = document.getElementById('app');
            App().mount(root);
        }
        
        main().catch(console.error);
    </script>
</body>
</html>"#,
    )
}
