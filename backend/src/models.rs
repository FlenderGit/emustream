use std::ptr;

use axum::{
    Json, Router,
    extract::{
        Form, FromRequest, Request,
        rejection::{FormRejection, JsonRejection},
    },
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Game {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1, max = 64))]
    pub title: String,

    #[validate(length(min = 0, max = 10))]
    pub tags: Vec<String>,
    pub developers: Vec<String>,
    pub release_date: chrono::DateTime<chrono::Utc>,
    pub releases: Vec<Release>,
    pub metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    /* #[serde(rename = "_id")]
    id: Option<ObjectId>, */
    pub title: Option<String>,
    pub platforms: Vec<String>,
    pub languages: Vec<String>,
    pub region: Option<String>,
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
    pub path: String,
}

// Nouvelle fonction helper pour gérer Option<ObjectId>
fn serialize_option_object_id_as_hex_string<S>(
    value: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(oid) => serialize_object_id_as_hex_string(oid, serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ServerError::AxumJsonRejection(err))?;
        value.validate().map_err(|err| ServerError::ValidationError(err))?;
        Ok(ValidatedJson(value))
    }
}

/* #[derive(Debug)]
pub struct ValidatedForm<T>(pub T);
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state)
            .await
            .map_err(|err| ServerError::AxumJsonRejection(err))?;
        value.validate().map_err(|err| ServerError::ValidationError(err))?;
        Ok(ValidatedForm(value))
    }
} */

// Can be with error

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                println!("Validation error: {:?}", e);
                let message = format!("Input validation error: [{:?}]", e);
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(e) => {
                println!("Axum JSON rejection: {:?}", e);
                (StatusCode::BAD_REQUEST, e.to_string())
            }
        }
        .into_response()
    }
}
