use std::sync::Arc;

use axum::Router;

use crate::AppContext;



pub fn get_games_router(app_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/games", axum::routing::get(get_all_games))
        .route("/games/{id}", axum::routing::get(get_game_by_id)) 
}

async fn get_all_games() -> &'static str {
    "List of all games"
}

async fn get_game_by_id() -> &'static str {
    "Game details by ID"
}