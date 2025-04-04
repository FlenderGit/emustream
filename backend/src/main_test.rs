use std::sync::Arc;

use api::auth;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use dotenv;
use log::info;
use route::create_router;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::net::TcpListener;
use uuid::Uuid;

mod api;
mod error;
mod middleware;
mod route;

#[macro_use]
mod macros;

pub struct AppContext {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("[^] Emustream Rest API Service");

    auth::init_global_secret();

    /* let filter_level = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| {
        if (cfg!(debug_assertions)) {
            tracing_subscriber::EnvFilter::new("debug")
        } else {
            tracing_subscriber::EnvFilter::new("info")
        }
    }); */

    let level_max = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level_max)
        .with_target(false)
        //.with_env_filter(filter_level)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("[✓] Connection to the database is successful!");
            pool
        }
        Err(err) => {
            log::error!("[x] Failed to connect to the database: {}", err.to_string());
            std::process::exit(1);
        }
    };

    let app_ctx = AppContext { db: pool.clone() };

    let app_ctx = Arc::new(app_ctx);
    let app = create_router(app_ctx.clone())
        .layer(axum::middleware::from_fn(last_middleware))
        .layer(axum::middleware::from_fn_with_state(
            app_ctx.clone(),
            middleware::middleware_tracer,
        ))
        .layer(axum::middleware::from_fn(middleware_response_time));

    let address = "127.0.0.1:8080";

    println!("[✓] Server started successfully at http://{}", address);

    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn middleware_response_time(request: Request, next: Next) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let mut response = next.run(request).await;
    let duration = start.elapsed();

    response.headers_mut().insert(
        "X-Response-Time",
        format!("{}ms", duration.as_millis()).parse().unwrap(),
    );

    response
}

async fn last_middleware(request: Request, next: Next) -> impl IntoResponse {
    let uuid = request.extensions().get::<Uuid>().unwrap().clone();

    let response = next.run(request).await;

    if response.status().is_client_error() || response.status().is_server_error() {
        log::error!("[Error]: {} ({uuid})", response.status());
    }

    response
}
