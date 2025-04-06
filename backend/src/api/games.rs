use std::sync::Arc;

use axum::{extract::{Path, State}, Router};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

use crate::{AppContext, error::ApiResult, models::Game};

pub fn get_games_router(app_state: Arc<AppContext>) -> Router<Arc<AppContext>> {
    Router::new()
        .route("/", axum::routing::get(get_all_games))
        .route("/{id}", axum::routing::get(get_game_by_id))
        .with_state(app_state)
}

async fn get_all_games(State(app_state): State<Arc<AppContext>>) -> ApiResult<Vec<Game>> {
    let cursor = app_state.db.collection::<Game>("game").find(doc! {}).await?;
    let games = cursor.try_collect().await?;
    Ok(axum::Json(games))
}

async fn get_game_by_id(
    State(app_state): State<Arc<AppContext>>,
    Path(id): Path<String>,
) -> ApiResult<Option<Game>> {
    let game_id = id.parse::<ObjectId>().unwrap();
    let filter = doc! { "_id": game_id };
    let game = app_state.db.collection::<Game>("game").find_one(filter).await.unwrap();
    Ok(axum::Json(game))
}
