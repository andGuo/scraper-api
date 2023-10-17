mod db;
mod error;
mod handler;
mod middleware;
mod model;
mod pipelines;
mod response;
mod route;
mod schema;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware::from_fn,
};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use middleware::redirect_middleware;
use route::create_router;
use std::net::SocketAddr;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

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
            "info,tower_http=debug", //",mongodb::command=debug",
        )
    }

    if std::env::var_os("RUST_SERVER_ADDRESS").is_none() {
        std::env::set_var("RUST_SERVER_ADDRESS", "127.0.0.1")
    }

    if std::env::var_os("RUST_SERVER_PORT").is_none() {
        std::env::set_var("RUST_SERVER_PORT", "3000")
    }

    let addr_str = std::env::var("RUST_SERVER_ADDRESS").expect("RUST_SERVER_ADDRESS must be set.");
    let port_str = std::env::var("RUST_SERVER_PORT").expect("RUST_SERVER_PORT must be set.");
    let addr: Ipv4Addr = addr_str.parse().expect("Invalid IPv4 address");
    let port: u16 = port_str.parse().expect("Invalid port number");

    // Create a SocketAddrV4 using the parsed IPv4 address and port
    let socket_addr_v4 = SocketAddrV4::new(addr, port);

    let db = DB::init().await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    tracing_subscriber::fmt::init();
    let app = create_router(Arc::new(AppState { db }))
        .layer(from_fn(redirect_middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new());

    let addr = SocketAddr::from(socket_addr_v4);
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
