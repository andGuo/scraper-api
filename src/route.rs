use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    handler::{handler_fruit, 
        handler_fruits, 
        handler_popular_fruit, 
        handler_root, 
        handler_personal, handler_personals, handler_popular_personal},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler_root))
        .route("/popular/fruits", get(handler_popular_fruit))
        .route("/popular/personal", get(handler_popular_personal))
        .route("/fruits", get(handler_fruits))
        .route("/fruits/:fruit_id", get(handler_fruit))
        .route("/personal", get(handler_personals))
        .route("/personal/:personal_id", get(handler_personal))
        .with_state(app_state)
}
