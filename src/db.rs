use crate::{
    error::MyError,
    model::Fruit,
    response::{FruitResponse, FruitsResponse},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Collection,
};

#[derive(Clone, Debug)]
pub struct DB {
    pub fruit_collection: Collection<Fruit>,
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

        let fruit_collection = database.collection(collection_name.as_str());
        let collection = database.collection::<Document>(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            fruit_collection,
            collection,
        })
    }

    pub async fn get_fruits(&self) -> Result<FruitsResponse> {
        let mut cursor = self
            .fruit_collection
            .find(None, None)
            .await
            .map_err(MyError::MongoQueryError)?;

        let mut json_res: Vec<FruitResponse> = Vec::new();

        while let Some(pg) = cursor.try_next().await? {
            json_res.push(pg.into());
        }

        Ok(FruitsResponse {
            status: "success",
            data: json_res,
        })
    }

    pub async fn get_fruit(&self, fruit_id: ObjectId) -> Result<FruitsResponse> {
        let fruit = match self
            .fruit_collection
            .find_one(doc! {"_id": fruit_id}, None)
            .await
        {
            Ok(Some(fruit)) => fruit,
            Ok(None) => return Err(MyError::NotFoundError(fruit_id.to_string())),
            Err(e) => return Err(MyError::MongoError(e)),
        };

        Ok(FruitsResponse {
            status: "success",
            data: vec![fruit.into()],
        })
    }

    pub async fn get_popular(&self) -> Result<FruitsResponse> {
        let find_options = FindOptions::builder()
            .sort(doc! { "page_rank": -1 })
            .limit(10)
            .build();

        let mut cursor = self
            .fruit_collection
            .find(None, find_options)
            .await
            .map_err(MyError::MongoQueryError)?;

        let mut json_res: Vec<FruitResponse> = Vec::new();

        while let Some(pg) = cursor.try_next().await? {
            json_res.push(pg.into());
        }

        Ok(FruitsResponse {
            status: "success",
            data: json_res,
        })
    }
}
