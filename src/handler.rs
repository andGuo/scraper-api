use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{error::MyError, schema::SearchParamOptions, AppState};

pub async fn handler_root() -> Redirect {
    Redirect::permanent(&std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set."))
}

pub async fn handler_popular(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.get_popular().await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn handler_fruits(
    params: Option<Query<SearchParamOptions>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(mut params) = params.unwrap_or_default();
    params.validate();

    println!("opts: {:?}", params);

    match app_state.db.get_fruits(params).await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn handler_fruit(
    Path(fruit_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match ObjectId::parse_str(fruit_id) {
        Ok(id) => match app_state.db.get_fruit(id).await.map_err(MyError::from) {
            Ok(res) => Ok(Json(res)),
            Err(e) => Err(e.into()),
        },
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid id"})),
        )),
    }
}
