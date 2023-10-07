mod db;
mod error;
mod handler;
mod model;
mod route;
mod response;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use route::create_router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "tower_http=debug,mongodb::command=debug",
        )
    }

    let db = DB::init().await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    tracing_subscriber::fmt::init();
    let app = create_router(Arc::new(AppState { db })).layer(cors).layer(TraceLayer::new_for_http());;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
