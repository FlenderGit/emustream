use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    Json, Router,
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    AppContext,
    api::{
        auth::{handler_login, handler_me_claims},
        games::get_games_router,
    },
    middleware::middleware_request_time,
};

pub fn create_router(app_state: Arc<AppContext>) -> Router {
    let router_api = Router::new().nest("/games", get_games_router(app_state.clone()));

    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .route("/me", get(handler_me_claims))
        .route("/login", post(handler_login))
        .nest("/api", router_api)
        .layer(middleware::from_fn(middleware_request_time))
        .with_state(app_state)
}

#[derive(Serialize)]
enum Status {
    UP,
    DOWN,
}

pub async fn health_check_handler(State(app_state): State<Arc<AppContext>>) -> impl IntoResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let db_status = Status::DOWN;

    let json_response = serde_json::json!({
        "details": {
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Emustream Rest API Service",
            "database": "mysql"
        },
        "status": {
            "app": Status::UP,
            "database": db_status
        },
        "timestamp": timestamp
    });

    Json(json_response)
}
