use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};

use crate::{api::{auth::{handler_login, handler_me_claims}, games::get_games_router}, AppContext};



pub fn create_router(app_state: Arc<AppContext>) -> Router {

    /* let me_router = Router::new()
        .route("/me", get(handler_me))
        .layer(middleware::from_fn(middleware_require_auth)); */

    let base_router = Router::new()
        .route("/login", post(handler_login))
        .route("/healthcheck", get(health_check_handler))
        .route("/me", get(handler_me_claims))
        .with_state(app_state.clone());

    Router::new()
        .nest("/api", get_games_router(app_state))
        .merge(base_router)
}

#[derive(Serialize)]
enum Status {
    UP,
    DOWN
}


pub async fn health_check_handler(
    State(app_state): State<Arc<AppContext>>
) -> impl IntoResponse {

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
