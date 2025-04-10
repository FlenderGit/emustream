use axum::extract::{FromRequest, Query};
use serde::Deserialize;

use crate::error::ApiError;

pub mod games;
pub mod auth;


#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_limit")]	
    pub limit: i64,
    pub query: Option<String>,
    pub sort_dir: Option<SortDirection>,
}

fn default_limit() -> i64 {
    10
}
