use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    handler::{handler_fruit, handler_fruits, handler_popular_fruit, handler_root},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler_root))
        .route("/popular/fruits", get(handler_popular_fruit))
        .route("/fruits", get(handler_fruits))
        .route("/fruits/:fruit_id", get(handler_fruit))
        .with_state(app_state)
}
