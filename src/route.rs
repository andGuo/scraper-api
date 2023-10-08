use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    handler::{handler_fruit, handler_fruits, handler_popular, handler_root},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler_root))
        .route("/popular", get(handler_popular))
        .route("/fruits", get(handler_fruits))
        .route("/fruits/:fruit_id", get(handler_fruit))
        .with_state(app_state)
}
