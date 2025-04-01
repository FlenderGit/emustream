use std::env;

use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string());
    let is_prod = env == "production";

    println!("Environment: {}", env);

    let mut app = axum::Router::new()
        //.route("/", axum::routing::get(root))
        .route("/api/hello", axum::routing::get(hello));

    if is_prod {
        app = app.fallback_service(
            ServeDir::new("/app/public").not_found_service(ServeFile::new("/app/public/200.html")),
        );
    }

    // let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let port = if is_prod { 80 } else { 3000 };
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello() -> &'static str {
    "Hello, Jean!"
}
