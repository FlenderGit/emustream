use std::env;

use api::auth::init_global_secret;
use chrono::DateTime;
use db::get_database;
use models::{Game, Release};
use routes::create_router;
use tower_http::services::{ServeDir, ServeFile};

use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Database};

mod models;
mod db;
mod api;
mod error;
mod middleware;
mod routes;

pub struct AppContext {
    db: Database,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    init_global_secret();

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string());
    let is_prod = env == "production";

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set").unwrap();
    println!("Database URL: {}", database_url);

    const DATABASE_NAME: &str = "emustream";
    let database = get_database(&database_url, DATABASE_NAME).await?;

    /* let database_url =
        "mongodb://user:password@localhost:27017/emustream?authSource=admin&directConnection=true"; */

    
    let collection: Collection<Game> = database.collection("game");
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
    };

    let release_2 = Release {
        title: Some("Clannad - First press edition".to_string()),
        languages: vec!["JP".to_string()],
        platforms: vec!["WIN".to_string()],
        region: None,
        release_date: Some(DateTime::from_timestamp_nanos(1674_000_000)),
        path: "CLANNAD-fp".to_string(),
    };

    let game = Game {
        id: None,
        title: "CLANNAD".to_string(),
        tags: vec!["VN".to_string(), "Drama".to_string()],
        developers: vec!["Key".to_string()],
        release_date: chrono::DateTime::from_timestamp_nanos(1674_000_000),
        releases: vec![release_1, release_2],
        metadata: Some(true),
    }; 

    // Insert the game into the collection
    let insert_result = collection.insert_one(game).await.unwrap();
    println!("Inserted document with id: {}", insert_result.inserted_id);

    // Find the game by title
    let filter = doc! { "title": "CLANNAD" };
    let found_game: Option<Game> = collection.find_one(filter.clone()).await.unwrap();
    match found_game {
        Some(game) => {
            let json = serde_json::to_string_pretty(&game).unwrap();
            println!("Found game: {}", json);
        }
        None => println!("Game not found"),
    }*/

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
