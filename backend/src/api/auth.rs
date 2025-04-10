use std::str::FromStr;
use std::sync::{Arc, LazyLock, OnceLock};

use axum::extract::{Path, State};
use axum::RequestPartsExt;
use axum::{
    Extension, Json,
    extract::{FromRequestParts, Request},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::{ApiError, ApiResult};
use crate::models::{self, ValidatedJson};
use crate::AppContext;

// Déclaration de la variable globale
static SECRET: OnceLock<String> = OnceLock::new();

// Initialisation au démarrage de l'app
pub fn init_global_secret() {
    SECRET.get_or_init(|| {
        std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            eprintln!("[x] JWT_SECRET must be set in environment variables");
            std::process::exit(1);
        })
    });
}

// Accès sécurisé depuis n'importe quel module
pub fn get_secret() -> &'static str {
    SECRET.get().expect("Secret non initialisé")
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(length(min = 1, max = 64))]
    username: String,
    #[validate(length(min = 1, max = 64))]
    password: String,
}

#[derive(Clone, Serialize)]
pub struct User {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    pub sub: String, // Subject (id)
    exp: usize,
    iat: usize,
    iss: String,
    pub name: String,
}

pub async fn middleware_require_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let user = User {
        username: "admin".to_string(),
    };
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::AuthError(crate::error::AuthError::AuthMissingToken))?;

        // Decode the user data
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&["emustream_app"]);

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(get_secret().as_ref()),
            &validation,
        )
        .map_err(|e| {
            println!("Error decoding token: {:?}", e);
            ApiError::AuthError(crate::error::AuthError::AuthGeneric())
        })?;

        Ok(token_data.claims)
    }
}

pub async fn handler_me(Extension(user): Extension<User>) -> ApiResult<User> {
    Ok(Json(user))
}

pub async fn handler_me_claims(claims: Claims) -> ApiResult<Claims> {
    Ok(Json(claims))
}

pub async fn handler_login(
    State(app_state): State<Arc<AppContext>>,
    ValidatedJson(payload): ValidatedJson<AuthPayload>,
) -> ApiResult<AuthBody> {
    /* ensure!(!payload.username.is_empty(), Error::AuthMissingCredentials);
    ensure!(!payload.password.is_empty(), Error::AuthMissingCredentials); */

    let iat = Utc::now().timestamp() as usize;
    let exp = iat + 7200;

    let password_hashed = bcrypt::hash(payload.password.clone(), 4).unwrap();
    println!("Password hashed: {}", password_hashed);
    let user = app_state
        .db
        .collection::<models::User>("user")
        .find_one(doc! { "username": payload.username.clone() })
        .await
        .map_err(|_| ApiError::AuthError(crate::error::AuthError::AuthInvalidCredentials))?
        .ok_or(ApiError::AuthError(crate::error::AuthError::AuthInvalidCredentials))?;

    if !bcrypt::verify(payload.password, &user.password).unwrap() {
        return Err(ApiError::AuthError(crate::error::AuthError::AuthInvalidCredentials));
    }

    let claims = Claims {
        aud: "emustream_app".to_string(),
        sub: user.id.unwrap_or_default().to_hex(),
        iat,
        iss: "emustream_api".to_string(),
        name: payload.username.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    )?;

    Ok(Json(AuthBody { token }))
}
