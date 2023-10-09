use crate::model::{Fruit, ScoreDetails};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct FruitResponse {
    pub id: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub text_content: String,
    pub out_links: Vec<String>,
    pub in_links: Vec<String>,
    pub pr: f64,
    pub score: Option<ScoreDetails>,
    pub name: &'static str,
}

impl From<Fruit> for FruitResponse {
    fn from(fruit: Fruit) -> Self {
        FruitResponse {
            id: fruit.id.to_hex(),
            url: fruit.url,
            created_at: fruit.created_at.to_rfc3339(),
            updated_at: fruit.updated_at.to_rfc3339(),
            title: fruit.title,
            keywords: fruit.keywords,
            text_content: fruit.text_content,
            out_links: fruit.out_links,
            in_links: fruit.in_links,
            pr: fruit.page_rank,
            score: fruit.score,
            name: "Andrew Guo",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FruitsResponse {
    pub status: &'static str,
    pub data: Vec<FruitResponse>,
}
