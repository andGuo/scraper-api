use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    error::Result,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();
    let uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let client = connect_to_mongodb(&uri).await?;

    let db_names = client.list_database_names(None, None).await?;
    println!("Available databases: {:?}", db_names);

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/popular", get(handler_popular));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn connect_to_mongodb(uri: &str) -> Result<Client> {
    // Parse the client options
    let mut client_options = ClientOptions::parse(uri).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(client)
}

async fn handler_root() -> &'static str {
    "Hello, World!"
}

async fn handler_popular() -> impl IntoResponse {
    println!("popular");
}
