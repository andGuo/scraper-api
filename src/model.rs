use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreDetails {
    pub value: f64,
    pub description: String,
    pub details: Vec<ScoreDetails>,
}

// TODO: Make this not use clone(), 
// using a Box<> or Arc<> in struct causes an error when unwrap() in db.
impl Fruit {
    pub fn boost_score(&mut self) {
        if let Some(score) = self.score.clone() {
            let new_value = self.page_rank * score.value;
            let new_description = format!("score boosted by page_rank");
            let new_details = vec![score];

            let new_score = ScoreDetails {
                value: new_value,
                description: new_description,
                details: new_details,
            };
            self.score = Some(new_score);
        }
    }
}
