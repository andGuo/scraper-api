use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<bson::DateTime>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<bson::DateTime>,
    pub title: String,
    pub keywords: Vec<String>,
    pub text_content: String,
    pub out_links: Vec<String>,
    pub in_links: Vec<String>,
    #[serde(rename = "pageRank", skip_serializing_if = "Option::is_none")]
    pub page_rank: Option<f64>,
}
