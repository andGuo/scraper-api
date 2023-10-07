use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    error::MyError,
    model::Page,
    AppState,
};

pub async fn handler_root() -> &'static str {
    "Hello, World!"
}

pub async fn handler_popular() -> impl IntoResponse {
    println!("popular");
}

pub async fn handler_pages(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.get_pages().await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn handler_page(Path(id): Path<String>) -> impl IntoResponse {
    println!("page: {}", id);
}