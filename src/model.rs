use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use bson::serde_helpers::chrono_datetime_as_bson_datetime; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Fruit {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub keywords: Vec<String>,
    pub text_content: String,
    pub out_links: Vec<String>,
    pub in_links: Vec<String>,
    pub page_rank: f64,
    pub score: Option<ScoreDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreDetails {
    pub value: f32,
    pub description: String,
    pub details: Vec<ScoreDetails>,
}