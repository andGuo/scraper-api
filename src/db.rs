use crate::error::MyError;
use crate::response::{GenericResponse, PagesResponse};
use crate::{error::MyError::*, model::Page};
use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub page_collection: Collection<Page>,
    pub collection: Collection<Document>,
}

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name = std::env::var("MONGO_DATABASE").expect("MONGO_DATABASE must be set.");
        let collection_name =
            std::env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let page_collection = database.collection(collection_name.as_str());
        let collection = database.collection::<Document>(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            page_collection,
            collection,
        })
    }

    pub async fn get_pages(&self) -> Result<PagesResponse> {
        let mut cursor = self
            .page_collection
            .find(None, None)
            .await
            .map_err(MyError::MongoQueryError)?;

        let mut json_res: Vec<Page> = Vec::new();

        while let Some(pg) = cursor.try_next().await? {
            json_res.push(pg);
        }

        Ok(PagesResponse {
            status: "success",
            data: json_res,
        })
    }
}
