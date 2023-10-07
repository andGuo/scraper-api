use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{error::MyError, AppState};

pub async fn handler_root() -> &'static str {
    "Hello, World!"
}

pub async fn handler_popular(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.get_popular().await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn handler_pages(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.get_pages().await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn handler_page(
    Path(page_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match ObjectId::parse_str(page_id) {
        Ok(id) => match app_state.db.get_page(id).await.map_err(MyError::from) {
            Ok(res) => Ok(Json(res)),
            Err(e) => Err(e.into()),
        },
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid id"})),
        )),
    }
}
