use axum::http::StatusCode;
use futures::TryStreamExt;
use mongodb::{bson::{oid::ObjectId, Document}, Cursor};

use std::io::Error;

use crate::error::ApiError;



pub trait CursorExt {
    /// Convertit le curseur en un Vec<ObjectId>.
    async fn to_object_ids(self) -> Result<Vec<ObjectId>, ApiError>;
}


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
                    "Failed to get ObjectId from document",
                    StatusCode::BAD_REQUEST,
                ))?;
            object_ids.push(object_id);
        }
        Ok(object_ids)
    }
}


