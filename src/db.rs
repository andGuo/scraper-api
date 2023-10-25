use crate::{
    error::MyError,
    model::{Fruit, Xkcd},
    pipelines::{create_search_pipe, create_random_pipe},
    response::{FruitResponse, XkcdResponse},
    schema::SearchParamOptions,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document, oid::ObjectId, Document};
use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Collection,
};

#[derive(Clone, Debug)]
pub struct DB {
    pub xkcd_collection: Collection<Xkcd>,
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
        let personal_collection_name = std::env::var("MONGODB_PERSONAL_COLLECTION")
            .expect("MONGODB_PERSONAL_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let fruit_collection = database.collection::<Fruit>(collection_name.as_str());
        let xkcd_collection = database.collection::<Xkcd>(personal_collection_name.as_str());
        let collection = database.collection::<Document>(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            xkcd_collection,
            fruit_collection,
            collection,
        })
    }

    pub async fn get_fruits(&self, params: SearchParamOptions) -> Result<Vec<FruitResponse>> {
        if params.q.is_some() {
            let is_boost = params.boost.unwrap_or(false);

            let pipeline = create_search_pipe(&params.q.unwrap(), is_boost, params.limit.unwrap());

            let mut cursor = self
                .fruit_collection
                .aggregate(pipeline, None)
                .await
                .map_err(MyError::MongoQueryError)?;

            let mut json_res: Vec<FruitResponse> = Vec::new();

            while let Some(doc) = cursor.try_next().await? {
                let fruit: Fruit = from_document(doc).unwrap();
                json_res.push(fruit.into());
            }

            // If no results, return random results
            if json_res.is_empty() {
                let pipeline = create_random_pipe(params.limit.unwrap());

                let mut cursor = self
                    .fruit_collection
                    .aggregate(pipeline, None)
                    .await
                    .map_err(MyError::MongoQueryError)?;

                while let Some(doc) = cursor.try_next().await? {
                    let fruit: Fruit = from_document(doc).unwrap();
                    json_res.push(fruit.into());
                }
            }

            Ok(json_res)
        } else {
            let find_options = FindOptions::builder()
                .sort(doc! { "page_rank": -1 })
                .limit(params.limit.unwrap())
                .build();

            let mut cursor = self
                .fruit_collection
                .find(None, find_options)
                .await
                .map_err(MyError::MongoQueryError)?;

            let mut json_res: Vec<FruitResponse> = Vec::new();

            while let Some(fruit) = cursor.try_next().await? {
                json_res.push(fruit.into());
            }

            Ok(json_res)
        }
    }

    pub async fn get_fruit(&self, fruit_id: ObjectId) -> Result<Vec<FruitResponse>> {
        let fruit = match self
            .fruit_collection
            .find_one(doc! {"_id": fruit_id}, None)
            .await
        {
            Ok(Some(fruit)) => fruit,
            Ok(None) => return Err(MyError::NotFoundError(fruit_id.to_string())),
            Err(e) => return Err(MyError::MongoError(e)),
        };

        Ok(vec![fruit.into()])
    }

    pub async fn get_popular_fruit(&self) -> Result<Vec<FruitResponse>> {
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

        Ok(json_res)
    }

    pub async fn get_personal(&self, pid: ObjectId) -> Result<Vec<XkcdResponse>> {
        let comic = match self.xkcd_collection.find_one(doc! {"_id": pid}, None).await {
            Ok(Some(comic)) => comic,
            Ok(None) => return Err(MyError::NotFoundError(pid.to_string())),
            Err(e) => return Err(MyError::MongoError(e)),
        };

        Ok(vec![comic.into()])
    }

    pub async fn get_personals(&self, params: SearchParamOptions) -> Result<Vec<XkcdResponse>> {
        if params.q.is_some() {
            let is_boost = params.boost.unwrap_or(false);

            let pipeline = create_search_pipe(&params.q.unwrap(), is_boost, params.limit.unwrap());

            let mut cursor = self
                .xkcd_collection
                .aggregate(pipeline, None)
                .await
                .map_err(MyError::MongoQueryError)?;

            let mut json_res: Vec<XkcdResponse> = Vec::new();

            while let Some(doc) = cursor.try_next().await? {
                let comic: Xkcd = from_document(doc).unwrap();
                json_res.push(comic.into());
            }

            // If no results, return random results
            if json_res.is_empty() {
                let pipeline = create_random_pipe(params.limit.unwrap());

                let mut cursor = self
                    .xkcd_collection
                    .aggregate(pipeline, None)
                    .await
                    .map_err(MyError::MongoQueryError)?;

                while let Some(doc) = cursor.try_next().await? {
                    let comic: Xkcd = from_document(doc).unwrap();
                    json_res.push(comic.into());
                }
            }

            Ok(json_res)
        } else {
            let find_options = FindOptions::builder()
                .sort(doc! { "page_rank": -1 })
                .limit(params.limit.unwrap())
                .build();

            let mut cursor = self
                .xkcd_collection
                .find(None, find_options)
                .await
                .map_err(MyError::MongoQueryError)?;

            let mut json_res: Vec<XkcdResponse> = Vec::new();

            while let Some(comic) = cursor.try_next().await? {
                json_res.push(comic.into());
            }

            Ok(json_res)
        }
    }

    pub async fn get_popular_personal(&self) -> Result<Vec<XkcdResponse>> {
        let find_options = FindOptions::builder()
            .sort(doc! { "page_rank": -1 })
            .limit(10)
            .build();

        let mut cursor = self
            .xkcd_collection
            .find(None, find_options)
            .await
            .map_err(MyError::MongoQueryError)?;

        let mut json_res: Vec<XkcdResponse> = Vec::new();

        while let Some(pg) = cursor.try_next().await? {
            json_res.push(pg.into());
        }

        Ok(json_res)
    }
}
