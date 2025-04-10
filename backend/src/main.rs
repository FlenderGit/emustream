use std::{env, str::FromStr};

use api::auth::init_global_secret;
use chrono::DateTime;
use db::get_database;
use models::{Game, GameSave, Release, User};
use routes::create_router;
use tower_http::services::{ServeDir, ServeFile};

use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
};

mod api;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod traits;

pub struct AppContext {
    db: Database,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_global_secret();

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string());
    let is_prod = env == "production";

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set")
        .unwrap();
    println!("Database URL: {}", database_url);

    const DATABASE_NAME: &str = "emustream";
    let database = get_database(&database_url, DATABASE_NAME).await?;

    /* let database_url =
    "mongodb://user:password@localhost:27017/emustream?authSource=admin&directConnection=true"; */

    let collection: Collection<Game> = database.collection("game");
    let collection_user: Collection<User> = database.collection("user");
    println!("Connected to MongoDB");

    // Delete all documents in the collection
    /* let delete_result = collection.delete_many(doc! {}).await.unwrap();
    println!("Deleted {} documents", delete_result.deleted_count);

    let release_1 = Release {
        title: Some("Clannad - First press edition".to_string()),
        languages: vec!["JP".to_string(), "EN".to_string()],
        region: None,
        release_date: Some(DateTime::from_timestamp_nanos(1674_000_000)),
        path: "CLANNAD".to_string(),
        platforms: vec!["PS4".to_string()],
        created_at: chrono::Utc::now(),
    };

    let release_2 = Release {
        title: Some("Clannad - First press edition".to_string()),
        languages: vec!["JP".to_string()],
        platforms: vec!["WIN".to_string()],
        region: None,
        release_date: Some(DateTime::from_timestamp_nanos(1674_000_000)),
        path: "CLANNAD-fp".to_string(),
        created_at: chrono::Utc::now(),
    };

    let game = Game {
        id: None,
        title: "CLANNAD".to_string(),
        tags: vec!["VN".to_string(), "Drama".to_string()],
        developers: vec!["Key".to_string()],
        release_date: chrono::DateTime::from_timestamp_nanos(1674_000_000),
        releases: vec![release_1, release_2],
        metadata: Some(true),
        created_at: chrono::Utc::now(),
    };

    // Insert the game into the collection
    let insert_result = collection.insert_one(game).await.unwrap();
    println!("Inserted document with id: {}", insert_result.inserted_id);

    let release = Release {
        title: Some("Pokemon version rouge".to_string()),
        languages: vec!["EN".to_string()],
        region: None,
        release_date: Some(DateTime::from_timestamp_nanos(1674_000_000)),
        path: "pokemon-red".to_string(),
        platforms: vec!["Gameboy".to_string()],
        created_at: chrono::Utc::now(),
    };

    let game = Game {
        id: None,
        title: "Pokemon red".to_string(),
        tags: vec!["RPG".to_string(), "Pokemon".to_string()],
        developers: vec!["Game Freak".to_string()],
        release_date: chrono::DateTime::from_timestamp_nanos(1674_000_000),
        releases: vec![release],
        metadata: Some(true),
        created_at: chrono::Utc::now(),
    };

    // Insert the game into the collection
    let insert_result = collection.insert_one(game).await.unwrap();
    println!("Inserted document with id: {}", insert_result.inserted_id);

    let filter = doc! { "title": "Pokemon red" };
    let found_game: Option<Game> = collection.find_one(filter).await.unwrap();
    let oid = match found_game {
        Some(game) => {
            let json = serde_json::to_string_pretty(&game).unwrap();
            println!("Found game: {}", json);
            game.id.unwrap_or_default()
        }
        None => {
            println!("Game not found");
            std::process::exit(1);
        }
    };

    println!("Game ID: {}", oid.to_hex());

    // Insert two game data
    let game_data = GameSave {
        id: ObjectId::new(),
        game_id: oid,
        release_id: ObjectId::new(),
        save_data: "save-red-1".to_string(),
        time_spent: 100,
        timestamp: 1674_000_000,
        save_name: "Main".to_string(),
        created_at: chrono::Utc::now(),
    };

    let user = User {
        id: None,
        username: "Flender".to_string(),
        email: "flender@emustream.com".to_string(),
        password: "password".to_string(),
        saves: vec![game_data],
        created_at: chrono::Utc::now(),
    };

    // Delete all users in the collection
    let delete_result = collection_user.delete_many(doc! {}).await.unwrap();
    println!("Deleted {} documents", delete_result.deleted_count);

    // Print user pretty
    let user_json = serde_json::to_string_pretty(&user).unwrap();
    println!("User: {}", user_json);

    // Insert the user into the collection
    let insert_result = collection_user.insert_one(user).await.unwrap();
    println!("Inserted user with id: {}", insert_result.inserted_id); */

    // Find the game by title

    /* let games = collection.find(None).await.unwrap();
    while let Some(game) = games.next().await {
        match game {
            Ok(game) => {
                let json = serde_json::to_string_pretty(&game).unwrap();
                println!("Game: {}", json);
            },
            Err(e) => println!("Error: {}", e),
        }

    } */

    println!("Environment: {}", env);

    let app_state = AppContext { db: database };
    let app_state = std::sync::Arc::new(app_state);

    let mut app = create_router(app_state.clone());

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

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello() -> &'static str {
    "Hello, Jean!"
}
