use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{handler_page, handler_pages, handler_popular, handler_root},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler_root))
        .route("/popular", get(handler_popular))
        .route("/pages", get(handler_pages))
        .route("/pages/:id", get(handler_page))
        .with_state(app_state)
}