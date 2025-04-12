use axum::{extract::Multipart, http::StatusCode};
use mongodb::{action::Find, bson::{oid::ObjectId, Document}, Cursor};

use std::{fmt::format, io::Error};

use crate::{api::Pagination, error::ApiError, models::{Game, Release}};



pub trait CursorExt {
    /// Convertit le curseur en un Vec<ObjectId>.
    async fn to_object_ids(self) -> Result<Vec<ObjectId>, ApiError>;
}

pub trait Paginate {
    /// Paginer les résultats d'une requête MongoDB.
    fn paginate(self, paginate: Pagination) -> Self;
}


use futures::TryStreamExt;
/// Implémentation de CursorExt pour un curseur sur Document.
impl CursorExt for Cursor<Document> {
    async fn to_object_ids(self) -> Result<Vec<ObjectId>, ApiError> {
        // Parcourt chaque document du curseur, récupère l'_id et le collecte dans un Vec.
        let mut object_ids = Vec::new();
        let mut cursor = self;
        while let Some(doc) = cursor.try_next().await? {
            let object_id = doc
                .get_object_id("_id")
                .map_err(|err| ApiError::Generic(
                    "Failed to get ObjectId from document".to_string(),
                    StatusCode::BAD_REQUEST,
                ))?;
            object_ids.push(object_id);
        }
        Ok(object_ids)
    }
}


pub trait MultipartExt {
    /// Extrait le jeu et la ROM du multipart/form-data.
    async fn extract_game_and_save(&mut self) -> Result<(Game, Vec<u8>), ApiError>;
    
}

impl MultipartExt for Multipart {
    async fn extract_game_and_save(&mut self) -> Result<(Game, Vec<u8>), ApiError> {
        let mut game = Game::default();
        let mut release = Release {
            id: Some(ObjectId::new()),
            ..Default::default()
        };
        let mut rom = Vec::new();

        while let Some(field) = self.next_field().await? {
            let field_name = field.name().map(|s| s.to_string());
            if let Some(field_name) = field_name {
                if field_name == "rom" {
                    rom = field.bytes().await?.to_vec();
                    continue;
                }
                let text = field.text().await?;
                match field_name.as_str() {
                    "title" => game.title = text,
                    "tags" => game.tags = text.split(',').map(|s| s.trim().to_string()).collect(),
                    _  => Err(ApiError::Generic(
                        "Invalid field name".to_string(),
                        StatusCode::BAD_REQUEST,
                    ))?,
                }
            }
        }

        release.path = format!("/data/games/{}/{}", game.title, release.id.unwrap_or_default());
        game.releases.push(release);

        Ok((game, rom))
    }

}
