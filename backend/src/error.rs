use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::fmt;
use thiserror::Error;

const INTERNAL: &str = "Internal error";

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct ErrorJson {
    key: String,
    errors: Vec<String>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct ErrorsJson(pub Vec<ErrorJson>);
impl std::error::Error for ErrorsJson {}

impl fmt::Display for ErrorsJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(&self.0).unwrap_or_else(|_| "[]".to_string());
        write!(f, "{}", json)
    }
}

impl ErrorJson {
    pub fn new(key: String, errors: Vec<String>) -> Self {
        Self { key, errors }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ApiError {
    #[error("{0}")]
    Generic(&'static str, StatusCode),

    #[error("{INTERNAL}")]
    Internal,

    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    BadRequest(String),

    // Json
    #[error("Json error: {0}")]
    JsonNotOk(String),

    // Display as a JSON object
    #[error(transparent)]
    JsonNotValid(#[from] ErrorsJson),

    // Serde
    #[error(transparent)]
    AuthError(#[from] AuthError),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum AuthError {
    // Auth
    #[error("Missing token")]
    AuthMissingToken,

    #[error("Missing credentials")]
    AuthMissingCredentials,

    #[error("Invalid credentials")]
    AuthInvalidCredentials,

    #[error("Invalid token")]
    AuthGeneric(),
}

#[derive(serde::Serialize)]
pub(crate) struct ErrorResponse {
    pub error: String,
    pub trace_id: String,
    pub timestamp: String,
}

/* impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Generic(value, StatusCode::INTERNAL_SERVER_ERROR)
    }
} */

impl From<mongodb::error::Error> for ApiError {
    fn from(err: mongodb::error::Error) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::InvalidArgument { message, .. } => ApiError::BadRequest(message),
            _ => ApiError::Internal,
        }
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

/* impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Internal => write!(f, "{}", INTERNAL),
            Error::NotFound(message) | Error::Generic(message, _) => write!(f, "{}", message),
            Error::AuthMissingToken => write!(f, "No token provided"),
            Error::AuthInvalidToken(kind) => match kind {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => write!(f, "Token expired"),
                jsonwebtoken::errors::ErrorKind::InvalidSignature => write!(f, "Invalid signature"),
                jsonwebtoken::errors::ErrorKind::InvalidToken => write!(f, "Invalid token"),
                _ => write!(f, "Invalid token"),
            },
            _ => write!(f, "{}", INTERNAL),
        }
    }
} */

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiError::Generic(_, status) => *status,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status().clone();

        /* let trace_id = middleware::TRACE_ID
        .try_with(|trace_id| trace_id.clone().to_string())
        .unwrap_or_else(|_| "unknown".to_string()); */

        let trace_id = "not-implemented".to_string(); // TODO: Implement trace_id

        let body = Json(ErrorResponse {
            error: self.to_string(),
            trace_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        (status, body).into_response()
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        /* match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                ApiError::new(Error::AuthInvalidToken)
            }

            _ => ApiError::new(Error::Internal),
        } */
        ApiError::Internal
    }
}
