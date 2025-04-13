use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{Multipart, Path, Query, State},
    http::HeaderMap,
    routing::post,
};
use futures::{TryStreamExt, io::Cursor};
use log::info;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};

use crate::{
    AppContext,
    api::SortDirection,
    error::{ApiError, ApiResult},
    models::{Game, User},
    traits::CursorExt,
};

use super::{Pagination, auth::Claims};

pub fn get_games_router(app_state: Arc<AppContext>) -> Router<Arc<AppContext>> {
    Router::new()
        .route("/", axum::routing::get(get_all_games))
        .route("/homepage", axum::routing::get(get_data_homepage))
        .route("/upload-game", post(handler_upload_new_game))
        //.route("/upload-new-release", post(handler_upload_new_release))
        .route("/{id}", axum::routing::get(get_game_by_id))
        .with_state(app_state)
}
use crate::traits::MultipartExt;
async fn handler_upload_new_game(
    State(app_state): State<Arc<AppContext>>,
    mut multipart: Multipart,
) -> ApiResult<Game> {
    let (game, rom) = multipart.extract_game_and_save().await?;

    info!("Game uploaded: {:?}", game);
    info!("ROM size: {} bytes", rom.len());

    let game_dir = format!("/data/games/{}", game.slug);
    let path = std::path::Path::new(&game_dir);
    if !path.exists() {
        std::fs::create_dir_all(&path)
            .map_err(|e| ApiError::BadRequest(format!("Failed to create directory: {}", e)))?;
    }

    let release = game
        .releases
        .get(0)
        .ok_or_else(|| ApiError::BadRequest("No release found for the game".to_string()))?;

    if let Some(id) = release.id {
        let rom_path = format!("{}/{}", game_dir, id);
        std::fs::write(&rom_path, &rom)
            .map_err(|e| ApiError::BadRequest(format!("Failed to write file: {}", e)))?;
        info!("ROM saved to: {}", rom_path);
    } else {
        return Err(ApiError::BadRequest("Release ID is missing".to_string()));
    }

    app_state
        .db
        .collection::<Game>("game")
        .insert_one(&game)
        .await?;

    Ok(axum::Json(game))
}

async fn get_all_games(
    paginate: Query<Pagination>,
    State(app_state): State<Arc<AppContext>>,
) -> ApiResult<Vec<Game>> {
    let sort = match &paginate.sort_dir {
        Some(sort) => {
            let dir = if *sort == SortDirection::Desc { -1 } else { 1 };
            doc! { "title": dir }
        }
        None => doc! {},
    };

    let options = FindOptions::builder()
        .limit(paginate.limit)
        .skip(paginate.page)
        .sort(sort)
        .build();

    let filter_doc = match &paginate.query {
        Some(query) => doc! { "title": { "$regex": query, "$options": "i" } },
        None => doc! {},
    };

    let cursor = app_state
        .db
        .collection::<Game>("game")
        .find(filter_doc)
        .with_options(options)
        .await?;
    let games = cursor.try_collect().await?;
    Ok(axum::Json(games))
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HomepageResult {
    recent: Vec<String>,
    //popular: Vec<ObjectId>,
    recent_added: Vec<String>,
    data: HashMap<String, Game>,
}

async fn get_user_recent_games(
    collection_user: &mongodb::Collection<User>,
    user_id: ObjectId,
) -> Result<Vec<ObjectId>, ApiError> {
    let pipeline = vec![
        doc! { "$match": { "_id": user_id } },
        doc! { "$unwind": "$saves" },
        doc! { "$sort": { "saves.timestamp": -1 } },
        doc! { "$limit": 10 },
        doc! { "$group": { "_id": "$saves.game_id" } },
    ];
    collection_user
        .aggregate(pipeline)
        .await?
        .to_object_ids()
        .await
}

async fn get_data_homepage(
    State(app_state): State<Arc<AppContext>>,
    claims: Claims,
) -> ApiResult<HomepageResult> {
    let collection_games: mongodb::Collection<Game> = app_state.db.collection::<Game>("game");
    let collection_user: mongodb::Collection<User> = app_state.db.collection::<User>("user");

    // Convertir l'ID utilisateur
    let user_id = ObjectId::parse_str(&claims.sub)
        .map_err(|e| ApiError::BadRequest(format!("ID utilisateur invalide : {}", e)))?;

    // TEST Print the user
    let filter = doc! { "_id": user_id };
    let user = collection_user.find_one(filter).await?;
    println!("User: {:?}", user);

    let user_recent_games_ids = get_user_recent_games(&collection_user, user_id).await?;

    // List of games added recently
    let filter = doc! {};

    let options = FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(10)
        .build();

    let mut recent_added_ids = collection_games.find(filter).with_options(options).await?;

    let mut v_recent_added_ids = Vec::new();
    while let Some(doc) = recent_added_ids.try_next().await? {
        let game: Game = doc;
        println!("Game vvv: {:?}", game);
        if let Some(id) = game.id {
            v_recent_added_ids.push(id);
        }
    }

    let list_ids = vec![user_recent_games_ids.clone(), v_recent_added_ids.clone()].concat();

    println!("List of recent games IDs: {:?}", list_ids);

    // Get all games from the database using ids
    let filter = doc! { "_id": { "$in": list_ids } };
    let mut cursor = collection_games.find(filter).await?;

    // Init data using redis for simular response like popularity or recent added
    let mut data = HashMap::new(); // Placeholder for Redis data

    while let Some(doc) = cursor.try_next().await? {
        let game: Game = doc;
        println!("Game: {:?}", game);
        // id Option<ObjectId> is not serialized by default, so we need to serialize it manually
        let game_id = game.id.unwrap_or_default().to_hex();
        data.insert(game_id, game);
    }

    let recent_games_ids: Vec<String> =
        user_recent_games_ids.iter().map(|id| id.to_hex()).collect();

    let recent_added_ids: Vec<String> = v_recent_added_ids.iter().map(|id| id.to_hex()).collect();

    let result = HomepageResult {
        recent: recent_games_ids,
        //popular: popular_ids,
        recent_added: recent_added_ids,
        data,
    };

    Ok(axum::Json(result))
}

async fn get_game_by_id(
    State(app_state): State<Arc<AppContext>>,
    Path(id): Path<String>,
) -> ApiResult<Option<Game>> {
    let game_id = id.parse::<ObjectId>().unwrap();
    let filter = doc! { "_id": game_id };
    let game = app_state
        .db
        .collection::<Game>("game")
        .find_one(filter)
        .await
        .unwrap();
    Ok(axum::Json(game))
}
